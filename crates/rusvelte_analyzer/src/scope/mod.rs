use std::collections::{HashMap, HashSet};

use oxc_index::IndexVec;
use oxc_span::CompactStr;
use oxc_syntax::keyword::is_reserved_keyword;
use oxc_syntax::reference::ReferenceId;
use oxc_syntax::symbol::SymbolId;
use oxc_syntax::{node::NodeId, scope::ScopeId};
use rusvelte_utils::regex_pattern::REGEX_NOT_VALID_IDENTIFIER_CHAR;

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
    /// The scope depth allows us to determine if a state variable is referenced in its own scope,
    /// which is usually an error. Block statements do not increase this value
    function_depth: u8,
}

impl Scope {
    pub fn new(
        parent_id: Option<ScopeId>,
        node_id: NodeId,
        porous: bool,
        function_depth: u8,
    ) -> Self {
        Self {
            parent_id,
            node_id,
            porous,
            function_depth,
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
    pub const ROOT_SCOPE_ID: ScopeId = ScopeId::new(0);

    pub fn root_scope_id(&self) -> ScopeId {
        Self::ROOT_SCOPE_ID
    }

    pub fn add_scope(
        &mut self,
        parent_id: Option<ScopeId>,
        node_id: NodeId,
        porous: bool,
    ) -> ScopeId {
        let function_depth = parent_id
            .map(|parent_id| self.scopes[parent_id].function_depth + if porous { 0 } else { 1 })
            .unwrap_or(0);
        self.scopes
            .push(Scope::new(parent_id, node_id, porous, function_depth));
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

    /// Return unique name based on the preferred name
    pub fn unique(&mut self, preferred_name: &str) -> CompactStr {
        let preferred_name = REGEX_NOT_VALID_IDENTIFIER_CHAR.replace_all(preferred_name, "_");
        let mut final_name = preferred_name.to_string();
        let mut n = 1;
        while self.conflicts.contains(final_name.as_str()) {
            final_name = format!("{}_{}", preferred_name, n);
            n += 1;
        }

        self.conflicts.insert(final_name.clone().into());

        final_name.into()
    }

    pub fn generate(&mut self, preferred_name: &str, scope_id: ScopeId) -> CompactStr {
        let scope = self.scopes[scope_id];
        if scope.porous && scope.parent_id.is_some() {
            return self.generate(preferred_name, scope.parent_id.unwrap());
        }

        let preferred_name = REGEX_NOT_VALID_IDENTIFIER_CHAR.replace_all(preferred_name, "_");
        let mut name = preferred_name.to_string();
        let mut n = 1;

        while self.references[scope_id].contains_key(name.as_str())
            || self.bindings[scope_id].contains_key(name.as_str())
            || self.conflicts.contains(name.as_str())
            || is_reserved_keyword(&name)
        {
            name = format!("{}_{}", preferred_name, n);
            n += 1;
        }

        let name = CompactStr::from(name);
        self.references[scope_id].insert(name.clone(), Vec::new());
        self.conflicts.insert(name.clone());

        name
    }
}
