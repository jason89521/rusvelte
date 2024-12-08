use oxc_span::{CompactStr, Span};
use oxc_syntax::reference::ReferenceId;
use oxc_syntax::symbol::SymbolId;
use oxc_syntax::{node::NodeId, scope::ScopeId};
use rusvelte_ast::ast::Root;
use rusvelte_ast::ast_kind::AstKind;
use rusvelte_ast::visit::Visit;

use crate::binding::{BindingKind, BindingTable, DeclarationKind};
use crate::node::AstNodes;
use crate::reference::ReferenceTable;

use super::Scopes;

pub struct ScopeBuilder<'a> {
    pub scopes: Scopes,
    pub nodes: AstNodes<'a>,
    pub binding_table: BindingTable,
    pub reference_table: ReferenceTable,
    pub current_node_id: NodeId,
    pub current_scope_id: ScopeId,
}

impl Default for ScopeBuilder<'_> {
    fn default() -> Self {
        let scopes = Scopes::default();
        let current_scope_id = scopes.root_scope_id();

        Self {
            scopes,
            nodes: AstNodes::new(),
            binding_table: BindingTable::default(),
            reference_table: ReferenceTable::default(),
            current_node_id: NodeId::new(0),
            current_scope_id,
        }
    }
}

pub struct ScopeBuilderReturn<'a> {
    pub scopes: Scopes,
    pub nodes: AstNodes<'a>,
    pub binding_table: BindingTable,
    pub reference_table: ReferenceTable,
}

impl<'a> ScopeBuilder<'a> {
    pub fn build(mut self, root: &Root<'a>) -> ScopeBuilderReturn<'a> {
        self.visit_root(root);

        for reference in self.reference_table.unresolved_references_mut() {
            let symbol_id = self
                .scopes
                .find_symbol_id(reference.scope_id(), reference.name());
            reference.set_symbol_id(symbol_id);
        }

        ScopeBuilderReturn {
            scopes: self.scopes,
            nodes: self.nodes,
            binding_table: self.binding_table,
            reference_table: self.reference_table,
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
        let symbol_id = self.binding_table.create_symbol(
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
        let (reference_id, scope_id) = self.create_reference(node_id, scope_id, &name);
        self.scopes.add_reference(scope_id, name, reference_id);
        reference_id
    }

    fn create_reference(
        &mut self,
        node_id: NodeId,
        scope_id: ScopeId,
        name: &str,
    ) -> (ReferenceId, ScopeId) {
        for scope_id in self.scopes.ancestors(scope_id) {
            if let Some(symbol_id) = self.scopes.bindings[scope_id].get(name) {
                let reference_id = self.reference_table.create_reference(
                    node_id,
                    Some(*symbol_id),
                    scope_id,
                    name,
                );
                return (reference_id, scope_id);
            }
        }

        let reference_id =
            self.reference_table
                .create_reference(self.current_node_id, None, scope_id, name);
        (reference_id, scope_id)
    }
}
