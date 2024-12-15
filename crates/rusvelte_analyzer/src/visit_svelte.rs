use std::rc::Rc;

use rusvelte_ast::{
    ast::*,
    ast_kind::{AstKind, SvelteAstKind},
    js_ast::Expression,
    visit::{walk::*, JsVisit, Visit},
};

use crate::Analyzer;

impl<'a> Visit<'a> for Analyzer<'a> {
    fn enter_svelte_node(&mut self, kind: SvelteAstKind<'a>) {
        if !matches!(kind, SvelteAstKind::Root(_)) {
            self.move_to_next_node();
        }
    }

    fn leave_svelte_node(&mut self, _kind: SvelteAstKind<'a>) {
        self.move_to_parent_node();
    }

    fn visit_expression_tag(&mut self, it: &ExpressionTag<'a>) {
        let kind = SvelteAstKind::ExpressionTag(self.alloc(it));
        self.enter_svelte_node(kind);
        let old_expression_metadata = self
            .state
            .replace_expression_metadata(Some(Rc::clone(&it.expression_metadata)));
        self.mark_subtree_dynamic();
        self.visit_expression(&it.expression);
        self.state
            .replace_expression_metadata(old_expression_metadata);
        self.leave_svelte_node(kind);
    }

    fn visit_normal_attribute(&mut self, attr: &NormalAttribute<'a>) {
        let node_id = self.next_node_id;
        walk_normal_attribute(self, attr);
        if attr.is_event_attribute() {
            self.mark_subtree_dynamic();
        }

        if attr.value.is_true() {
            return;
        }

        for tag in attr.get_expression_tag_values() {
            if matches!(
                tag.expression,
                Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_)
            ) {
                continue;
            }

            attr.expression_metadata.borrow_mut().has_state |=
                tag.expression_metadata.borrow().has_state;
            attr.expression_metadata.borrow_mut().has_call |=
                tag.expression_metadata.borrow().has_call;
        }

        if attr.is_event_attribute() {
            if self
                .nodes
                .parent_node(node_id)
                .map(|node| matches!(node.kind, AstKind::Svelte(SvelteAstKind::RegularElement(_))))
                .is_some()
            {
                self.use_event_attribute = true;
            } else {
                // TODO
            }
        }
    }
}
