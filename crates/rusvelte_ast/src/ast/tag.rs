use oxc_ast::ast::Expression;
use oxc_span::Span;
use rusvelte_derive::{AstTree, OxcSpan};

#[derive(Debug, AstTree, OxcSpan)]
pub enum Tag<'a> {
    ExpressionTag(ExpressionTag<'a>),
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct ExpressionTag<'a> {
    pub span: Span,
    pub expression: Expression<'a>,
}
