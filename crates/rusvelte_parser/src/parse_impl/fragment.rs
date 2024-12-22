use std::sync::LazyLock;

use crate::{constants::SVELTE_OPTIONS_TAG, Parser, ParserError, ParserErrorKind};
use oxc_allocator::CloneIn;
use oxc_ast::ast::{Expression, ObjectPropertyKind};
use oxc_span::{GetSpan, Span};
use regex::Regex;
use rusvelte_ast::ast::{
    Attribute, Comment, CustomElement, Fragment, FragmentNode, Script, ScriptContext, StyleSheet,
};
use rusvelte_utils::{
    constants::{NAMESPACE_MATHML, NAMESPACE_SVG},
    special_element::disallow_children,
};

use super::element::ParseElementReturn;

static REGEX_VALID_TAG_NAME: LazyLock<Regex> = LazyLock::new(|| {
    let tag_name_char = r"[a-z0-9_.\xB7\xC0-\xD6\xD8-\xF6\xF8-\u037D\u037F-\u1FFF\u200C-\u200D\u203F-\u2040\u2070-\u218F\u2C00-\u2FEF\u3001-\uD7FF\uF900-\uFDCF\uFDF0-\uFFFD\u{10000}-\u{EFFFF}-]";
    Regex::new(&format!("^[a-z]{tag_name_char}*-{tag_name_char}*$")).unwrap()
});

const RESERVED_TAG_NAMES: [&str; 8] = [
    "annotation-xml",
    "color-profile",
    "font-face",
    "font-face-src",
    "font-face-uri",
    "font-face-format",
    "font-face-name",
    "missing-glyph",
];

enum ParseFragmentNodeReturn<'a> {
    Node(FragmentNode<'a>),
    ClosePrev,
    /// Encounter a next/close block notation like `{:else }` or `{/if}`
    NextOrCloseBlock,
    Script(Script<'a>),
    StyleSheet(StyleSheet<'a>),
    SvelteOptions,
}

impl<'a> Parser<'a> {
    pub fn parse_fragment(&mut self, transparent: bool) -> Result<Fragment<'a>, ParserError> {
        let mut nodes = self.ast.vec([]);
        while self.offset_usize() < self.source.len() && !self.match_str("</") {
            match self.parse_fragment_node()? {
                ParseFragmentNodeReturn::Node(node) => {
                    nodes.push(node);
                }
                ParseFragmentNodeReturn::ClosePrev | ParseFragmentNodeReturn::NextOrCloseBlock => {
                    return Ok(self.ast.fragment(nodes, transparent))
                }
                ParseFragmentNodeReturn::Script(mut script) => {
                    script.leading_comment = self.find_leading_comment(&nodes);
                    match script.context {
                        ScriptContext::Default => {
                            if self.instance.is_some() {
                                return Err(ParserError::new(
                                    script.span,
                                    ParserErrorKind::ScriptDuplicate,
                                ));
                            }
                            self.instance = Some(script)
                        }
                        ScriptContext::Module => {
                            if self.module.is_some() {
                                return Err(ParserError::new(
                                    script.span,
                                    ParserErrorKind::ScriptDuplicate,
                                ));
                            }
                            self.module = Some(script)
                        }
                    }
                }
                ParseFragmentNodeReturn::StyleSheet(mut style_sheet) => {
                    if self.css.is_some() {
                        return Err(ParserError::new(
                            style_sheet.span,
                            ParserErrorKind::StyleDuplicate,
                        ));
                    }
                    style_sheet.content.comment = self.find_leading_comment(&nodes);
                    self.css = Some(style_sheet);
                }
                ParseFragmentNodeReturn::SvelteOptions => (),
            }
        }
        Ok(self.ast.fragment(nodes, transparent))
    }

    fn parse_fragment_node(&mut self) -> Result<ParseFragmentNodeReturn<'a>, ParserError> {
        let node = if self.match_str("<") {
            let parse_element_return = self.parse_element()?;
            match parse_element_return {
                ParseElementReturn::Element(element) => FragmentNode::Element(Box::new(element)),
                ParseElementReturn::Comment(comment) => FragmentNode::Comment(comment),
                ParseElementReturn::Script(script) => {
                    return Ok(ParseFragmentNodeReturn::Script(script))
                }
                ParseElementReturn::StyleSheet(style_sheet) => {
                    return Ok(ParseFragmentNodeReturn::StyleSheet(style_sheet))
                }
                ParseElementReturn::ClosePrev => return Ok(ParseFragmentNodeReturn::ClosePrev),
                ParseElementReturn::SvelteOptions {
                    span,
                    attributes,
                    fragment,
                } => {
                    let mut options = self.ast.svelte_options(span);
                    for attr in attributes.iter() {
                        let span = attr.span();
                        let attr = if let Attribute::NormalAttribute(attr) = attr {
                            attr
                        } else {
                            return Err(ParserError {
                                span,
                                kind: ParserErrorKind::SvelteOptionsInvalidAttribute,
                            });
                        };
                        match attr.name {
                            "runes" => options.runes = Some(attr.value.get_boolean_value()),
                            "tag" => {
                                return Err(ParserError {
                                    span,
                                    kind: ParserErrorKind::SvelteOptionsDeprecatedTag,
                                })
                            }
                            "customElement" => {
                                let mut custom_element = CustomElement::default();
                                if let Some(tag) = attr.value.as_raw_text() {
                                    validate_tag(span, tag)?;
                                    custom_element.tag = Some(tag);
                                    options.custom_element = Some(custom_element);
                                    continue;
                                }
                                let svelte_options_invalid_custom_element = ParserError {
                                    span,
                                    kind: ParserErrorKind::SvelteOptionsInvalidCustomElement,
                                };
                                let expr = attr
                                    .value
                                    .as_expression_tag()
                                    .map(|expr_tag| &expr_tag.expression)
                                    .ok_or(svelte_options_invalid_custom_element.clone())?;
                                let value = if let Expression::ObjectExpression(expr) = expr {
                                    expr
                                } else {
                                    return Err(svelte_options_invalid_custom_element.clone());
                                };
                                for prop in value.properties.iter() {
                                    let prop = match prop {
                                        ObjectPropertyKind::ObjectProperty(prop)
                                            if !prop.computed
                                                && prop.key.is_identifier()
                                                && !prop.kind.is_accessor() =>
                                        {
                                            prop
                                        }
                                        _ => {
                                            return Err(
                                                svelte_options_invalid_custom_element.clone()
                                            )
                                        }
                                    };

                                    let prop_name = if let Some(name) = prop.key.name() {
                                        name
                                    } else {
                                        return Err(svelte_options_invalid_custom_element);
                                    };

                                    match prop_name.as_ref() {
                                        "tag" => {
                                            if let Expression::StringLiteral(s) = &prop.value {
                                                validate_tag(span, s.value.as_str())?;
                                                custom_element.tag = Some(s.value.as_str());
                                            } else {
                                                return Err(ParserError {
                                                    span,
                                                    kind:
                                                        ParserErrorKind::SvelteOptionsInvalidTagName,
                                                });
                                            }
                                        }
                                        "props" => {
                                            if let Expression::ObjectExpression(expr) = &prop.value
                                            {
                                                // TODO: should check whether the key and value is valid
                                                custom_element.props =
                                                    Some(expr.as_ref().clone_in(self.allocator));
                                            } else {
                                                return Err(
                                                    ParserError {
                                                        span, kind: ParserErrorKind::SvelteOptionsInvalidCustomElementProps
                                                    },
                                                );
                                            };
                                        }
                                        "shadow" => {
                                            match &prop.value {
                                                Expression::StringLiteral(s) if s.value == "open" || s.value == "none" => {
                                                    custom_element.shadow = Some(s.value.as_str());
                                                }
                                                _ => return Err(ParserError {
                                                    span, kind:ParserErrorKind::SvelteOptionsInvalidCustomElementShadow
                                                })
                                            }
                                        }
                                        "extend" => {
                                            custom_element.extend =
                                                Some(prop.value.clone_in(self.allocator))
                                        }
                                        _ => (),
                                    }
                                }

                                options.custom_element = Some(custom_element)
                            }
                            "namespace" => {
                                let err = ParserError {
                                    span,
                                    kind: ParserErrorKind::SvelteOptionsInvalidAttributeValue(
                                        r#""html", "mathml" or "svg""#.to_string(),
                                    ),
                                };
                                let value = attr.value.get_static_value().ok_or(err.clone())?;
                                if value == NAMESPACE_SVG {
                                    options.namespace = Some("svg");
                                } else if value == NAMESPACE_MATHML {
                                    options.namespace = Some("mathml");
                                } else if value == "html" || value == "mathml" || value == "svg" {
                                    options.namespace = Some(value);
                                } else {
                                    return Err(err);
                                }
                            }
                            "css" => match attr.value.get_static_value() {
                                Some(value) if value == "injected" => options.css = Some(value),
                                _ => {
                                    return Err(ParserError {
                                        span,
                                        kind: ParserErrorKind::SvelteOptionsInvalidAttributeValue(
                                            "Injected".to_string(),
                                        ),
                                    })
                                }
                            },
                            "immutable" => {
                                options.immutable = if attr.value.get_static_value().is_some() {
                                    Some(attr.value.get_boolean_value())
                                } else {
                                    None
                                }
                            }
                            "preserveWhitespace" => {
                                options.preserve_whitespace =
                                    if attr.value.get_static_value().is_some() {
                                        Some(attr.value.get_boolean_value())
                                    } else {
                                        None
                                    }
                            }
                            "accessors" => {
                                options.accessors = if attr.value.get_static_value().is_some() {
                                    Some(attr.value.get_boolean_value())
                                } else {
                                    None
                                }
                            }
                            name => {
                                return Err(ParserError {
                                    span,
                                    kind: ParserErrorKind::SvelteOptionsUnknownAttribute(
                                        name.to_string(),
                                    ),
                                })
                            }
                        }
                    }

                    if let Some(span) = disallow_children(&fragment) {
                        return Err(ParserError {
                            span,
                            kind: ParserErrorKind::SvelteMetaInvalidContent(
                                SVELTE_OPTIONS_TAG.to_string(),
                            ),
                        });
                    }

                    options.attributes = attributes;
                    self.options = Some(options);

                    return Ok(ParseFragmentNodeReturn::SvelteOptions);
                }
            }
        } else if self.match_ch('{') {
            let start = self.offset;
            self.expect('{')?;
            self.skip_whitespace();

            match self
                .peek()
                .ok_or(self.error(ParserErrorKind::UnexpectedEOF))?
            {
                '#' => FragmentNode::Block(self.parse_block(start)?),
                ':' | '/' => {
                    // rewind
                    self.offset = start;
                    return Ok(ParseFragmentNodeReturn::NextOrCloseBlock);
                }
                _ => FragmentNode::Tag(self.parse_tag(start)?),
            }
        } else {
            FragmentNode::Text(self.parse_text())
        };

        Ok(ParseFragmentNodeReturn::Node(node))
    }

    fn find_leading_comment(&mut self, nodes: &[FragmentNode<'a>]) -> Option<Comment<'a>> {
        for node in nodes.iter().rev() {
            match node {
                FragmentNode::Comment(comment) => return Some(*comment),
                FragmentNode::Text(text) if !text.raw.is_empty() => (),
                _ => break,
            }
        }
        None
    }
}

fn validate_tag(span: Span, tag: &str) -> Result<(), ParserError> {
    if !REGEX_VALID_TAG_NAME.is_match(tag) {
        Err(ParserError {
            span,
            kind: ParserErrorKind::SvelteOptionsInvalidTagName,
        })
    } else if RESERVED_TAG_NAMES.contains(&tag) {
        Err(ParserError {
            span,
            kind: ParserErrorKind::SvelteOptionsReservedTagName,
        })
    } else {
        Ok(())
    }
}
