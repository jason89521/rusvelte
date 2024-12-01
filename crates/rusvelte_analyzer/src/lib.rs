use binder::Binder;
use node::AstNodes;
use oxc_span::{CompactStr, Span};
use oxc_syntax::{node::NodeId, scope::ScopeId, symbol::SymbolId};
use rusvelte_ast::{
    ast::Root,
    ast_kind::{AstKind, JsAstKind, SvelteAstKind},
    js_ast::{Expression, Program},
    visit::{JsVisit, Visit},
};
use scope::Scopes;
use symbol::{Binding, BindingFlags, BindingKind, DeclarationKind, Symbols};

mod binder;
pub mod node;
pub mod scope;
pub mod symbol;

#[derive(Debug)]
pub struct Analyzer<'a> {
    pub scopes: Scopes,
    pub nodes: AstNodes<'a>,
    pub symbols: Symbols,
    current_node_id: NodeId,
    current_scope_id: ScopeId,
}

impl<'a> Default for Analyzer<'a> {
    fn default() -> Self {
        let scopes = Scopes::default();
        let current_scope_id = scopes.root_scope_id();

        Self {
            scopes,
            nodes: AstNodes::new(),
            symbols: Symbols::default(),
            current_node_id: NodeId::new(0),
            current_scope_id,
        }
    }
}

impl<'a> Analyzer<'a> {
    pub fn analyze(mut self, root: &Root<'a>) -> (Scopes, AstNodes<'a>, Symbols) {
        self.visit_root(root);
        (self.scopes, self.nodes, self.symbols)
    }

    fn create_ast_node(&mut self, kind: AstKind<'a>) {
        self.current_node_id =
            self.nodes
                .add_node(kind, self.current_scope_id, self.current_node_id);
    }

    fn pop_ast_node(&mut self) {
        if let Some(parent_id) = self.nodes.parent_id(self.current_node_id) {
            self.current_node_id = parent_id
        }
    }

    fn declare_symbol(
        &mut self,
        span: Span,
        name: &str,
        kind: BindingKind,
        declaration_kind: DeclarationKind,
    ) -> SymbolId {
        self.declare_symbol_on_scope(span, name, self.current_scope_id, kind, declaration_kind)
    }

    fn declare_symbol_on_scope(
        &mut self,
        span: Span,
        name: &str,
        scope_id: ScopeId,
        kind: BindingKind,
        declaration_kind: DeclarationKind,
    ) -> SymbolId {
        // TODO: check redeclaration
        let name = CompactStr::new(name);
        let symbol_id = self.symbols.create_symbol(
            span,
            name.clone(),
            scope_id,
            self.current_node_id,
            kind,
            declaration_kind,
        );
        self.scopes.add_binding(scope_id, name, symbol_id);
        symbol_id
    }

    pub fn find_binding_mut(&mut self, name: &str) -> Option<(SymbolId, &mut Binding)> {
        self.scopes
            .find_symbol_id(self.current_scope_id, name)
            .map(|symbol_id| (symbol_id, self.symbols.get_binding_mut(symbol_id)))
    }
}

impl<'a> JsVisit<'a> for Analyzer<'a> {
    fn enter_node(&mut self, kind: JsAstKind<'a>) {
        self.create_ast_node(AstKind::Js(kind));
        match kind {
            JsAstKind::VariableDeclarator(decl) => {
                let binding_kind =
                    decl.init
                        .as_ref()
                        .map_or(BindingKind::Normal, |expr| match expr {
                            Expression::CallExpression(call_expr) => {
                                if call_expr
                                    .callee_name()
                                    .map_or(false, |name| name == "$state")
                                {
                                    BindingKind::State
                                } else {
                                    BindingKind::Normal
                                }
                            }
                            _ => BindingKind::Normal,
                        });
                decl.bind(self, binding_kind);
            }
            JsAstKind::Function(func) => {
                if func.is_declaration() {
                    func.bind(self, BindingKind::Normal);
                }
            }
            JsAstKind::AssignmentExpression(expr) => {
                if let Some((_, binding)) = expr
                    .left
                    .get_identifier()
                    .and_then(|name| self.find_binding_mut(name))
                {
                    binding.binding_flags |= BindingFlags::reassigned();
                }
            }
            JsAstKind::IdentifierReference(ident) => {
                if let Some((symbol_id, binding)) = self.find_binding_mut(&ident.name) {
                    binding.binding_flags |= BindingFlags::read();
                    let reference_id = self
                        .symbols
                        .create_reference(Some(symbol_id), self.current_node_id);
                    ident.set_reference_id(reference_id);
                } else {
                    let reference_id = self.symbols.create_reference(None, self.current_node_id);
                    ident.set_reference_id(reference_id);
                }
            }
            _ => (),
        }
    }

    fn leave_node(&mut self, _kind: JsAstKind<'a>) {
        self.pop_ast_node();
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
}

impl<'a> Visit<'a> for Analyzer<'a> {
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
