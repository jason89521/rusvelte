use crate::Parser;
use oxc_span::Span;
use rusvelte_ast::ast::Text;

impl<'a> Parser<'a> {
    pub fn parse_text(&mut self) -> Text<'a> {
        let start = self.offset_usize();
        while let Some(ch) = self.peek() {
            if matches!(ch, '<' | '{') {
                break;
            }
            self.next();
        }
        let raw = &self.source[start..self.offset_usize()];
        let text = Text::new(Span::new(start as u32, self.offset), raw);

        text
    }

    pub fn create_text(&self, span: Span) -> Text<'a> {
        Text::new(span, span.source_text(self.source))
    }
}
