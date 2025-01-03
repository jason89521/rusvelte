use clean_nodes::CleanNodesReturn;
use rusvelte_analyzer::binding::BindingKind;
use rusvelte_ast::{ast::*, js_ast::Statement, visit_mut::VisitMut};

use oxc_allocator::{CloneIn, Vec as OxcVec};

use crate::Transformer;

mod clean_nodes;

impl<'a> VisitMut<'a> for Transformer<'a> {
    fn vec<T, const N: usize>(&self, array: [T; N]) -> OxcVec<'a, T> {
        self.ast.vec(array)
    }

    fn visit_fragment(&mut self, fragment: &mut Fragment<'a>) -> OxcVec<'a, Statement<'a>> {
        let mut body = OxcVec::new_in(self.allocator);
        let mut close = None;

        let CleanNodesReturn {
            hoisted: _,
            mut trimmed,
            is_standalone: _,
            is_text_first,
        } = self.clean_nodes(fragment);

        if is_text_first {
            body.push(
                self.ast.statement_expression(
                    self.ast
                        .expression_call_with_atom("$.next", self.ast.vec([])),
                ),
            );
        }

        let template_name = self.scopes.unique("root");

        let is_single_element = trimmed.len() == 1 && trimmed[0].is_regular_element();
        if is_single_element {
            let element = trimmed[0].as_regular_element_mut().unwrap();
            let id = self.scopes.generate(element.name, self.current_scope_id);
            body.push(
                self.ast.statement_var(
                    self.ast.binding_pattern_identifier(id.as_str()),
                    self.ast
                        .expression_call_with_atom(&template_name, self.vec([])),
                ),
            )
        } else if !trimmed.is_empty() {
            let use_space_template = trimmed.iter().any(FragmentNode::is_expression_tag)
                && trimmed
                    .iter()
                    .all(|node| node.is_expression_tag() || node.is_text());
            if use_space_template {
                // TODO: should generate by scopes
                let id = self.ast.binding_pattern_identifier("text");
                // TODO: should process all children nodes before push to body
                // the following is a simplified process
                // maybe we can define `visit_fragment_nodes` to process all children nodes
                for node in trimmed.iter() {
                    if let Some(tag) = node.as_expression_tag() {
                        if tag
                            .expression
                            .get_identifier_reference()
                            .and_then(|ident| {
                                let reference = self.references.get_reference(ident.reference_id());
                                let binding = self.symbols.get_binding(reference.symbol_id()?);
                                Some(binding.kind() == BindingKind::State)
                            })
                            .unwrap_or(false)
                        {
                            let update =
                                self.ast
                                    .statement_expression(self.ast.expression_call_with_atom(
                                        "$.set_text",
                                        self.ast.vec([
                                            self.ast.expression_identifier_reference("text").into(),
                                            tag.expression.clone_in(self.allocator).into(),
                                        ]),
                                    ));
                            self.state.update.push(update);
                        }
                    }
                }

                body.push(
                    self.ast.statement_var(
                        id.clone_in(self.allocator),
                        self.ast
                            .expression_call_with_atom("$.text", self.ast.vec([])),
                    ),
                );
                close = Some(
                    self.ast
                        .statement_expression(self.ast.expression_call_with_atom(
                            "$.append",
                            self.ast.vec([
                                self.ast.expression_identifier_reference("$$anchor").into(),
                                self.ast.expression_identifier_reference("text").into(),
                            ]),
                        )),
                )
            }
        }

        if !self.state.update.is_empty() {
            let update = self.state.take_update();
            let stmt = self.ast.statement_expression(
                self.ast.expression_call_with_atom(
                    "$.template_effect",
                    self.ast
                        .vec([self.ast.expression_arrow(self.ast.vec([]), update).into()]),
                ),
            );
            body.push(stmt);
        }

        if let Some(stmt) = close {
            body.push(stmt);
        }

        body
    }
}
