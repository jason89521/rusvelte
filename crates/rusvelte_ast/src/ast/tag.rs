use derive_macro::{AstTree, OxcSpan};
use oxc_ast::ast::Expression;
use oxc_span::Span;

#[derive(Debug, AstTree, OxcSpan)]
pub enum Tag<'a> {
    ExpressionTag(ExpressionTag<'a>),
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct ExpressionTag<'a> {
    pub span: Span,
    pub expression: Expression<'a>,
}
