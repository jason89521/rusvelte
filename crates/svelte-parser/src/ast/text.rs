use std::borrow::Cow;

use crate::Parser;
use derive_macro::AstTree;
use oxc_span::Span;

#[derive(Debug, Clone, AstTree)]
pub struct Text<'a> {
    pub span: Span,
    pub raw: &'a str,
    pub data: Cow<'a, str>,
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
}
