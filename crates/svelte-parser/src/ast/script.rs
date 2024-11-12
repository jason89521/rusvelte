use derive_macro::{AstTree, OxcSpan};
use oxc_ast::ast::Program;
use oxc_span::Span;
use serde::Serialize;
use std::sync::LazyLock;

use regex::Regex;

use crate::{ast::attribute::AttributeValue, error::ParserError, Parser};

use super::attribute::Attribute;

pub static REGEX_CLOSING_SCRIPT_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<\/script\s*>"#).unwrap());
pub static REGEX_STARTS_WITH_CLOSING_SCRIPT_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^<\/script\s*>"#).unwrap());

const RESERVED_ATTRIBUTES: [&str; 5] = ["server", "client", "worker", "test", "default"];
const ALLOWED_ATTRIBUTES: [&str; 4] = ["context", "generics", "lang", "module"];

#[derive(Debug, AstTree, OxcSpan)]
pub struct Script<'a> {
    pub span: Span,
    pub context: ScriptContext,
    pub content: Program<'a>,
    pub attributes: Vec<Attribute<'a>>,
}

#[derive(Debug)]
pub enum ScriptContext {
    Default,
    Module,
}

impl Serialize for ScriptContext {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ScriptContext::Default => serializer.serialize_str("default"),
            ScriptContext::Module => serializer.serialize_str("module"),
        }
    }
}

impl<'a> Parser<'a> {
    pub fn parse_script(
        &mut self,
        start: u32,
        attributes: Vec<Attribute<'a>>,
    ) -> Result<Script<'a>, ParserError> {
        let script_start = self.offset;
        let data = self.eat_until(&REGEX_CLOSING_SCRIPT_TAG);
        if self.remain().len() == 0 {
            return Err(ParserError::ElementUnclosed(String::from("script")));
        }
        self.expect_regex(&REGEX_STARTS_WITH_CLOSING_SCRIPT_TAG)?;
        let program = self.parse_program(data, script_start)?;
        let context =
            attributes
                .iter()
                .fold(Ok(ScriptContext::Default), |context, attribute| {
                    if let Attribute::NormalAttribute(attribute) = attribute {
                        let name = attribute.name;
                        if RESERVED_ATTRIBUTES.contains(&name) {
                            return Err(ParserError::ScriptReservedAttribute(name.to_string()));
                        }
                        if !ALLOWED_ATTRIBUTES.contains(&name) {
                            // TODO warning
                        }
                        if name == "module" {
                            if let AttributeValue::True = &attribute.value {
                                return Ok(ScriptContext::Module);
                            } else {
                                return Err(ParserError::ScriptInvalidAttributeValue(
                                    name.to_string(),
                                ));
                            }
                        }
                        if name == "context" {
                            if attribute.value.is_true() || !attribute.value.is_text() {
                                return Err(ParserError::ScriptInvalidContext);
                            }
                            if attribute.value.as_text().unwrap() != "module" {
                                return Err(ParserError::ScriptInvalidContext);
                            }
                            return Ok(ScriptContext::Module);
                        }
                    }
                    context
                })?;

        Ok(Script {
            span: Span::new(start, self.offset),
            context,
            content: program,
            attributes,
        })
    }
}
