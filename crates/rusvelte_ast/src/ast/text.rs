use std::borrow::Cow;

use oxc_span::{Atom, Span};
use rusvelte_derive::{AstTree, OxcSpan};

#[derive(Debug, Clone, AstTree, OxcSpan)]
pub struct Text<'a> {
    pub span: Span,
    pub raw: Atom<'a>,
    pub data: Cow<'a, str>,
}

impl<'a> Text<'a> {
    pub fn new(span: Span, raw: &'a str) -> Self {
        Self {
            span,
            raw: raw.into(),
            data: htmlize::unescape(raw),
        }
    }
}
