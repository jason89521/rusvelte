use node::AstNodes;
use rusvelte_ast::ast::Root;
use scope::{scope_builder::ScopeBuilderReturn, Scopes};
use symbol::Symbols;

pub use oxc_syntax::{
    node::NodeId,
    scope::{ScopeFlags, ScopeId},
    symbol::{SymbolFlags, SymbolId},
};

pub mod node;
mod reference;
pub mod scope;
pub mod symbol;

#[derive(Debug, Default)]
pub struct Analyzer {}

impl Analyzer {
    pub fn analyze<'a>(self, root: &Root<'a>) -> (Scopes, AstNodes<'a>, Symbols) {
        let ScopeBuilderReturn {
            scopes,
            nodes,
            symbols,
        } = scope::scope_builder::ScopeBuilder::default().build(root);
        (scopes, nodes, symbols)
    }
}
