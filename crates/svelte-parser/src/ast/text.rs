use crate::Parser;
use oxc_span::Span;

#[derive(Debug, Clone)]
pub struct Text<'a> {
    pub span: Span,
    pub data: String,
    pub raw: &'a str,
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
        // TODO: data should be decoded
        let data = raw.to_string();
        let text = Text {
            span: Span::new(start as u32, self.offset),
            raw,
            data,
        };

        text
    }
}
