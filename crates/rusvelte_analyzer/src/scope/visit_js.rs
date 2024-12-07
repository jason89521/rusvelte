use oxc_syntax::scope::ScopeId;
use rusvelte_ast::{
    ast_kind::{AstKind, JsAstKind, SvelteAstKind},
    js_ast::*,
    visit::JsVisit,
};

use crate::{reference::UnresolvedJsReference, symbol::BindingKind};

use super::{binder::Binder, scope_builder::ScopeBuilder};

impl<'a> JsVisit<'a> for ScopeBuilder<'a> {
    fn enter_node(&mut self, kind: JsAstKind<'a>) {
        self.create_ast_node(AstKind::Js(kind));

        #[allow(clippy::single_match)]
        match kind {
            JsAstKind::IdentifierReference(ident) => {
                self.unresolved_js_references
                    .push(UnresolvedJsReference::new(
                        self.current_node_id,
                        self.current_scope_id,
                        ident.name.clone(),
                        ident,
                    ));
            }

            _ => {}
        }
    }

    fn enter_scope(
        &mut self,
        _flags: oxc_syntax::scope::ScopeFlags,
        scope_id: &std::cell::Cell<Option<ScopeId>>,
    ) {
        let parent_scope_id = self.current_scope_id;
        self.current_scope_id = self
            .scopes
            .add_scope(Some(parent_scope_id), self.current_node_id);
        scope_id.set(Some(self.current_scope_id));
    }

    fn leave_scope(&mut self) {
        if let Some(parent_id) = self.scopes.get_parent_id(self.current_scope_id) {
            self.current_scope_id = parent_id
        }
    }

    fn visit_program(&mut self, program: &Program<'a>) {
        // Don't call enter_scope because we consider it is in the root scope
        program.set_scope_id(self.current_scope_id);
        if let Some(hashbang) = &program.hashbang {
            self.visit_hashbang(hashbang);
        }

        for directive in &program.directives {
            self.visit_directive(directive);
        }

        self.visit_statements(&program.body);
    }

    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration<'a>) {
        let is_parent_const_tag = matches!(
            self.nodes.get_node(self.current_node_id).kind,
            AstKind::Svelte(SvelteAstKind::ConstTag(_))
        );
        let kind = JsAstKind::VariableDeclaration(self.alloc(decl));
        self.enter_node(kind);
        for decl in decl.declarations.iter() {
            let ast_kind = JsAstKind::VariableDeclarator(self.alloc(decl));
            self.enter_node(ast_kind);

            let binding_kind = decl.init.as_ref().map_or(BindingKind::Normal, |expr| {
                if let Expression::CallExpression(expr) = expr {
                    if expr.callee_name().map_or(false, |name| name == "$state") {
                        BindingKind::State
                    } else {
                        BindingKind::Normal
                    }
                } else if is_parent_const_tag {
                    BindingKind::Template
                } else {
                    BindingKind::Normal
                }
            });
            decl.bind(self, binding_kind);

            self.visit_binding_pattern(&decl.id);
            if let Some(init) = &decl.init {
                self.visit_expression(init);
            }
            self.leave_node(ast_kind);
        }
        self.leave_node(kind);
    }
}
