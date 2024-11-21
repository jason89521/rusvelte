use oxc_ast::ast::Expression;
use oxc_span::Span;
use rusvelte_derive::{AstTree, OxcSpan};

use super::Fragment;

#[derive(Debug, AstTree, OxcSpan)]
pub enum Block<'a> {
    IfBlock(IfBlock<'a>),
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct IfBlock<'a> {
    pub span: Span,
    pub elseif: bool,
    pub test: Expression<'a>,
    pub consequent: Fragment<'a>,
    pub alternate: Option<Fragment<'a>>,
}
