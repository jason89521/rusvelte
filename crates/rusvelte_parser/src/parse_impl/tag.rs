use oxc_span::Span;
use rusvelte_ast::ast::{ExpressionTag, Tag};

use crate::{error::ParserError, Parser};

impl<'a> Parser<'a> {
    pub fn parse_tag(&mut self) -> Result<Tag<'a>, ParserError> {
        let start = self.offset;
        self.next();

        let expr = self.parse_expression()?;

        self.expect('}')?;

        Ok(Tag::ExpressionTag(ExpressionTag {
            span: Span::new(start, self.offset),
            expression: expr,
        }))
    }
}
