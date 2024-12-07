use oxc_span::{CompactStr, Span};
use oxc_syntax::reference::ReferenceId;
use oxc_syntax::symbol::SymbolId;
use oxc_syntax::{node::NodeId, scope::ScopeId};
use rusvelte_ast::ast::Root;
use rusvelte_ast::ast_kind::AstKind;
use rusvelte_ast::visit::Visit;

use crate::node::AstNodes;
use crate::reference::UnresolvedJsReference;
use crate::symbol::{BindingKind, DeclarationKind, Symbols};

use super::Scopes;

pub struct ScopeBuilder<'a> {
    pub scopes: Scopes,
    pub nodes: AstNodes<'a>,
    pub symbols: Symbols,
    pub current_node_id: NodeId,
    pub current_scope_id: ScopeId,

    pub unresolved_js_references: Vec<UnresolvedJsReference<'a>>,
}

impl Default for ScopeBuilder<'_> {
    fn default() -> Self {
        let scopes = Scopes::default();
        let current_scope_id = scopes.root_scope_id();

        Self {
            scopes,
            nodes: AstNodes::new(),
            symbols: Symbols::default(),
            current_node_id: NodeId::new(0),
            current_scope_id,
            unresolved_js_references: vec![],
        }
    }
}

pub struct ScopeBuilderReturn<'a> {
    pub scopes: Scopes,
    pub nodes: AstNodes<'a>,
    pub symbols: Symbols,
}

impl<'a> ScopeBuilder<'a> {
    pub fn build(mut self, root: &Root<'a>) -> ScopeBuilderReturn<'a> {
        self.visit_root(root);

        for reference in std::mem::take(&mut self.unresolved_js_references) {
            let id = self.reference(reference.name, reference.node_id, reference.scope_id);
            reference.ident.set_reference_id(id);
        }

        ScopeBuilderReturn {
            scopes: self.scopes,
            nodes: self.nodes,
            symbols: self.symbols,
        }
    }

    pub fn create_ast_node(&mut self, kind: AstKind<'a>) {
        self.current_node_id =
            self.nodes
                .add_node(kind, self.current_scope_id, self.current_node_id);
    }

    pub fn pop_ast_node(&mut self) {
        if let Some(parent_id) = self.nodes.parent_id(self.current_node_id) {
            self.current_node_id = parent_id;
        }
    }

    pub fn declare(
        &mut self,
        span: Span,
        name: &str,
        kind: BindingKind,
        declaration_kind: DeclarationKind,
    ) -> SymbolId {
        let scope_id = self.current_scope_id;
        if let Some(_parent_scope_id) = self.scopes.get_parent_id(scope_id) {
            // TODO: check var & porous
            if declaration_kind == DeclarationKind::Import {
                self.declare_in_scope(span, name, kind, declaration_kind, scope_id);
            }
        }

        if self.scopes.bindings[scope_id].contains_key(name) {
            // TODO: error
        }

        self.declare_in_scope(span, name, kind, declaration_kind, scope_id)
    }

    fn declare_in_scope(
        &mut self,
        span: Span,
        name: &str,
        kind: BindingKind,
        declaration_kind: DeclarationKind,
        scope_id: ScopeId,
    ) -> SymbolId {
        // TODO: validate identifier name
        let symbol_id = self.symbols.create_symbol(
            span,
            name,
            scope_id,
            self.current_node_id,
            kind,
            declaration_kind,
        );
        self.scopes.add_binding(scope_id, name, symbol_id);
        self.scopes.conflicts.insert(name.into());
        symbol_id
    }

    pub fn reference<T: Into<CompactStr>>(
        &mut self,
        name: T,
        node_id: NodeId,
        scope_id: ScopeId,
    ) -> ReferenceId {
        let name: CompactStr = name.into();
        for scope_id in self.scopes.ancestors(scope_id) {
            if let Some(symbol_id) = self.scopes.bindings[scope_id].get(&name) {
                return self.symbols.create_reference(Some(*symbol_id), node_id);
            }
        }

        self.symbols.create_reference(None, node_id)
    }
}
