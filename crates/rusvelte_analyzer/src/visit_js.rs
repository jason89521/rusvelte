use oxc_syntax::scope::{ScopeFlags, ScopeId};
use rusvelte_ast::{ast_kind::*, js_ast::*, js_walk::walk::*, visit::JsVisit};

use crate::{binding::BindingKind, Analyzer};

impl<'a> JsVisit<'a> for Analyzer<'a> {
    fn enter_node(&mut self, _kind: JsAstKind<'a>) {
        self.move_to_next_node();
    }

    fn leave_node(&mut self, _kind: JsAstKind<'a>) {
        self.move_to_parent_node();
    }

    fn enter_scope(&mut self, _flags: ScopeFlags, scope_id: &std::cell::Cell<Option<ScopeId>>) {
        self.current_scope_id = scope_id.get().unwrap();
    }

    fn leave_scope(&mut self) {
        if let Some(scope_id) = self.scopes.get_parent_id(self.current_scope_id) {
            self.current_scope_id = scope_id;
        }
    }

    fn visit_identifier_reference(&mut self, it: &IdentifierReference<'a>) {
        let kind = JsAstKind::IdentifierReference(self.alloc(it));
        self.enter_node(kind);
        self.mark_subtree_dynamic();
        if let Some((symbol_id, binding)) = self.find_binding(self.current_scope_id, &it.name) {
            if let Some(expr_metadata) = self.state.expression_metadata() {
                expr_metadata.borrow_mut().dependencies.insert(symbol_id);
                expr_metadata.borrow_mut().has_state |= binding.kind() != BindingKind::Normal;
            }
        }
        self.leave_node(kind);
    }

    fn visit_call_expression(&mut self, it: &CallExpression<'a>) {
        walk_call_expression(self, it);
        if let Some(expr_metadata) = self.state.expression_metadata() {
            if !expr_metadata.borrow().dependencies.is_empty() {
                expr_metadata.borrow_mut().has_call = true;
            }
        }
    }
}
