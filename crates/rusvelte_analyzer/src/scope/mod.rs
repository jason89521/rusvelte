use std::collections::{HashMap, HashSet};

use oxc_index::IndexVec;
use oxc_span::CompactStr;
use oxc_syntax::symbol::SymbolId;
use oxc_syntax::{node::NodeId, scope::ScopeId};

pub type Bindings = HashMap<CompactStr, SymbolId>;

mod binder;
pub mod scope_builder;
mod visit_js;
mod visit_svelte;

#[derive(Debug, Default)]
pub struct Scopes {
    parent_ids: IndexVec<ScopeId, Option<ScopeId>>,
    child_ids: IndexVec<ScopeId, Vec<ScopeId>>,
    node_ids: IndexVec<ScopeId, NodeId>,
    bindings: IndexVec<ScopeId, Bindings>,
    conflicts: HashSet<CompactStr>,
}

impl Scopes {
    const ROOT_SCOPE_ID: ScopeId = ScopeId::new(0);

    pub fn root_scope_id(&self) -> ScopeId {
        Self::ROOT_SCOPE_ID
    }

    pub fn add_scope(&mut self, parent_id: Option<ScopeId>, node_id: NodeId) -> ScopeId {
        let scope_id = self.parent_ids.push(parent_id);
        self.child_ids.push(vec![]);
        self.node_ids.push(node_id);
        self.bindings.push(HashMap::new());
        scope_id
    }

    pub fn get_parent_id(&self, scope_id: ScopeId) -> Option<ScopeId> {
        self.parent_ids[scope_id]
    }

    pub fn add_binding<T: Into<CompactStr>>(
        &mut self,
        scope_id: ScopeId,
        name: T,
        symbol_id: SymbolId,
    ) {
        self.bindings[scope_id].insert(name.into(), symbol_id);
    }

    pub fn ancestors(&self, scope_id: ScopeId) -> impl Iterator<Item = ScopeId> + '_ {
        std::iter::successors(Some(scope_id), |&scope_id| self.parent_ids[scope_id])
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
