use rusvelte_ast::{
    ast::*,
    ast_kind::{AstKind, SvelteAstKind},
    visit::{JsVisit, Visit},
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
        self.current_scope_id = self.scopes.add_scope(None, self.current_node_id);
        if let Some(module) = &root.module {
            self.visit_program(&module.content);
        }
        if let Some(instance) = &root.instance {
            self.visit_program(&instance.content);
        }

        self.visit_fragment(&root.fragment);

        self.leave_svelte_node(kind);
    }
}
