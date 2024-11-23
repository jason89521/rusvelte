use oxc_ast::ast::{
    CallExpression, ChainExpression, Expression, IdentifierReference, VariableDeclaration,
};
use oxc_span::Span;
use rusvelte_derive::{AstTree, OxcSpan};

#[derive(Debug, AstTree, OxcSpan)]
pub enum Tag<'a> {
    ExpressionTag(ExpressionTag<'a>),
    HtmlTag(HtmlTag<'a>),
    DebugTag(DebugTag<'a>),
    ConstTag(ConstTag<'a>),
    RenderTag(RenderTag<'a>),
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct ExpressionTag<'a> {
    pub span: Span,
    pub expression: Expression<'a>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct HtmlTag<'a> {
    pub span: Span,
    pub expression: Expression<'a>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct DebugTag<'a> {
    pub span: Span,
    pub identifiers: Vec<IdentifierReference<'a>>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct ConstTag<'a> {
    pub span: Span,
    pub declaration: VariableDeclaration<'a>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct RenderTag<'a> {
    pub span: Span,
    pub expression: RenderTagExpression<'a>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub enum RenderTagExpression<'a> {
    CallExpression(CallExpression<'a>),
    // Only accept CallExpression in it.
    ChainExpression(ChainExpression<'a>),
}
