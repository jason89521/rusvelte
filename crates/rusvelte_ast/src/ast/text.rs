use std::borrow::Cow;

use oxc_span::Span;
use rusvelte_derive::{AstTree, OxcSpan};

#[derive(Debug, Clone, AstTree, OxcSpan)]
pub struct Text<'a> {
    pub span: Span,
    pub raw: &'a str,
    pub data: Cow<'a, str>,
}

impl<'a> Text<'a> {
    pub fn new(span: Span, raw: &'a str) -> Self {
        Self {
            span,
            raw,
            data: htmlize::unescape(raw),
        }
    }
}
