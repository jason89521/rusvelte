use rusvelte_ast::{
    ast::*,
    ast_kind::{AstKind, SvelteAstKind},
    visit::{walk::*, JsVisit, Visit},
};

use super::scope_builder::ScopeBuilder;

impl<'a> Visit<'a> for ScopeBuilder<'a> {
    fn enter_svelte_node(&mut self, kind: SvelteAstKind<'a>) {
        self.create_ast_node(AstKind::Svelte(kind));
    }

    fn leave_svelte_node(&mut self, _kind: SvelteAstKind<'a>) {
        self.pop_ast_node();
    }

    fn visit_root(&mut self, root: &Root<'a>) {
        let kind = SvelteAstKind::Root(self.alloc(root));

        self.current_node_id = self.nodes.add_root_node(kind, self.current_scope_id);
        self.current_scope_id = self.scopes.add_scope(None, self.current_node_id, false);
        if let Some(module) = &root.module {
            self.visit_script(module);
        }
        if let Some(instance) = &root.instance {
            self.visit_script(instance);
        }

        self.visit_fragment(&root.fragment);

        self.leave_svelte_node(kind);
    }

    fn visit_bind_directive(&mut self, directive: &BindDirective<'a>) {
        walk_bind_directive(self, directive);
        self.extend_updates(&directive.expression);
    }

    fn visit_fragment(&mut self, fragment: &Fragment<'a>) {
        let kind = SvelteAstKind::Fragment(self.alloc(fragment));
        self.enter_svelte_node(kind);
        self.enter_scope_internal(&fragment.scope_id, fragment.metadata.borrow().transparent);
        walk_fragment_nodes(self, &fragment.nodes);
        self.leave_scope();
        self.leave_svelte_node(kind);
    }
}
