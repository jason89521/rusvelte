use oxc_ast::ast::{BindingPattern, Expression};
use oxc_span::Span;
use rusvelte_derive::{AstTree, OxcSpan};

use super::Fragment;

#[derive(Debug, AstTree, OxcSpan)]
pub enum Block<'a> {
    IfBlock(IfBlock<'a>),
    EachBlock(EachBlock<'a>),
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct IfBlock<'a> {
    pub span: Span,
    pub elseif: bool,
    pub test: Expression<'a>,
    pub consequent: Fragment<'a>,
    pub alternate: Option<Fragment<'a>>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct EachBlock<'a> {
    pub span: Span,
    pub expression: Expression<'a>,
    pub context: BindingPattern<'a>,
    pub body: Fragment<'a>,
    pub fallback: Option<Fragment<'a>>,
    pub index: Option<&'a str>,
    pub key: Option<Expression<'a>>,
}
