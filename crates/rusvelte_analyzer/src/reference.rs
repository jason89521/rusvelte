use oxc_span::Atom;
use oxc_syntax::{node::NodeId, scope::ScopeId, symbol::SymbolId};
use rusvelte_ast::js_ast::IdentifierReference;

#[derive(Debug, Clone, Copy)]
pub struct Reference {
    /// The node that this reference is associated with.
    node_id: NodeId,
    /// The symbol that this reference refers to.
    symbol_id: Option<SymbolId>,
}

impl Reference {
    pub fn new(node_id: NodeId, symbol_id: Option<SymbolId>) -> Self {
        Self { node_id, symbol_id }
    }

    pub fn symbol_id(&self) -> Option<SymbolId> {
        self.symbol_id
    }

    pub fn node_id(&self) -> NodeId {
        self.node_id
    }
}

#[derive(Debug)]
pub struct UnresolvedJsReference<'a> {
    pub node_id: NodeId,
    pub name: Atom<'a>,
    pub scope_id: ScopeId,
    pub ident: &'a IdentifierReference<'a>,
}

impl<'a> UnresolvedJsReference<'a> {
    pub fn new(
        node_id: NodeId,
        scope_id: ScopeId,
        name: Atom<'a>,
        ident: &'a IdentifierReference<'a>,
    ) -> Self {
        Self {
            node_id,
            scope_id,
            name,
            ident,
        }
    }
}
