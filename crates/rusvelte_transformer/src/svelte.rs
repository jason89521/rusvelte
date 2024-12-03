use oxc_allocator::{CloneIn, Vec as OxcVec};
use rusvelte_analyzer::symbol::BindingKind;
use rusvelte_ast::{
    ast::{Fragment, FragmentNode},
    js_ast::Statement,
};
use rusvelte_utils::regex_pattern::REGEX_NOT_WHITESPACE;

use crate::Transformer;

impl<'a> Transformer<'a> {
    pub fn visit_fragment(&mut self, fragment: &mut Fragment<'a>) -> OxcVec<'a, Statement<'a>> {
        let mut body = OxcVec::new_in(self.allocator);
        let mut close = None;

        let trimmed = clean_nodes(&fragment.nodes);

        // if is_text_first {
        body.push(
            self.ast
                .statement_expression(self.ast.expression_call_with_atom("$.next", vec![])),
        );
        // }

        let is_single_element = trimmed.len() == 1 && trimmed[0].is_regular_element();
        if is_single_element {
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
                                let reference = self.symbols.get_reference(ident.reference_id());
                                let binding = self.symbols.get_binding(reference.symbol_id()?);
                                Some(binding.kind() == BindingKind::State)
                            })
                            .unwrap_or(false)
                        {
                            let update =
                                self.ast
                                    .statement_expression(self.ast.expression_call_with_atom(
                                        "$.set_text",
                                        vec![
                                            self.ast.expression_identifier_reference("text").into(),
                                            tag.expression.clone_in(self.allocator).into(),
                                        ],
                                    ));
                            self.state.update.push(update);
                        }
                    }
                }

                body.push(self.ast.statement_var(
                    id.clone_in(self.allocator),
                    self.ast.expression_call_with_atom("$.text", vec![]),
                ));
                close = Some(
                    self.ast
                        .statement_expression(self.ast.expression_call_with_atom(
                            "$.append",
                            vec![
                                self.ast.expression_identifier_reference("$$anchor").into(),
                                self.ast.expression_identifier_reference("text").into(),
                            ],
                        )),
                )
            }
        }

        if !self.state.update.is_empty() {
            let update = self.state.take_update();
            let stmt = self
                .ast
                .statement_expression(self.ast.expression_call_with_atom(
                    "$.template_effect",
                    vec![self.ast.expression_arrow(self.ast.vec([]), update).into()],
                ));
            body.push(stmt);
        }

        if let Some(stmt) = close {
            body.push(stmt);
        }

        body
    }
}

fn clean_nodes<'a>(nodes: &'a [FragmentNode<'a>]) -> &'a [FragmentNode<'a>] {
    let (start, end) = nodes
        .iter()
        .position(|node| {
            if let FragmentNode::Text(text) = node {
                REGEX_NOT_WHITESPACE.is_match(&text.data)
            } else {
                true
            }
        })
        .map_or((0, nodes.len()), |start| {
            let end = nodes[start..]
                .iter()
                .position(|node| {
                    if let FragmentNode::Text(text) = node {
                        !REGEX_NOT_WHITESPACE.is_match(&text.data)
                    } else {
                        false
                    }
                })
                .unwrap_or(nodes.len());
            (start, end)
        });
    &nodes[start..end]
}
