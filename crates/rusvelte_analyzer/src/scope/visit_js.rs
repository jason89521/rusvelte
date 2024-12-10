use oxc_syntax::scope::{ScopeFlags, ScopeId};
use rusvelte_ast::{
    ast_kind::{AstKind, JsAstKind, SvelteAstKind},
    js_ast::*,
    visit::JsVisit,
    walk::walk::*,
};

use crate::binding::BindingKind;

use super::{binder::Binder, scope_builder::ScopeBuilder};

impl<'a> JsVisit<'a> for ScopeBuilder<'a> {
    fn enter_node(&mut self, kind: JsAstKind<'a>) {
        self.create_ast_node(AstKind::Js(kind));

        match kind {
            JsAstKind::IdentifierReference(ident) => {
                let reference_id = self.reference(
                    ident.name.as_str(),
                    self.current_node_id,
                    self.current_scope_id,
                );
                ident.set_reference_id(reference_id);
            }
            JsAstKind::VariableDeclarator(decl) => {
                let is_in_const_tag = self
                    .nodes
                    .parent_id(self.current_node_id) // VariableDeclaration
                    .and_then(|node_id| {
                        let grand_parent = self.nodes.parent_node(node_id)?;
                        let is_const_tag = matches!(
                            grand_parent.kind,
                            AstKind::Svelte(SvelteAstKind::ConstTag(_))
                        );
                        Some(is_const_tag)
                    })
                    .unwrap_or(false);
                let binding_kind = decl.init.as_ref().map_or(BindingKind::Normal, |expr| {
                    if let Expression::CallExpression(expr) = expr {
                        if expr.callee_name().map_or(false, |name| name == "$state") {
                            BindingKind::State
                        } else {
                            BindingKind::Normal
                        }
                    } else if is_in_const_tag {
                        BindingKind::Template
                    } else {
                        BindingKind::Normal
                    }
                });
                decl.bind(self, binding_kind);
            }
            JsAstKind::ImportDeclaration(decl) => {
                decl.bind(self, BindingKind::Normal);
            }
            JsAstKind::FormalParameter(param) => {
                param.bind(self, BindingKind::Normal);
            }
            JsAstKind::BindingRestElement(rest) => {
                rest.bind(self, BindingKind::Normal);
            }
            JsAstKind::Class(it) => {
                it.bind(self, BindingKind::Normal);
            }
            _ => {}
        }
    }

    fn enter_scope(
        &mut self,
        _flags: oxc_syntax::scope::ScopeFlags,
        scope_id: &std::cell::Cell<Option<ScopeId>>,
    ) {
        self.enter_scope_internal(scope_id, false);
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

    fn visit_assignment_expression(&mut self, expr: &AssignmentExpression<'a>) {
        walk_assignment_expression(self, expr);
        self.extend_updates(&expr.left);
    }

    fn visit_update_expression(&mut self, expr: &UpdateExpression<'a>) {
        walk_update_expression(self, expr);
        self.extend_updates(&expr.argument);
    }

    fn visit_function(&mut self, func: &Function<'a>, flags: ScopeFlags) {
        let kind = JsAstKind::Function(self.alloc(func));
        self.enter_node(kind);

        let flags = {
            let mut flags = flags;
            if func.is_strict() {
                flags |= ScopeFlags::StrictMode;
            }
            flags
        };
        let binding_kind = BindingKind::Normal;
        if func.is_expression() {
            self.enter_scope(flags, &func.scope_id);
            func.bind(self, binding_kind);
        } else {
            func.bind(self, binding_kind);
            self.enter_scope(flags, &func.scope_id);
        }

        if let Some(id) = &func.id {
            self.visit_binding_identifier(id);
        }
        if let Some(type_parameters) = &func.type_parameters {
            self.visit_ts_type_parameter_declaration(type_parameters);
        }
        if let Some(this_param) = &func.this_param {
            self.visit_ts_this_parameter(this_param);
        }
        self.visit_formal_parameters(&func.params);
        if let Some(return_type) = &func.return_type {
            self.visit_ts_type_annotation(return_type);
        }
        if let Some(body) = &func.body {
            self.visit_function_body(body);
        }
        self.leave_scope();
        self.leave_node(kind);
    }

    fn visit_for_statement(&mut self, it: &ForStatement<'a>) {
        let kind = JsAstKind::ForStatement(self.alloc(it));
        self.enter_node(kind);
        self.enter_scope_internal(&it.scope_id, true);
        if let Some(init) = &it.init {
            self.visit_for_statement_init(init);
        }
        if let Some(test) = &it.test {
            self.visit_expression(test);
        }
        if let Some(update) = &it.update {
            self.visit_expression(update);
        }
        self.visit_statement(&it.body);
        self.leave_scope();
        self.leave_node(kind);
    }

    fn visit_for_in_statement(&mut self, it: &ForInStatement<'a>) {
        let kind = JsAstKind::ForInStatement(self.alloc(it));
        self.enter_node(kind);
        self.enter_scope_internal(&it.scope_id, true);
        self.visit_for_statement_left(&it.left);
        self.visit_expression(&it.right);
        self.visit_statement(&it.body);
        self.leave_scope();
        self.leave_node(kind);
    }

    fn visit_for_of_statement(&mut self, it: &ForOfStatement<'a>) {
        let kind = JsAstKind::ForOfStatement(self.alloc(it));
        self.enter_node(kind);
        self.enter_scope_internal(&it.scope_id, true);
        self.visit_for_statement_left(&it.left);
        self.visit_expression(&it.right);
        self.visit_statement(&it.body);
        self.leave_scope();
        self.leave_node(kind);
    }

    fn visit_switch_statement(&mut self, it: &SwitchStatement<'a>) {
        let kind = JsAstKind::SwitchStatement(self.alloc(it));
        self.enter_node(kind);
        self.enter_scope_internal(&it.scope_id, true);
        self.visit_expression(&it.discriminant);
        self.visit_switch_cases(&it.cases);
        self.leave_scope();
        self.leave_node(kind);
    }

    fn visit_block_statement(&mut self, it: &BlockStatement<'a>) {
        let kind = JsAstKind::BlockStatement(self.alloc(it));
        self.enter_node(kind);
        self.enter_scope_internal(&it.scope_id, true);
        self.visit_statements(&it.body);
        self.leave_scope();
        self.leave_node(kind);
    }

    fn visit_catch_clause(&mut self, it: &CatchClause<'a>) {
        let kind = JsAstKind::CatchClause(self.alloc(it));
        self.enter_node(kind);
        self.enter_scope_internal(&it.scope_id, true);
        if let Some(param) = &it.param {
            param.bind(self, BindingKind::Normal);
        }
        self.visit_block_statement(&it.body);
        self.leave_scope();
        self.leave_node(kind);
    }
}
