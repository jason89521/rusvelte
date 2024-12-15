use oxc_index::IndexVec;
use oxc_syntax::{node::NodeId, scope::ScopeId};
use rusvelte_ast::ast_kind::{AstKind, SvelteAstKind};

#[derive(Debug, Clone, Copy)]
pub struct AstNode<'a> {
    pub id: NodeId,
    pub kind: AstKind<'a>,
    pub scope_id: ScopeId,
}

impl<'a> AstNode<'a> {
    pub fn new(id: NodeId, kind: AstKind<'a>, scope_id: ScopeId) -> Self {
        Self { id, kind, scope_id }
    }
}

#[derive(Debug, Default)]
pub struct AstNodes<'a> {
    nodes: IndexVec<NodeId, AstNode<'a>>,
    parent_ids: IndexVec<NodeId, Option<NodeId>>,
}

impl<'a> AstNodes<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_root_node(&mut self, kind: SvelteAstKind<'a>, scope_id: ScopeId) -> NodeId {
        let kind = AstKind::Svelte(kind);
        let node_id = self.parent_ids.push(None);
        let node = AstNode::new(node_id, kind, scope_id);
        self.nodes.push(node);
        node_id
    }

    pub fn add_node(
        &mut self,
        kind: AstKind<'a>,
        scope_id: ScopeId,
        parent_node_id: NodeId,
    ) -> NodeId {
        let node_id = self.parent_ids.push(Some(parent_node_id));
        let node = AstNode::new(node_id, kind, scope_id);
        self.nodes.push(node);
        node_id
    }

    pub fn node(&self, node_id: NodeId) -> &AstNode<'a> {
        &self.nodes[node_id]
    }

    pub fn parent_id(&self, node_id: NodeId) -> Option<NodeId> {
        self.parent_ids[node_id]
    }

    pub fn parent_node(&self, node_id: NodeId) -> Option<&AstNode<'a>> {
        self.parent_id(node_id).map(|node_id| self.node(node_id))
    }
    pub fn ancestors(&self, node_id: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        std::iter::successors(Some(node_id), |&node_id| self.parent_ids[node_id])
    }
}
