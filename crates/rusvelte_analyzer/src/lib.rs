use binding::{Binding, BindingTable};
use node::AstNodes;
use oxc_index::Idx;
use reference::ReferenceTable;
use rusvelte_ast::{
    ast::Root,
    ast_kind::{AstKind, SvelteAstKind},
    visit::Visit,
};
use scope::{scope_builder::ScopeBuilderReturn, ScopeTable};

pub use oxc_syntax::{
    node::NodeId,
    scope::{ScopeFlags, ScopeId},
    symbol::{SymbolFlags, SymbolId},
};
use state::State;

pub mod binding;
pub mod node;
pub mod reference;
pub mod scope;

mod state;
mod visit_js;
mod visit_svelte;

pub struct Analysis<'a> {
    pub scopes: ScopeTable,
    pub symbols: BindingTable,
    pub references: ReferenceTable,
    pub nodes: AstNodes<'a>,
    pub used_event_attribute: bool,
}

#[derive(Debug)]
pub struct CompileOptions {
    pub component_name: String,
}

impl CompileOptions {
    pub fn new(component_name: String) -> Self {
        Self { component_name }
    }
}

#[derive(Debug)]
pub struct Analyzer<'a> {
    current_scope_id: ScopeId,
    current_node_id: NodeId,
    next_node_id: NodeId,
    #[allow(dead_code)]
    compile_options: CompileOptions,
    state: State,
    scopes: ScopeTable,
    nodes: AstNodes<'a>,
    bindings: BindingTable,
    #[allow(dead_code)]
    references: ReferenceTable,
    use_event_attribute: bool,
}

impl<'a> Analyzer<'a> {
    pub fn new(compile_options: CompileOptions, root: &Root<'a>) -> Self {
        let ScopeBuilderReturn {
            scopes,
            nodes,
            binding_table,
            reference_table,
        } = scope::scope_builder::ScopeBuilder::default().build(root);
        Self {
            compile_options,
            state: State::default(),
            scopes,
            nodes,
            bindings: binding_table,
            references: reference_table,
            current_scope_id: ScopeTable::ROOT_SCOPE_ID,
            current_node_id: NodeId::new(0),
            next_node_id: NodeId::new(1),
            use_event_attribute: false,
        }
    }

    pub fn analyze(mut self, root: &Root<'a>) -> Analysis<'a> {
        let ScopeBuilderReturn {
            scopes,
            nodes,
            binding_table: symbols,
            reference_table: references,
        } = scope::scope_builder::ScopeBuilder::default().build(root);
        self.visit_root(root);
        Analysis {
            scopes,
            nodes,
            symbols,
            references,
            used_event_attribute: self.use_event_attribute,
        }
    }

    fn move_to_next_node(&mut self) {
        self.current_node_id = self.next_node_id;
        self.next_node_id = NodeId::from_usize(self.next_node_id.index() + 1);
    }

    fn move_to_parent_node(&mut self) {
        if let Some(parent_id) = self.nodes.parent_id(self.current_node_id) {
            self.current_node_id = parent_id;
        }
    }

    fn mark_subtree_dynamic(&self) {
        for node_id in self.nodes.ancestors(self.current_node_id) {
            let AstKind::Svelte(SvelteAstKind::Fragment(fragment)) = self.nodes.node(node_id).kind
            else {
                continue;
            };
            if fragment.metadata.borrow().dynamic {
                return;
            }
            fragment.metadata.borrow_mut().dynamic = true;
        }
    }

    fn find_binding(&self, scope_id: ScopeId, name: &str) -> Option<(SymbolId, &Binding)> {
        self.scopes
            .find_symbol_id(scope_id, name)
            .map(|symbol_id| (symbol_id, self.bindings.get_binding(symbol_id)))
    }
}
