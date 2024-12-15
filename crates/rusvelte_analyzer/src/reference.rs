use oxc_index::IndexVec;
use oxc_span::CompactStr;
use oxc_syntax::{node::NodeId, reference::ReferenceId, scope::ScopeId, symbol::SymbolId};
use rusvelte_ast::{
    ast_kind::{AstKind, JsAstKind},
    js_ast::IdentifierReference,
};

use crate::node::AstNodes;

#[derive(Debug, Clone)]
pub struct Reference {
    name: CompactStr,
    /// The node that this reference is associated with.
    node_id: NodeId,
    /// The symbol that this reference refers to.
    symbol_id: Option<SymbolId>,
    scope_id: ScopeId,
}

impl Reference {
    pub fn new(
        name: CompactStr,
        node_id: NodeId,
        symbol_id: Option<SymbolId>,
        scope_id: ScopeId,
    ) -> Self {
        Self {
            name,
            node_id,
            symbol_id,
            scope_id,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn symbol_id(&self) -> Option<SymbolId> {
        self.symbol_id
    }

    pub fn set_symbol_id(&mut self, symbol_id: Option<SymbolId>) {
        self.symbol_id = symbol_id;
    }

    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn scope_id(&self) -> ScopeId {
        self.scope_id
    }

    pub fn is_unresolved(&self) -> bool {
        self.symbol_id.is_none()
    }

    pub fn identifier_reference<'a>(&self, nodes: &AstNodes<'a>) -> &IdentifierReference<'a> {
        let node = nodes.node(self.node_id);
        if let AstKind::Js(JsAstKind::IdentifierReference(ident)) = node.kind {
            ident
        } else {
            panic!("Expected IdentifierReference, found {:?}", node)
        }
    }
}

#[derive(Debug, Default)]
pub struct ReferenceTable {
    references: IndexVec<ReferenceId, Reference>,
}

impl ReferenceTable {
    pub fn create_reference(
        &mut self,
        node_id: NodeId,
        symbol_id: Option<SymbolId>,
        scope_id: ScopeId,
        name: &str,
    ) -> ReferenceId {
        self.references
            .push(Reference::new(name.into(), node_id, symbol_id, scope_id))
    }

    pub fn get_reference(&self, reference_id: ReferenceId) -> &Reference {
        &self.references[reference_id]
    }

    pub fn unresolved_references(&self) -> impl Iterator<Item = &Reference> {
        self.references
            .iter()
            .filter(|reference| reference.is_unresolved())
    }

    pub fn unresolved_references_mut(&mut self) -> impl Iterator<Item = &mut Reference> {
        self.references
            .iter_mut()
            .filter(|reference| reference.is_unresolved())
    }
}
