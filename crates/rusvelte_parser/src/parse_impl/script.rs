use oxc_span::{GetSpan, Span};
use std::sync::LazyLock;

use regex::Regex;

use crate::{
    error::{ParserError, ParserErrorKind},
    Parser,
};

use rusvelte_ast::ast::{Attribute, AttributeValue, Script, ScriptContext};

static REGEX_CLOSING_SCRIPT_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<\/script\s*>"#).unwrap());
static REGEX_STARTS_WITH_CLOSING_SCRIPT_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^<\/script\s*>"#).unwrap());

const RESERVED_ATTRIBUTES: [&str; 5] = ["server", "client", "worker", "test", "default"];
const ALLOWED_ATTRIBUTES: [&str; 4] = ["context", "generics", "lang", "module"];

impl<'a> Parser<'a> {
    pub fn parse_script(
        &mut self,
        start: u32,
        attributes: Vec<Attribute<'a>>,
    ) -> Result<Script<'a>, ParserError> {
        let script_start = self.offset;
        let data = self.eat_until(&REGEX_CLOSING_SCRIPT_TAG);
        if self.remain().is_empty() {
            return Err(ParserError::new(
                Span::empty(self.offset),
                ParserErrorKind::ElementUnclosed(String::from("script")),
            ));
        }
        self.expect_regex(&REGEX_STARTS_WITH_CLOSING_SCRIPT_TAG)?;
        let program = self.parse_program(data, script_start)?;
        let context =
            attributes
                .iter()
                .try_fold(ScriptContext::Default, |context, attribute| {
                    if let Attribute::NormalAttribute(attribute) = attribute {
                        let name = attribute.name;
                        let attr_start = attribute.span.start;
                        if RESERVED_ATTRIBUTES.contains(&name) {
                            return Err(ParserError::new(
                                Span::sized(attr_start, name.len() as u32),
                                ParserErrorKind::ScriptReservedAttribute(name.to_string()),
                            ));
                        }
                        if !ALLOWED_ATTRIBUTES.contains(&name) {
                            // TODO warning
                        }
                        if name == "module" {
                            if let AttributeValue::True = &attribute.value {
                                return Ok(ScriptContext::Module);
                            } else {
                                return Err(ParserError::new(
                                    Span::sized(attribute.span.start, name.len() as u32),
                                    ParserErrorKind::ScriptInvalidAttributeValue(name.to_string()),
                                ));
                            }
                        }
                        if name == "context" {
                            if attribute.value.is_true() || !attribute.value.is_text() {
                                return Err(ParserError::new(
                                    attribute.span,
                                    ParserErrorKind::ScriptInvalidContext,
                                ));
                            }
                            if attribute.value.as_text().unwrap() != "module" {
                                return Err(ParserError::new(
                                    attribute.value.span(),
                                    ParserErrorKind::ScriptInvalidContext,
                                ));
                            }
                            return Ok(ScriptContext::Module);
                        }
                    }
                    Ok(context)
                })?;

        Ok(Script {
            span: Span::new(start, self.offset),
            context,
            content: program,
            attributes,
            leading_comment: None,
        })
    }
}
