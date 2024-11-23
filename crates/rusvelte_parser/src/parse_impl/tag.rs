use oxc_ast::ast::{ChainElement, Expression};
use oxc_span::{GetSpan, Span};
use rusvelte_ast::ast::{
    ConstTag, DebugTag, ExpressionTag, HtmlTag, RenderTag, RenderTagExpression, Tag,
};

use crate::{
    error::{ParserError, ParserErrorKind},
    regex_pattern::REGEX_START_WHITESPACE_WITH_CLOSING_CURLY_BRACE,
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_tag(&mut self, start: u32) -> Result<Tag<'a>, ParserError> {
        let get_span = |parser: &Self| Span::new(start, parser.offset);

        if self.eat('@') {
            if self.eat_str("html") {
                self.expect_whitespace()?;
                let expression = self.parse_expression()?;
                self.expect_close_tag()?;
                Ok(Tag::HtmlTag(HtmlTag {
                    span: get_span(self),
                    expression,
                }))
            } else if self.eat_str("debug") {
                let identifiers = if self
                    .eat_regex(&REGEX_START_WHITESPACE_WITH_CLOSING_CURLY_BRACE)
                    .is_some()
                {
                    vec![]
                } else {
                    let mut result = vec![];
                    let mut error_start = self.offset;
                    self.expect_whitespace()?;
                    while let Some((.., ident)) = self.eat_identifier()? {
                        result.push(ident);
                        self.skip_whitespace();
                        self.eat(',');
                        self.skip_whitespace();
                        error_start = self.offset;
                    }

                    if !self.eat('}') {
                        return Err(
                            self.error_at(error_start, ParserErrorKind::DebugTagInvalidArguments)
                        );
                    }

                    result
                };

                Ok(Tag::DebugTag(DebugTag {
                    span: get_span(self),
                    identifiers,
                }))
            } else if self.match_str("const") {
                // use match here because we are going to consume it.
                let declaration = self.parse_variable_declaration()?;
                if declaration.declarations.len() > 1 {
                    return Err(ParserError {
                        span: declaration.span,
                        kind: ParserErrorKind::ConstTagInvalidExpression,
                    });
                }
                self.expect_close_tag()?;
                Ok(Tag::ConstTag(ConstTag {
                    span: get_span(self),
                    declaration,
                }))
            } else if self.eat_str("render") {
                self.expect_whitespace()?;
                let expression = match self.parse_expression()? {
                    Expression::CallExpression(expr) => {
                        RenderTagExpression::CallExpression(expr.unbox())
                    }
                    // TODO: if the only matter thing is CallExpression after we implement analyzer and transformer,
                    // remove this type and use CallExpression directly
                    Expression::ChainExpression(expr)
                        if matches!(expr.expression, ChainElement::CallExpression(_)) =>
                    {
                        RenderTagExpression::ChainExpression(expr.unbox())
                    }
                    expr => {
                        return Err(ParserError {
                            span: expr.span(),
                            kind: ParserErrorKind::RenderTagInvalidExpression,
                        })
                    }
                };
                self.expect_close_tag()?;
                Ok(Tag::RenderTag(RenderTag {
                    span: get_span(self),
                    expression,
                }))
            } else {
                // Svelte parser doesn't return error here, weird
                Err(self.error(ParserErrorKind::ExpectedTagType))
            }
        } else {
            let expr = self.parse_expression()?;
            self.skip_whitespace();
            self.expect('}')?;

            Ok(Tag::ExpressionTag(ExpressionTag {
                span: Span::new(start, self.offset),
                expression: expr,
            }))
        }
    }

    fn expect_close_tag(&mut self) -> Result<(), ParserError> {
        self.skip_whitespace();
        self.expect('}')?;
        Ok(())
    }
}
