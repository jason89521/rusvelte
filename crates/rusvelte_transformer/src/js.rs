use std::cell::Cell;

use rusvelte_analyzer::{symbol::BindingKind, ScopeFlags, ScopeId};
use rusvelte_ast::visit::JsVisitMut;

use rusvelte_ast::js_ast::*;
use rusvelte_ast::walk::walk_mut::{walk_assignment_expression, walk_expression, walk_statements};

use crate::Transformer;

impl<'a> JsVisitMut<'a> for Transformer<'a> {
    fn enter_scope(&mut self, _flags: ScopeFlags, scope_id: &Cell<Option<ScopeId>>) {
        self.current_scope_id = scope_id.get().unwrap();
    }

    fn leave_scope(&mut self) {
        if let Some(scope_id) = self.scopes.get_parent_id(self.current_scope_id) {
            self.current_scope_id = scope_id;
        }
    }

    fn visit_variable_declarator(&mut self, decl: &mut VariableDeclarator<'a>) {
        if let Some(ident) = decl.id.get_binding_identifier() {
            let symbol_id = ident.symbol_id();
            let binding = self.symbols.get_binding(symbol_id);
            if binding.kind() == BindingKind::State {
                if let Expression::CallExpression(expr) = decl.init.as_mut().unwrap() {
                    expr.callee = self.ast.expression_identifier_reference("$.state")
                }
            }
        }
    }

    fn visit_statements(&mut self, stmts: &mut oxc_allocator::Vec<'a, Statement<'a>>) {
        walk_statements(self, stmts);
        stmts.retain(|stmt| !matches!(stmt, Statement::EmptyStatement(_)));
    }

    fn visit_expression(&mut self, expr: &mut Expression<'a>) {
        match expr {
            Expression::Identifier(ident) => {
                let (_, binding) = if let Some(v) = self.find_binding(&ident.name) {
                    v
                } else {
                    return;
                };
                // TODO: Svelte seems has different getter inside the `transform` object
                if binding.is_init_by_state() {
                    let call_expr = self.ast.call_with_atom(
                        "$.get",
                        self.ast
                            .vec([self.ast.expression_identifier_reference(&ident.name).into()]),
                    );
                    *expr = Expression::CallExpression(self.ast.alloc(call_expr))
                }
            }
            Expression::AssignmentExpression(assignment_expr) => {
                walk_assignment_expression(self, assignment_expr.as_mut());
                let (symbol_id, binding) = if let Some(v) = assignment_expr
                    .left
                    .get_identifier()
                    .and_then(|name| self.find_binding(name))
                {
                    v
                } else {
                    return;
                };
                if binding.is_init_by_state() {
                    let name = self.symbols.get_name(symbol_id);
                    let right = self.ast.move_expression(&mut assignment_expr.right);
                    let call_expr = self.ast.call_with_atom(
                        "$.set",
                        self.ast.vec([
                            self.ast.expression_identifier_reference(name).into(),
                            right.into(),
                        ]),
                    );
                    *expr = Expression::CallExpression(self.ast.alloc(call_expr));
                }
            }
            _ => walk_expression(self, expr),
        }
    }
}
