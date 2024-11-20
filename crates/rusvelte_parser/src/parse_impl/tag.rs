use oxc_span::Span;
use rusvelte_ast::ast::{ExpressionTag, Tag};

use crate::{error::ParserError, Parser};

impl<'a> Parser<'a> {
    pub fn parse_tag(&mut self, start: u32) -> Result<Tag<'a>, ParserError> {
        let expr = self.parse_expression()?;

        self.skip_whitespace();
        self.expect('}')?;

        Ok(Tag::ExpressionTag(ExpressionTag {
            span: Span::new(start, self.offset),
            expression: expr,
        }))
    }
}
