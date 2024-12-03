use oxc_ast::ast::{Expression, ObjectExpression};
use oxc_span::Span;
use rusvelte_derive::{AstTree, OxcSpan};

use super::{style_sheet::StyleSheet, Attribute, Fragment, Script};

#[derive(Debug, AstTree, OxcSpan)]
pub struct Root<'a> {
    pub css: Option<StyleSheet<'a>>,
    pub span: Span,
    pub fragment: Fragment<'a>,
    pub module: Option<Script<'a>>,
    pub instance: Option<Script<'a>>,
    pub options: Option<SvelteOptions<'a>>,
}

#[derive(Debug, AstTree, OxcSpan, Default)]
pub struct SvelteOptions<'a> {
    pub span: Span,
    pub runes: Option<bool>,
    pub immutable: Option<bool>,
    pub accessors: Option<bool>,
    pub attributes: Vec<Attribute<'a>>,
    pub custom_element: Option<CustomElement<'a>>,
    pub namespace: Option<&'a str>,
    pub css: Option<&'a str>,
    pub preserve_whitespace: Option<bool>,
}

impl SvelteOptions<'_> {
    pub fn new(span: Span) -> Self {
        Self {
            span,
            ..Default::default()
        }
    }
}

#[derive(Debug, AstTree, Default)]
pub struct CustomElement<'a> {
    pub tag: Option<&'a str>,
    pub shadow: Option<&'a str>,
    // TODO: should define the shape of `props`
    pub props: Option<ObjectExpression<'a>>,
    pub extend: Option<Expression<'a>>,
}
