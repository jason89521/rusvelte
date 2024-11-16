use std::{collections::HashMap, ops::Deref, sync::LazyLock};

use derive_macro::{AstTree, OxcSpan};
use oxc_span::Span;
use regex::Regex;

use crate::{
    ast::Fragment,
    error::{ParserError, ParserErrorKind},
    regex_pattern::{REGEX_CLOSING_COMMENT, REGEX_WHITESPACE_OR_SLASH_OR_CLOSING_TAG},
    Context, Parser,
};

use super::{attribute::Attribute, style_sheet::StyleSheet, Script};

static ROOT_ONLY_META_TAGS: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from([
        ("svelte:head", "SvelteHead"),
        ("svelte:options", "SvelteOptions"),
        ("svelte:window", "SvelteWindow"),
        ("svelte:document", "SvelteDocument"),
        ("svelte:body", "SvelteBody"),
    ])
});
static META_TAGS: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    ROOT_ONLY_META_TAGS
        .clone()
        .into_iter()
        .chain(
            HashMap::from([
                ("svelte:element", "SvelteElement"),
                ("svelte:component", "SvelteComponent"),
                ("svelte:self", "SvelteSelf"),
                ("svelte:fragment", "SvelteFragment"),
            ])
            .into_iter(),
        )
        .collect()
});

static REGEX_VALID_ELEMENT_NAME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:![a-zA-Z]+|[a-zA-Z](?:[a-zA-Z0-9-]*[a-zA-Z0-9])?|[a-zA-Z][a-zA-Z0-9]*:[a-zA-Z][a-zA-Z0-9-]*[a-zA-Z0-9])$").unwrap()
});
static REGEX_VALID_COMPONENT_NAME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:\p{Lu}[$\u200c\u200d\p{ID_Continue}.]*|\p{ID_Start}[$\u200c\u200d\p{ID_Continue}]*(?:\.[$\u200c\u200d\p{ID_Continue}]+)+)$").unwrap()
});
static REGEX_CLOSING_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^<\/\s*(\S*)\s*>").unwrap());

#[derive(Debug, AstTree, OxcSpan)]
pub enum Element<'a> {
    RegularElement(RegularElement<'a>),
    Comment(Comment<'a>),
    Script(Script<'a>),
    StyleSheet(StyleSheet<'a>),
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct RegularElement<'a> {
    pub span: Span,
    pub name: &'a str,
    pub attributes: Vec<Attribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct Comment<'a> {
    pub span: Span,
    pub data: &'a str,
}

impl<'a> Parser<'a> {
    // now can only parse regular element
    pub fn parse_element(&mut self) -> Result<Element<'a>, ParserError> {
        let start = self.offset;
        self.expect('<')?;

        // parse comment
        if self.eat_str("!--") {
            let data = self.eat_until(&REGEX_CLOSING_COMMENT);
            self.expect_str("-->")?;

            return Ok(Element::Comment(Comment {
                span: Span::new(start, self.offset),
                data,
            }));
        }

        let name = self.eat_until(&REGEX_WHITESPACE_OR_SLASH_OR_CLOSING_TAG);

        if name.starts_with("svelte:") && !META_TAGS.contains_key(name) {
            return Err(ParserError::new(
                Span::new(start + 1, start + 1 + name.len() as u32),
                ParserErrorKind::SvelteMetaInvalidTag(META_TAGS.keys().map(Deref::deref).collect()),
            ));
        }

        if !REGEX_VALID_ELEMENT_NAME.is_match(name) && !REGEX_VALID_COMPONENT_NAME.is_match(name) {
            return Err(ParserError::new(
                Span::new(start + 1, start + 1 + name.len() as u32),
                ParserErrorKind::TagInvalidName,
            ));
        }

        if ROOT_ONLY_META_TAGS.contains_key(name) {
            if self.meta_tag_exist(name) {
                return Err(self.error_at(
                    start,
                    ParserErrorKind::SvelteMetaDuplicate(name.to_string()),
                ));
            }
            if !self.is_parent_root() {
                return Err(self.error_at(
                    start,
                    ParserErrorKind::SvelteMetaInvalidPlacement(name.to_string()),
                ));
            }
            self.meta_tags.insert(name);
        }

        self.skip_whitespace();
        let is_root_script = self.is_parent_root() && name == "script";
        let is_root_style = self.is_parent_root() && name == "style";
        let attributes = self.parse_attributes(is_root_script)?;

        if is_root_script {
            self.expect('>')?;
            let script = self.parse_script(start, attributes)?;

            return Ok(Element::Script(script));
        }

        if is_root_style {
            self.expect('>')?;
            let style_sheet = self.parse_style_sheet(start, attributes)?;

            return Ok(Element::StyleSheet(style_sheet));
        }

        // self closed element
        if self.eat('/') {
            self.expect('>')?;
            let span = Span::new(start, self.offset);
            let fragment = Fragment { nodes: vec![] };
            return Ok(Element::RegularElement(RegularElement {
                span,
                name,
                fragment,
                attributes,
            }));
        }

        self.expect('>')?;
        self.push_context(Context::default());
        let fragment = self.parse_fragment()?;
        self.pop_context();
        let closing_tag_name = self
            .peek_closing_tag_name()
            .ok_or(self.error(ParserErrorKind::ExpectedClosingTag))?;
        if closing_tag_name != name {
            // close any elements that don't have their own closing tags, e.g. <div><p></div>
            // TODO: check whether this element is regular.
            return Ok(Element::RegularElement(RegularElement {
                fragment,
                name,
                span: Span::new(start, self.offset),
                attributes,
            }));
        }
        self.expect_regex(&REGEX_CLOSING_TAG)?;
        let element = RegularElement {
            fragment,
            name,
            span: Span::new(start, self.offset),
            attributes,
        };

        Ok(Element::RegularElement(element))
    }

    fn peek_closing_tag_name(&self) -> Option<&'a str> {
        REGEX_CLOSING_TAG
            .captures(&self.remain())
            .and_then(|caps| caps.get(1).map(|mat| mat.as_str()))
    }
}
