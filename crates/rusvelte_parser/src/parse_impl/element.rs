use std::{collections::HashSet, ops::Deref, sync::LazyLock};

use oxc_span::{GetSpan, Span};
use regex::Regex;
use rusvelte_utils::closing_tag_omitted;

use crate::{
    context::ParentKind,
    error::{ParserError, ParserErrorKind},
    regex_pattern::{REGEX_CLOSING_COMMENT, REGEX_WHITESPACE_OR_SLASH_OR_CLOSING_TAG},
    Context, LastAutoClosedTag, Parser,
};

use rusvelte_ast::ast::{
    Comment, Element, ExpressionTag, Fragment, FragmentNode, RegularElement, Script, StyleSheet,
    Text,
};

const SVELTE_HEAD_TAG: &'static str = "svelte:head";
const SVELTE_OPTIONS_TAG: &'static str = "svelte:options";
const SVELTE_WINDOW_TAG: &'static str = "svelte:window";
const SVELTE_DOCUMENT_TAG: &'static str = "svelte:document";
const SVELTE_BODY_TAG: &'static str = "svelte:body";
const SVELTE_ELEMENT_TAG: &'static str = "svelte:element";
const SVELTE_COMPONENT_TAG: &'static str = "svelte:component";
const SVELTE_SELF_TAG: &'static str = "svelte:self";
const SVELTE_FRAGMENT_TAG: &'static str = "svelte:fragment";

static ROOT_ONLY_META_TAGS: LazyLock<HashSet<&str>> = LazyLock::new(|| {
    HashSet::from([
        SVELTE_HEAD_TAG,
        SVELTE_OPTIONS_TAG,
        SVELTE_WINDOW_TAG,
        SVELTE_DOCUMENT_TAG,
        SVELTE_BODY_TAG,
    ])
});
static META_TAGS: LazyLock<HashSet<&str>> = LazyLock::new(|| {
    HashSet::from([
        SVELTE_HEAD_TAG,
        SVELTE_OPTIONS_TAG,
        SVELTE_WINDOW_TAG,
        SVELTE_DOCUMENT_TAG,
        SVELTE_BODY_TAG,
        SVELTE_ELEMENT_TAG,
        SVELTE_COMPONENT_TAG,
        SVELTE_SELF_TAG,
        SVELTE_FRAGMENT_TAG,
    ])
});

static REGEX_VALID_ELEMENT_NAME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:![a-zA-Z]+|[a-zA-Z](?:[a-zA-Z0-9-]*[a-zA-Z0-9])?|[a-zA-Z][a-zA-Z0-9]*:[a-zA-Z][a-zA-Z0-9-]*[a-zA-Z0-9])$").unwrap()
});
static REGEX_VALID_COMPONENT_NAME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:\p{Lu}[$\u200c\u200d\p{ID_Continue}.]*|\p{ID_Start}[$\u200c\u200d\p{ID_Continue}]*(?:\.[$\u200c\u200d\p{ID_Continue}]+)+)$").unwrap()
});
static REGEX_CLOSING_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^<\/\s*(\S*)\s*>").unwrap());
static REGEX_NOT_LOWERCASE_A_TO_Z: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[^a-z]").unwrap());

#[derive(Debug)]
pub enum ParseElementReturn<'a> {
    Element(Element<'a>),
    Comment(Comment<'a>),
    Script(Script<'a>),
    StyleSheet(StyleSheet<'a>),
    Nodes(Vec<FragmentNode<'a>>),
}

#[derive(Debug)]
pub enum SequenceValue<'a> {
    ExpressionTag(ExpressionTag<'a>),
    Text(Text<'a>),
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

        if name.starts_with("svelte:") && !META_TAGS.contains(name) {
            return Err(ParserError::new(
                Span::new(start + 1, start + 1 + name.len() as u32),
                ParserErrorKind::SvelteMetaInvalidTag(META_TAGS.iter().map(Deref::deref).collect()),
            ));
        }

        if !REGEX_VALID_ELEMENT_NAME.is_match(name) && !REGEX_VALID_COMPONENT_NAME.is_match(name) {
            return Err(ParserError::new(
                Span::new(start + 1, start + 1 + name.len() as u32),
                ParserErrorKind::TagInvalidName,
            ));
        }

        if ROOT_ONLY_META_TAGS.contains(name) {
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

        // TODO: implement element
        #[allow(unused_variables)]
        let element_type = {
            match name {
                SVELTE_HEAD_TAG => unimplemented!(),
                SVELTE_OPTIONS_TAG => unimplemented!(),
                SVELTE_WINDOW_TAG => unimplemented!(),
                SVELTE_DOCUMENT_TAG => unimplemented!(),
                SVELTE_BODY_TAG => unimplemented!(),
                SVELTE_ELEMENT_TAG => unimplemented!(),
                SVELTE_COMPONENT_TAG => unimplemented!(),
                SVELTE_SELF_TAG => unimplemented!(),
                SVELTE_FRAGMENT_TAG => unimplemented!(),
                _ => (),
            };

            if REGEX_VALID_COMPONENT_NAME.is_match(name) {
                // Component
            }

            if name == "title" {
                // TitleElement
            }

            // RegularElement
        };

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

    /// Used for parse attribute value and textarea's nodes
    pub(crate) fn parse_sequence<F>(
        &mut self,
        done: F,
        location: &'static str,
    ) -> Result<Vec<SequenceValue<'a>>, ParserError>
    where
        F: Fn(&Self) -> bool,
    {
        let mut text_start = self.offset;

        let mut values = vec![];
        let mut flush = |value: SequenceValue<'a>| {
            match &value {
                SequenceValue::Text(text) if text.raw.is_empty() => (),
                _ => values.push(value),
            };
        };

        while let Some(_) = self.peek() {
            if done(&self) {
                flush(SequenceValue::Text(
                    self.create_text(Span::new(text_start, self.offset)),
                ));
                return Ok(values);
            } else if self.eat('{') {
                let tag_start = self.offset - 1;
                if self.eat('#') {
                    let name = self.eat_until(&REGEX_NOT_LOWERCASE_A_TO_Z);
                    return Err(self.error_at(
                        tag_start,
                        ParserErrorKind::BlockInvalidPlacement {
                            name: name.to_string(),
                            location: location.to_string(),
                        },
                    ));
                } else if self.eat('@') {
                    let name = self.eat_until(&REGEX_NOT_LOWERCASE_A_TO_Z);
                    return Err(self.error_at(
                        tag_start,
                        ParserErrorKind::TagInvalidPlacement {
                            name: name.to_string(),
                            location: location.to_string(),
                        },
                    ));
                }

                flush(SequenceValue::Text(
                    self.create_text(Span::new(text_start, self.offset - 1)),
                ));

                self.skip_whitespace();
                let expression = self.parse_expression()?;
                self.skip_whitespace();
                self.expect('}')?;
                flush(SequenceValue::ExpressionTag(ExpressionTag {
                    span: Span::new(tag_start, self.offset),
                    expression,
                }));

                text_start = self.offset;
            } else {
                self.next();
            }
        }

        Err(self.error(ParserErrorKind::UnexpectedEOF))
    }

    fn peek_closing_tag_name(&self) -> Option<&'a str> {
        REGEX_CLOSING_TAG
            .captures(&self.remain())
            .and_then(|caps| caps.get(1).map(|mat| mat.as_str()))
    }
}
