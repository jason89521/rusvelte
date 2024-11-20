use oxc_span::Span;
use rusvelte_derive::{AstTree, OxcSpan};

use super::{attribute::Attribute, Comment};

#[derive(Debug, AstTree, OxcSpan)]
pub struct StyleSheet<'a> {
    pub span: Span,
    pub attributes: Vec<Attribute<'a>>,
    pub children: Vec<StyleSheetChild<'a>>,
    pub content: StyleSheetContent<'a>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct StyleSheetContent<'a> {
    pub span: Span,
    pub styles: &'a str,
    pub comment: Option<Comment<'a>>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub enum StyleSheetChild<'a> {
    AtRule(AtRule<'a>),
    Rule(Rule<'a>),
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct AtRule<'a> {
    pub span: Span,
    pub name: &'a str,
    pub prelude: &'a str,
    pub block: Option<CSSBlock<'a>>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct RelativeSelector<'a> {
    pub span: Span,
    pub combinator: Option<Combinator<'a>>,
    pub selectors: Vec<SimpleSelector<'a>>,
}

impl<'a> RelativeSelector<'a> {
    pub fn new(combinator: Option<Combinator<'a>>, start: u32) -> Self {
        Self {
            span: Span::empty(start),
            combinator,
            selectors: vec![],
        }
    }
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct TypeSelector<'a> {
    pub span: Span,
    pub name: &'a str,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct IdSelector<'a> {
    pub span: Span,
    pub name: &'a str,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct ClassSelector<'a> {
    pub span: Span,
    pub name: &'a str,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct AttributeSelector<'a> {
    pub span: Span,
    pub name: &'a str,
    pub matcher: Option<&'a str>,
    pub value: Option<&'a str>,
    pub flags: Option<&'a str>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct PseudoElementSelector<'a> {
    pub span: Span,
    pub name: &'a str,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct PseudoClassSelector<'a> {
    pub span: Span,
    pub name: &'a str,
    pub args: Option<SelectorList<'a>>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct Percentage<'a> {
    pub span: Span,
    pub value: &'a str,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct NestingSelector<'a> {
    pub span: Span,
    pub name: &'a str,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct Nth<'a> {
    pub span: Span,
    pub value: &'a str,
}

#[derive(Debug, AstTree, OxcSpan)]
pub enum SimpleSelector<'a> {
    TypeSelector(TypeSelector<'a>),
    IdSelector(IdSelector<'a>),
    ClassSelector(ClassSelector<'a>),
    AttributeSelector(AttributeSelector<'a>),
    PseudoElementSelector(PseudoElementSelector<'a>),
    PseudoClassSelector(PseudoClassSelector<'a>),
    Percentage(Percentage<'a>),
    Nth(Nth<'a>),
    NestingSelector(NestingSelector<'a>),
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct ComplexSelector<'a> {
    pub span: Span,
    pub children: Vec<RelativeSelector<'a>>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct Rule<'a> {
    pub span: Span,
    pub prelude: SelectorList<'a>,
    pub block: CSSBlock<'a>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct Declaration<'a> {
    pub span: Span,
    pub property: &'a str,
    pub value: &'a str,
}

#[derive(Debug, AstTree, OxcSpan)]
pub enum BlockChild<'a> {
    Rule(Rule<'a>),
    AtRule(AtRule<'a>),
    Declaration(Declaration<'a>),
}

#[derive(Debug, AstTree, OxcSpan)]
#[ast_tree(type = "Block")]
pub struct CSSBlock<'a> {
    pub span: Span,
    pub children: Vec<BlockChild<'a>>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct SelectorList<'a> {
    pub span: Span,
    pub children: Vec<ComplexSelector<'a>>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct Combinator<'a> {
    pub span: Span,
    pub name: &'a str,
}
