use oxc_ast::ast::Expression;
use oxc_span::Span;
use rusvelte_derive::{AstTree, OxcSpan};

use super::{attribute::Attribute, Fragment};
use oxc_allocator::Vec;

#[derive(Debug, AstTree, OxcSpan)]
pub enum Element<'a> {
    RegularElement(RegularElement<'a>),
    SvelteComponent(SvelteComponent<'a>),
    SvelteElement(SvelteElement<'a>),
    SvelteBody(SvelteBody<'a>),
    SvelteWindow(SvelteWindow<'a>),
    SvelteDocument(SvelteDocument<'a>),
    SvelteHead(SvelteHead<'a>),
    SvelteFragment(SvelteFragment<'a>),
    SvelteSelf(SvelteSelf<'a>),
    TitleElement(TitleElement<'a>),
    SlotElement(SlotElement<'a>),
    Component(Component<'a>),
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct RegularElement<'a> {
    pub span: Span,
    pub name: &'a str,
    pub attributes: Vec<'a, Attribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug, AstTree, OxcSpan, Clone, Copy)]
pub struct Comment<'a> {
    pub span: Span,
    pub data: &'a str,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct SvelteHead<'a> {
    pub span: Span,
    pub name: &'a str,
    pub attributes: Vec<'a, Attribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct SvelteWindow<'a> {
    pub span: Span,
    pub name: &'a str,
    pub attributes: Vec<'a, Attribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct SvelteDocument<'a> {
    pub span: Span,
    pub name: &'a str,
    pub attributes: Vec<'a, Attribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct SvelteBody<'a> {
    pub span: Span,
    pub name: &'a str,
    pub attributes: Vec<'a, Attribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct SvelteElement<'a> {
    pub span: Span,
    pub name: &'a str,
    pub attributes: Vec<'a, Attribute<'a>>,
    pub fragment: Fragment<'a>,
    pub tag: Expression<'a>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct SvelteComponent<'a> {
    pub span: Span,
    pub name: &'a str,
    pub attributes: Vec<'a, Attribute<'a>>,
    pub fragment: Fragment<'a>,
    pub expression: Expression<'a>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct SvelteSelf<'a> {
    pub span: Span,
    pub name: &'a str,
    pub attributes: Vec<'a, Attribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct SvelteFragment<'a> {
    pub span: Span,
    pub name: &'a str,
    pub attributes: Vec<'a, Attribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct TitleElement<'a> {
    pub span: Span,
    pub name: &'a str,
    pub attributes: Vec<'a, Attribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct SlotElement<'a> {
    pub span: Span,
    pub name: &'a str,
    pub attributes: Vec<'a, Attribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct Component<'a> {
    pub span: Span,
    pub name: &'a str,
    pub attributes: Vec<'a, Attribute<'a>>,
    pub fragment: Fragment<'a>,
    #[ast_ignore]
    pub dynamic: bool,
}
