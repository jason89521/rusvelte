use oxc_span::Span;
use rusvelte_derive::{AstTree, OxcSpan};

use super::{attribute::Attribute, Fragment};

#[derive(Debug, AstTree, OxcSpan)]
pub enum Element<'a> {
    RegularElement(RegularElement<'a>),
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct RegularElement<'a> {
    pub span: Span,
    pub name: &'a str,
    pub attributes: Vec<Attribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct Comment<'a> {
    pub span: Span,
    pub data: &'a str,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct SvelteHead {
    pub span: Span,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct SvelteOptions {
    pub span: Span,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct SvelteWindow {
    pub span: Span,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct SvelteDocument {
    pub span: Span,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct SvelteBody {
    pub span: Span,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct SvelteElement {
    pub span: Span,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct SvelteComponent {
    pub span: Span,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct SvelteSelf {
    pub span: Span,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct SvelteFragment {
    pub span: Span,
}
