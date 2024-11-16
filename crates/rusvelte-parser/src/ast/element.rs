use std::{collections::HashMap, ops::Deref, sync::LazyLock};

use derive_macro::{AstTree, OxcSpan};
use oxc_span::{GetSpan, Span};
use regex::Regex;
use utils::closing_tag_omitted;

use crate::{
    ast::Fragment,
    context::ParentKind,
    error::{ParserError, ParserErrorKind},
    regex_pattern::{REGEX_CLOSING_COMMENT, REGEX_WHITESPACE_OR_SLASH_OR_CLOSING_TAG},
    Context, LastAutoClosedTag, Parser,
};

use super::{attribute::Attribute, style_sheet::StyleSheet, FragmentNode, Script};

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

#[derive(Debug)]
pub enum ParseElementReturn<'a> {
    Element(Element<'a>),
    Comment(Comment<'a>),
    Script(Script<'a>),
    StyleSheet(StyleSheet<'a>),
    Nodes(Vec<FragmentNode<'a>>),
}

#[derive(Debug, AstTree, OxcSpan)]
pub enum Element<'a> {
    RegularElement(RegularElement<'a>),
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
    pub fn parse_element(&mut self) -> Result<ParseElementReturn<'a>, ParserError> {
        let start = self.offset;
        self.expect('<')?;

        // parse comment
        if self.eat_str("!--") {
            let data = self.eat_until(&REGEX_CLOSING_COMMENT);
            self.expect_str("-->")?;

            return Ok(ParseElementReturn::Comment(Comment {
                span: Span::new(start, self.offset),
                data,
            }));
        }

        let name = self.eat_until(&REGEX_WHITESPACE_OR_SLASH_OR_CLOSING_TAG);

        if self.parent_kind() == ParentKind::RegularElement
            && closing_tag_omitted(self.parent_name(), name)
        {
            self.set_parent_closed_at(start);
            self.last_auto_closed_tag = Some(LastAutoClosedTag {
                tag: &self.parent_name(),
                reason: name,
                depth: self.context_stack.len() as u8,
            })
        }

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

            return Ok(ParseElementReturn::Script(script));
        }

        if is_root_style {
            self.expect('>')?;
            let style_sheet = self.parse_style_sheet(start, attributes)?;

            return Ok(ParseElementReturn::StyleSheet(style_sheet));
        }

        // self closed element
        if self.eat('/') {
            self.expect('>')?;
            let span = Span::new(start, self.offset);
            let fragment = Fragment { nodes: vec![] };
            return Ok(ParseElementReturn::Element(Element::RegularElement(
                RegularElement {
                    span,
                    name,
                    fragment,
                    attributes,
                },
            )));
        }

        self.expect('>')?;
        self.push_context(Context::regular_element_context(name));
        let fragment = self.parse_fragment()?;

        let ctx = self.pop_context().expect("Expected self context.");
        if let Some(closed_at) = ctx.closed_at() {
            let mut nodes = vec![];
            let mut fragment_nodes = std::collections::VecDeque::from(fragment.nodes);
            while fragment_nodes
                .front()
                .map_or(false, |node| node.span().start < closed_at)
            {
                let node = fragment_nodes.pop_front().expect("Expected fragment node");
                nodes.push(node);
            }

            let element =
                FragmentNode::Element(Box::new(Element::RegularElement(RegularElement {
                    span: Span::new(start, closed_at),
                    name,
                    attributes,
                    fragment: Fragment { nodes },
                })));
            let mut nodes = vec![element];
            for node in fragment_nodes.into_iter() {
                nodes.push(node);
            }

            return Ok(ParseElementReturn::Nodes(nodes));
        }

        let closing_tag_name = self
            .peek_closing_tag_name()
            .ok_or(self.error(ParserErrorKind::ExpectedClosingTag))?;
        if closing_tag_name != name {
            // close any elements that don't have their own closing tags, e.g. <div><p></div>
            if self.parent_kind() != ParentKind::RegularElement {
                match self.last_auto_closed_tag.as_ref() {
                    Some(last_auto_closed_tag) if last_auto_closed_tag.tag == name => {
                        return Err(self.error_at(
                            start,
                            ParserErrorKind::ElementInvalidClosingTagAutoClosed {
                                reason: last_auto_closed_tag.reason.to_string(),
                                name: name.to_string(),
                            },
                        ))
                    }
                    _ => {
                        return Err(self.error_at(
                            start,
                            ParserErrorKind::ElementInvalidClosingTag(name.to_string()),
                        ))
                    }
                }
            }

            if let Some(v) = self.last_auto_closed_tag.as_ref() {
                if (self.context_stack.len() as u8) < v.depth {
                    self.last_auto_closed_tag = None;
                }
            }

            return Ok(ParseElementReturn::Element(Element::RegularElement(
                RegularElement {
                    fragment,
                    name,
                    span: Span::new(start, self.offset),
                    attributes,
                },
            )));
        }

        self.expect_regex(&REGEX_CLOSING_TAG)?;
        let element = RegularElement {
            fragment,
            name,
            span: Span::new(start, self.offset),
            attributes,
        };

        Ok(ParseElementReturn::Element(Element::RegularElement(
            element,
        )))
    }

    fn peek_closing_tag_name(&self) -> Option<&'a str> {
        REGEX_CLOSING_TAG
            .captures(&self.remain())
            .and_then(|caps| caps.get(1).map(|mat| mat.as_str()))
    }
}
