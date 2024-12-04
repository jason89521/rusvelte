use std::{collections::HashSet, ops::Deref, sync::LazyLock};

use oxc_allocator::{Allocator, CloneIn, Vec};
use oxc_span::Span;
use regex::Regex;
use rusvelte_utils::{html_tree_validation::closing_tag_omitted, void_element::is_void};

use crate::{
    constants::{
        SVELTE_BODY_TAG, SVELTE_COMPONENT_TAG, SVELTE_DOCUMENT_TAG, SVELTE_ELEMENT_TAG,
        SVELTE_FRAGMENT_TAG, SVELTE_HEAD_TAG, SVELTE_OPTIONS_TAG, SVELTE_SELF_TAG,
        SVELTE_WINDOW_TAG,
    },
    error::{ParserError, ParserErrorKind},
    regex_pattern::{
        REGEX_CLOSING_COMMENT, REGEX_VALID_COMPONENT_NAME, REGEX_WHITESPACE_OR_SLASH_OR_CLOSING_TAG,
    },
    Context, LastAutoClosedTag, Parser,
};

use rusvelte_ast::ast::{
    Attribute, Comment, Component, Element, ExpressionTag, Fragment, RegularElement, Script,
    SlotElement, StyleSheet, SvelteBody, SvelteComponent, SvelteDocument, SvelteElement,
    SvelteFragment, SvelteHead, SvelteSelf, SvelteWindow, Text, TitleElement,
};

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
    /// Encounter auto closed element
    ClosePrev,
    SvelteOptions {
        span: Span,
        fragment: Fragment<'a>,
        attributes: Vec<'a, Attribute<'a>>,
    },
}

impl<'a> ParseElementReturn<'a> {
    fn regular_element(
        span: Span,
        name: &'a str,
        attributes: Vec<'a, Attribute<'a>>,
        fragment: Fragment<'a>,
    ) -> ParseElementReturn<'a> {
        ParseElementReturn::Element(Element::RegularElement(RegularElement {
            span,
            name,
            attributes,
            fragment,
        }))
    }

    fn element(
        allocator: &'a Allocator,
        span: Span,
        name: &'a str,
        mut attributes: Vec<'a, Attribute<'a>>,
        fragment: Fragment<'a>,
    ) -> Result<ParseElementReturn<'a>, ParserError> {
        let clone_this_expression = |attributes: &mut Vec<'a, Attribute<'a>>| {
            let (missing_this, invalid_this) = if name == SVELTE_COMPONENT_TAG {
                (
                    ParserErrorKind::SvelteComponentMissingThis,
                    ParserErrorKind::SvelteComponentInvalidThis,
                )
            } else {
                (
                    ParserErrorKind::SvelteElementMissingThis,
                    ParserErrorKind::SvelteElementInvalidThis,
                )
            };

            let index = attributes
                .iter()
                .position(|attr| {
                    matches!(attr, Attribute::NormalAttribute(_)) && attr.name() == "this"
                })
                .ok_or(ParserError {
                    kind: missing_this,
                    span,
                })?;
            let attr = attributes.remove(index);
            let expression = attr
                .get_expression_tag()
                .ok_or(ParserError {
                    kind: invalid_this,
                    span,
                })?
                .expression
                .clone_in(allocator);

            Ok(expression)
        };
        let element = match name {
            SVELTE_COMPONENT_TAG => Element::SvelteComponent(SvelteComponent {
                span,
                name,
                expression: clone_this_expression(&mut attributes)?,
                attributes,
                fragment,
            }),
            SVELTE_ELEMENT_TAG => Element::SvelteElement(SvelteElement {
                span,
                name,
                tag: clone_this_expression(&mut attributes)?,
                attributes,
                fragment,
            }),
            SVELTE_HEAD_TAG => Element::SvelteHead(SvelteHead {
                span,
                name,
                attributes,
                fragment,
            }),
            // special case, the options element should be remove
            SVELTE_OPTIONS_TAG => {
                return Ok(ParseElementReturn::SvelteOptions {
                    span,
                    attributes,
                    fragment,
                })
            }
            SVELTE_WINDOW_TAG => Element::SvelteWindow(SvelteWindow {
                span,
                name,
                attributes,
                fragment,
            }),
            SVELTE_DOCUMENT_TAG => Element::SvelteDocument(SvelteDocument {
                span,
                name,
                attributes,
                fragment,
            }),
            SVELTE_BODY_TAG => Element::SvelteBody(SvelteBody {
                span,
                name,
                attributes,
                fragment,
            }),
            SVELTE_SELF_TAG => Element::SvelteSelf(SvelteSelf {
                span,
                name,
                attributes,
                fragment,
            }),
            SVELTE_FRAGMENT_TAG => Element::SvelteFragment(SvelteFragment {
                span,
                name,
                attributes,
                fragment,
            }),
            _ if REGEX_VALID_COMPONENT_NAME.is_match(name) => Element::Component(Component {
                span,
                name,
                attributes,
                fragment,
                dynamic: false,
            }),
            "title" => Element::TitleElement(TitleElement {
                span,
                name,
                attributes,
                fragment,
            }),
            "slot" => Element::SlotElement(SlotElement {
                span,
                name,
                attributes,
                fragment,
            }),
            _ => Element::RegularElement(RegularElement {
                span,
                name,
                attributes,
                fragment,
            }),
        };

        Ok(ParseElementReturn::Element(element))
    }
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

        if self.is_parent_regular_element() && closing_tag_omitted(self.parent_name(), name) {
            self.close_parent();
            self.last_auto_closed_tag = Some(LastAutoClosedTag {
                tag: self.parent_name(),
                reason: name,
                depth: self.context_stack.len() as u8,
            });
            // rewind
            self.offset = start;

            return Ok(ParseElementReturn::ClosePrev);
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

        // self closed element or void element
        if self.eat('/') || is_void(name) {
            self.expect('>')?;
            let span = Span::new(start, self.offset);
            let fragment = self.ast.fragment(self.ast.vec([]));
            return ParseElementReturn::element(self.allocator, span, name, attributes, fragment);
        }

        self.expect('>')?;
        self.push_context(Context::element_context(name));
        let fragment = self.parse_fragment()?;

        let ctx = self.pop_context().expect("Expected self context");
        if ctx.auto_closed() {
            return Ok(ParseElementReturn::regular_element(
                Span::new(start, self.offset),
                name,
                attributes,
                fragment,
            ));
        }

        let closing_tag_name = self
            .peek_closing_tag_name()
            .ok_or(self.error(ParserErrorKind::ExpectedClosingTag))?;
        if closing_tag_name != name {
            // close any elements that don't have their own closing tags, e.g. <div><p></div>
            if !self.is_parent_regular_element() {
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

            return Ok(ParseElementReturn::regular_element(
                Span::new(start, self.offset),
                name,
                attributes,
                fragment,
            ));
        }

        self.expect_regex(&REGEX_CLOSING_TAG)?;

        ParseElementReturn::element(
            self.allocator,
            Span::new(start, self.offset),
            name,
            attributes,
            fragment,
        )
    }

    /// Used for parse attribute value and textarea's nodes
    pub(crate) fn parse_sequence<F>(
        &mut self,
        done: F,
        location: &'static str,
    ) -> Result<Vec<'a, SequenceValue<'a>>, ParserError>
    where
        F: Fn(&Self) -> bool,
    {
        let mut text_start = self.offset;

        let mut values = self.ast.vec([]);
        let mut flush = |value: SequenceValue<'a>| {
            match &value {
                SequenceValue::Text(text) if text.raw.is_empty() => (),
                _ => values.push(value),
            };
        };

        while self.peek().is_some() {
            if done(self) {
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
            .captures(self.remain())
            .and_then(|caps| caps.get(1).map(|mat| mat.as_str()))
    }
}
