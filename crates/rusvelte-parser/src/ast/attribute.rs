use std::{collections::HashSet, sync::LazyLock};

use derive_macro::{AstTree, OxcSpan};
use oxc_ast::ast::Expression;
use oxc_span::{GetSpan, Span, SPAN};
use regex::Regex;

use crate::{
    ast::SequenceValue,
    error::{ParserError, ParserErrorKind},
    Parser,
};

use super::{
    directive::{Directive, DirectiveKind, StyleDirective},
    ExpressionTag, Text,
};

static REGEX_TOKEN_ENDING_CHARACTER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"[\s=/>"']"#).unwrap());
static REGEX_ATTRIBUTE_VALUE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^(?:"([^"]*)"|'([^'])*'|([^>\s]+))"#).unwrap());
static REGEX_STARTS_WITH_QUOTE_CHARACTERS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^["']"#).unwrap());
static REGEX_INVALID_UNQUOTED_ATTRIBUTE_VALUE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^(\/>|[\s"'=<>`])"#).unwrap());

#[derive(Debug, AstTree, OxcSpan)]
pub enum Attribute<'a> {
    NormalAttribute(NormalAttribute<'a>),
    SpreadAttribute(SpreadAttribute<'a>),
    Directive(Directive<'a>),
}

#[derive(Debug, AstTree, OxcSpan)]
#[ast_tree(type = "Attribute")]
pub struct NormalAttribute<'a> {
    pub span: Span,
    pub name: &'a str,
    pub value: AttributeValue<'a>,
}

#[derive(Debug, AstTree)]
pub enum AttributeValue<'a> {
    ExpressionTag(ExpressionTag<'a>),
    Quoted(Vec<QuotedAttributeValue<'a>>),
    True,
}

impl oxc_span::GetSpan for AttributeValue<'_> {
    fn span(&self) -> Span {
        match self {
            AttributeValue::ExpressionTag(expression_tag) => expression_tag.span,
            AttributeValue::Quoted(vec) => {
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

#[derive(Debug, AstTree, OxcSpan)]
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
        if let AttributeValue::Quoted(values) = self {
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
        if let AttributeValue::Quoted(values) = self {
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
        let mut unique_names: HashSet<(&str, &str)> = HashSet::new();
        while let Some(attr) = self.parse_attribute_impl(parse_static)? {
            let key = match &attr {
                Attribute::NormalAttribute(attr) => Some(("Attribute", attr.name)),
                Attribute::Directive(d) if matches!(d, Directive::BindDirective(_)) => {
                    Some(("Attribute", d.name()))
                }
                Attribute::Directive(d)
                    if matches!(
                        d,
                        Directive::ClassDirective(_) | Directive::StyleDirective(_)
                    ) =>
                {
                    Some((d.kind_str(), d.name()))
                }
                _ => None,
            };

            if let Some(key) = key {
                // `bind:attribute` and `attribute` are just the same but `class:attribute`,
                // `style:attribute` and `attribute` are different and should be allowed together
                // so we concatenate the type while normalizing the type for BindDirective
                if unique_names.contains(&key) {
                    return Err(ParserError::new(
                        attr.span(),
                        ParserErrorKind::AttributeDuplicate,
                    ));
                // <svelte:element bind:this this=..> is allowed
                } else if key.1 != "this" {
                    unique_names.insert(key);
                }
            }

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
            let (name, expression) = if let Some(v) = self.eat_identifier()? {
                v
            } else {
                return Err(self.error(ParserErrorKind::AttributeEmptyShorthand));
            };
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

        let name = self.eat_until(&REGEX_TOKEN_ENDING_CHARACTER);
        if name == "" {
            return Ok(None);
        }

        let mut end = self.offset;
        self.skip_whitespace();

        let mut value = AttributeValue::True;
        if self.eat('=') {
            self.skip_whitespace();
            value = self.parse_attribute_value()?;
            end = self.offset;
        } else if self
            .match_regex(&REGEX_STARTS_WITH_QUOTE_CHARACTERS)
            .is_some()
        {
            return Err(self.error(ParserErrorKind::ExpectedToken('=')));
        }

        let directive_meta = name.find(':').and_then(|idx| {
            let directive_kind = DirectiveKind::from_name(&name[..idx])?;
            let (directive_name, modifiers) = if idx + 1 >= name.len() {
                ("", vec![])
            } else {
                let mut split = name[idx + 1..].split('|');
                (
                    split.next().expect("Expected a directive name"),
                    split.collect(),
                )
            };

            Some((idx, directive_kind, directive_name, modifiers))
        });

        if let Some((colon_idx, directive_kind, directive_name, modifiers)) = directive_meta {
            if directive_name == "" {
                return Err(ParserError::new(
                    Span::new(start, start + colon_idx as u32 + 1),
                    ParserErrorKind::DirectiveMissingName(name.to_string()),
                ));
            }

            if directive_kind == DirectiveKind::StyleDirective {
                return Ok(Some(Attribute::Directive(Directive::StyleDirective(
                    StyleDirective {
                        span: Span::new(start, end),
                        name: directive_name,
                        value,
                        modifiers,
                    },
                ))));
            }

            if value.is_text() || matches!(&value, AttributeValue::Quoted(q) if q.len() > 1) {
                return Err(
                    self.error_at(value.span().start, ParserErrorKind::DirectiveInvalidValue)
                );
            }
            let expression = match value {
                AttributeValue::ExpressionTag(expression_tag) => Some(expression_tag.expression),
                // for "{expr}"
                AttributeValue::Quoted(mut quoted) if quoted.len() == 1 => {
                    if let QuotedAttributeValue::ExpressionTag(tag) =
                        quoted.pop().expect("Expect a value")
                    {
                        Some(tag.expression)
                    } else {
                        None
                    }
                }
                _ => None,
            };
            let mut directive = Directive::create(
                Span::new(start, end),
                directive_kind,
                directive_name,
                expression,
                modifiers,
            )?;

            // Directive name is expression, e.g. <p class:isRed />
            if matches!(
                directive_kind,
                DirectiveKind::BindDirective | DirectiveKind::ClassDirective
            ) && directive.expression().is_none()
            {
                directive.set_expression(
                    self.parse_expression_in(directive_name, start + colon_idx as u32 + 1)?,
                );
            }

            return Ok(Some(Attribute::Directive(directive)));
        }

        return Ok(Some(Attribute::NormalAttribute(NormalAttribute {
            span: Span::new(start, end),
            value,
            name,
        })));
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

            value = AttributeValue::Quoted(vec![QuotedAttributeValue::Text(Text::new(
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

    fn parse_attribute_value(&mut self) -> Result<AttributeValue<'a>, ParserError> {
        let quote_mark = {
            if self.eat('"') {
                Some('"')
            } else if self.eat('\'') {
                Some('\'')
            } else {
                None
            }
        };

        if let Some(quote_mark) = quote_mark {
            if self.eat(quote_mark) {
                return Ok(AttributeValue::Quoted(vec![QuotedAttributeValue::Text(
                    Text::new(Span::new(self.offset - 1, self.offset - 1), ""),
                )]));
            }
        }

        let mut value = self.parse_sequence(
            |parser| {
                if let Some(quote_mark) = quote_mark {
                    parser.match_ch(quote_mark)
                } else {
                    parser
                        .match_regex(&REGEX_INVALID_UNQUOTED_ATTRIBUTE_VALUE)
                        .is_some()
                }
            },
            "in attribute value",
        )?;

        if value.len() == 0 && quote_mark.is_none() {
            return Err(self.error(ParserErrorKind::ExpectedAttributeValue));
        }

        if let Some(quote_mark) = quote_mark {
            self.expect(quote_mark)?;
        }

        if quote_mark.is_some() || value.len() > 1 || matches!(&value[0], SequenceValue::Text(_)) {
            return Ok(AttributeValue::Quoted(
                value
                    .into_iter()
                    .map(|v| match v {
                        SequenceValue::ExpressionTag(expression_tag) => {
                            QuotedAttributeValue::ExpressionTag(expression_tag)
                        }
                        SequenceValue::Text(text) => QuotedAttributeValue::Text(text),
                    })
                    .collect(),
            ));
        }

        if let SequenceValue::ExpressionTag(tag) = value.remove(0) {
            Ok(AttributeValue::ExpressionTag(tag))
        } else {
            unreachable!("Expect expression tag in the sequence's first value.")
        }
    }
}
