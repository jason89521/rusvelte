use derive_macro::AstTree;
use oxc_ast::ast::Expression;
use oxc_span::{GetSpan, Span};

use crate::{error::ParserError, regex_pattern::WHITESPACE_OR_SLASH_OR_CLOSING_TAG, Parser};

use super::{ExpressionTag, Text};

#[derive(Debug, AstTree)]
pub enum Attribute<'a> {
    NormalAttribute(NormalAttribute<'a>),
    SpreadAttribute(SpreadAttribute<'a>),
}

#[derive(Debug, AstTree)]
#[ast_tree(type = "Attribute")]
pub struct NormalAttribute<'a> {
    pub span: Span,
    pub name: &'a str,
    pub value: AttributeValue<'a>,
}

#[derive(Debug, AstTree)]
pub enum AttributeValue<'a> {
    ExpressionTag(ExpressionTag<'a>),
    Vec(Vec<QuotedAttributeValue<'a>>),
    True(bool),
}

#[derive(Debug, AstTree)]
pub enum QuotedAttributeValue<'a> {
    ExpressionTag(ExpressionTag<'a>),
    Text(Text<'a>),
}

#[derive(Debug, AstTree)]
pub struct SpreadAttribute<'a> {
    pub span: Span,
    pub expression: Expression<'a>,
}

impl<'a> Parser<'a> {
    pub fn parse_attributes(&mut self) -> Result<Vec<Attribute<'a>>, ParserError> {
        let mut attributes = vec![];
        while !self.match_regex(&WHITESPACE_OR_SLASH_OR_CLOSING_TAG) {
            attributes.push(self.parse_attribute()?);
            self.skip_whitespace();
        }

        Ok(attributes)
    }

    fn parse_attribute(&mut self) -> Result<Attribute<'a>, ParserError> {
        let start = self.offset;
        if self.eat('{') {
            self.skip_whitespace();
            if self.eat_str("...") {
                let expression = self.parse_expression()?;
                self.skip_whitespace();
                self.expect('}')?;

                return Ok(Attribute::SpreadAttribute(SpreadAttribute {
                    span: Span::new(start, self.offset),
                    expression,
                }));
            }

            // handle shorthand attr
            let (name, expression) = self.eat_identifier()?;
            self.skip_whitespace();
            self.expect('}')?;
            return Ok(Attribute::NormalAttribute(NormalAttribute {
                span: Span::new(start, self.offset),
                name,
                value: AttributeValue::ExpressionTag(ExpressionTag {
                    span: expression.span(),
                    expression,
                }),
            }));
        }

        unimplemented!()
    }
}
