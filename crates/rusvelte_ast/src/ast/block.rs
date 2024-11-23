use oxc_ast::ast::{BindingPattern, Expression};
use oxc_span::Span;
use rusvelte_derive::{AstTree, OxcSpan};

use super::Fragment;

#[derive(Debug, AstTree, OxcSpan)]
pub enum Block<'a> {
    IfBlock(IfBlock<'a>),
    EachBlock(EachBlock<'a>),
    AwaitBlock(AwaitBlock<'a>),
    KeyBlock(KeyBlock<'a>),
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

#[derive(Debug, AstTree, OxcSpan)]
pub struct AwaitBlock<'a> {
    pub span: Span,
    pub expression: Expression<'a>,
    /// The resolved value inside the `then` block
    pub value: Option<BindingPattern<'a>>,
    /// The rejection reason inside the `catch` block
    pub error: Option<BindingPattern<'a>>,
    pub pending: Option<Fragment<'a>>,
    pub then: Option<Fragment<'a>>,
    pub catch: Option<Fragment<'a>>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct KeyBlock<'a> {
    pub span: Span,
    pub expression: Expression<'a>,
    pub fragment: Fragment<'a>,
}
