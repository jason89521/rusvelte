use binding::BindingTable;
use node::AstNodes;
use reference::ReferenceTable;
use rusvelte_ast::ast::Root;
use scope::{scope_builder::ScopeBuilderReturn, ScopeTable};

pub use oxc_syntax::{
    node::NodeId,
    scope::{ScopeFlags, ScopeId},
    symbol::{SymbolFlags, SymbolId},
};

pub mod binding;
pub mod node;
pub mod reference;
pub mod scope;

#[derive(Debug, Default)]
pub struct Analyzer {}

impl Analyzer {
    pub fn analyze<'a>(
        self,
        root: &Root<'a>,
    ) -> (ScopeTable, AstNodes<'a>, BindingTable, ReferenceTable) {
        let ScopeBuilderReturn {
            scopes,
            nodes,
            binding_table: symbols,
            reference_table: references,
        } = scope::scope_builder::ScopeBuilder::default().build(root);
        (scopes, nodes, symbols, references)
    }
}
