use std::borrow::Cow;

use crate::Parser;
use derive_macro::{AstTree, OxcSpan};
use oxc_span::Span;

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

impl<'a> Parser<'a> {
    pub fn parse_text(&mut self) -> Text<'a> {
        let start = self.offset_u();
        while let Some(ch) = self.peek() {
            if matches!(ch, '<' | '{') {
                break;
            }
            self.next();
        }
        let raw = &self.source[start..self.offset_u()];
        let text = Text {
            span: Span::new(start as u32, self.offset),
            raw,
            data: htmlize::unescape(raw),
        };

        text
    }

    pub fn create_text(&self, span: Span) -> Text<'a> {
        Text::new(span, span.source_text(&self.source))
    }
}
