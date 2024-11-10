use oxc_ast::ast::Expression;
use oxc_span::Span;

use crate::{error::ParserError, Parser};

#[derive(Debug)]
pub enum Tag<'a> {
    ExpressionTag(ExpressionTag<'a>),
}

#[derive(Debug)]
pub struct ExpressionTag<'a> {
    pub span: Span,
    pub expression: Expression<'a>,
}

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
