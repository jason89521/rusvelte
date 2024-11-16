use std::sync::LazyLock;

use derive_macro::{AstTree, OxcSpan};
use oxc_ast::ast::Expression;
use oxc_span::{GetSpan, Span, SPAN};
use regex::Regex;

use crate::{
    error::{ParserError, ParserErrorKind},
    Parser,
};

use super::{ExpressionTag, Text};

pub static REGEX_TOKEN_ENDING_CHARACTER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"[\s=/>"']"#).unwrap());
pub static REGEX_ATTRIBUTE_VALUE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^(?:"([^"]*)"|'([^'])*'|([^>\s]+))"#).unwrap());
pub static REGEX_STARTS_WITH_QUOTE_CHARACTERS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^["']"#).unwrap());

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
    True,
}

impl oxc_span::GetSpan for AttributeValue<'_> {
    fn span(&self) -> Span {
        match self {
            AttributeValue::ExpressionTag(expression_tag) => expression_tag.span,
            AttributeValue::Vec(vec) => {
                let start = vec.iter().next().unwrap().span().start;
                let end = vec.iter().last().unwrap().span().end;
                return Span::new(start, end);
            }
            AttributeValue::True => SPAN,
        }
    }
}

#[derive(Debug, AstTree, OxcSpan)]
pub enum QuotedAttributeValue<'a> {
    ExpressionTag(ExpressionTag<'a>),
    Text(Text<'a>),
}

#[derive(Debug, AstTree)]
pub struct SpreadAttribute<'a> {
    pub span: Span,
    pub expression: Expression<'a>,
}

impl<'a> Attribute<'a> {
    pub fn is_text_attribute(&self) -> bool {
        match self {
            Attribute::NormalAttribute(attribute) => attribute.value.is_text(),
            _ => false,
        }
    }
}

impl<'a> AttributeValue<'a> {
    pub fn is_true(&self) -> bool {
        match self {
            AttributeValue::True => true,
            _ => false,
        }
    }

    pub fn is_text(&self) -> bool {
        if let AttributeValue::Vec(values) = self {
            values.len() == 1
                && if let QuotedAttributeValue::Text(_) = &values[0] {
                    true
                } else {
                    false
                }
        } else {
            false
        }
    }

    pub fn as_text(&self) -> Option<&str> {
        if !self.is_text() {
            return None;
        }
        if let AttributeValue::Vec(values) = self {
            if let QuotedAttributeValue::Text(text) = &values[0] {
                return Some(&text.data);
            }
        }
        None
    }
}

impl<'a> Parser<'a> {
    pub fn parse_attributes(
        &mut self,
        parse_static: bool,
    ) -> Result<Vec<Attribute<'a>>, ParserError> {
        let mut attributes = vec![];
        while let Some(attr) = self.parse_attribute_impl(parse_static)? {
            attributes.push(attr);
            self.skip_whitespace();
        }
        Ok(attributes)
    }

    fn parse_attribute_impl(
        &mut self,
        parse_static: bool,
    ) -> Result<Option<Attribute<'a>>, ParserError> {
        if parse_static {
            self.parse_static_attribute()
        } else {
            self.parse_attribute()
        }
    }

    fn parse_attribute(&mut self) -> Result<Option<Attribute<'a>>, ParserError> {
        let start = self.offset;
        if self.eat('{') {
            self.skip_whitespace();
            if self.eat_str("...") {
                let expression = self.parse_expression()?;
                self.skip_whitespace();
                self.expect('}')?;

                return Ok(Some(Attribute::SpreadAttribute(SpreadAttribute {
                    span: Span::new(start, self.offset),
                    expression,
                })));
            }

            // handle shorthand attr
            let (name, expression) = self.eat_identifier()?;
            self.skip_whitespace();
            self.expect('}')?;
            return Ok(Some(Attribute::NormalAttribute(NormalAttribute {
                span: Span::new(start, self.offset),
                name,
                value: AttributeValue::ExpressionTag(ExpressionTag {
                    span: expression.span(),
                    expression,
                }),
            })));
        }

        // TODO other attribute type
        return Ok(None);
    }

    fn parse_static_attribute(&mut self) -> Result<Option<Attribute<'a>>, ParserError> {
        let start = self.offset;
        let name = self.eat_until(&REGEX_TOKEN_ENDING_CHARACTER);
        if name == "" {
            return Ok(None);
        }
        let mut value = AttributeValue::True;
        if self.eat('=') {
            self.skip_whitespace();
            let mut raw = if let Some(raw) = self.match_regex(&REGEX_ATTRIBUTE_VALUE) {
                raw
            } else {
                return Err(ParserError::new(
                    Span::empty(self.offset),
                    ParserErrorKind::ExpectedAttributeValue,
                ));
            };
            self.offset += raw.len() as u32;
            let quoted = match raw.chars().next().unwrap() {
                '\'' | '"' => true,
                _ => false,
            };
            if quoted {
                raw = {
                    let mut chars = raw.chars();
                    chars.next();
                    chars.next_back();
                    chars.as_str()
                }
            }

            value = AttributeValue::Vec(vec![QuotedAttributeValue::Text(Text::new(
                Span::new(
                    self.offset - raw.len() as u32 - if quoted { 1 } else { 0 },
                    if quoted { self.offset - 1 } else { self.offset },
                ),
                raw,
            ))]);
        }

        if self
            .match_regex(&REGEX_STARTS_WITH_QUOTE_CHARACTERS)
            .is_some()
        {
            return Err(ParserError::new(
                Span::empty(self.offset),
                ParserErrorKind::ExpectedToken('='),
            ));
        }

        Ok(Some(Attribute::NormalAttribute(NormalAttribute {
            span: Span::new(start, self.offset),
            name,
            value,
        })))
    }
}
