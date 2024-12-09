use std::collections::{HashMap, HashSet};

use oxc_index::IndexVec;
use oxc_span::CompactStr;
use oxc_syntax::reference::ReferenceId;
use oxc_syntax::symbol::SymbolId;
use oxc_syntax::{node::NodeId, scope::ScopeId};

pub type Bindings = HashMap<CompactStr, SymbolId>;
pub type References = HashMap<CompactStr, Vec<ReferenceId>>;

mod binder;
pub mod scope_builder;
mod visit_js;
mod visit_svelte;

#[derive(Debug, Clone, Copy)]
pub struct Scope {
    parent_id: Option<ScopeId>,
    node_id: NodeId,
    porous: bool,
}

impl Scope {
    pub fn new(parent_id: Option<ScopeId>, node_id: NodeId, porous: bool) -> Self {
        Self {
            parent_id,
            node_id,
            porous,
        }
    }

    pub fn parent_id(&self) -> Option<ScopeId> {
        self.parent_id
    }

    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn porous(&self) -> bool {
        self.porous
    }
}

#[derive(Debug, Default)]
pub struct ScopeTable {
    scopes: IndexVec<ScopeId, Scope>,
    bindings: IndexVec<ScopeId, Bindings>,
    references: IndexVec<ScopeId, References>,
    conflicts: HashSet<CompactStr>,
}

impl ScopeTable {
    const ROOT_SCOPE_ID: ScopeId = ScopeId::new(0);

    pub fn root_scope_id(&self) -> ScopeId {
        Self::ROOT_SCOPE_ID
    }

    pub fn add_scope(
        &mut self,
        parent_id: Option<ScopeId>,
        node_id: NodeId,
        porous: bool,
    ) -> ScopeId {
        self.scopes.push(Scope::new(parent_id, node_id, porous));
        self.bindings.push(HashMap::new());
        self.references.push(HashMap::new())
    }

    pub fn get_parent_id(&self, scope_id: ScopeId) -> Option<ScopeId> {
        self.scopes[scope_id].parent_id
    }

    pub fn add_binding<T: Into<CompactStr>>(
        &mut self,
        scope_id: ScopeId,
        name: T,
        symbol_id: SymbolId,
    ) {
        self.bindings[scope_id].insert(name.into(), symbol_id);
    }

    pub fn add_reference<T: Into<CompactStr>>(
        &mut self,
        scope_id: ScopeId,
        name: T,
        reference_id: ReferenceId,
    ) {
        self.references[scope_id]
            .entry(name.into())
            .and_modify(|v| v.push(reference_id))
            .or_default();
    }

    pub fn ancestors(&self, scope_id: ScopeId) -> impl Iterator<Item = ScopeId> + '_ {
        std::iter::successors(Some(scope_id), |&scope_id| self.scopes[scope_id].parent_id)
    }

    pub fn find_symbol_id(&self, scope_id: ScopeId, name: &str) -> Option<SymbolId> {
        for scope_id in self.ancestors(scope_id) {
            if let Some(&symbol_id) = self.bindings[scope_id].get(name) {
                return Some(symbol_id);
            }
        }
        None
    }
}
