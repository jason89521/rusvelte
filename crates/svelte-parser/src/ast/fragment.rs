use derive_macro::{AstTree, OxcSpan};

use super::{Element, ScriptContext, Tag, Text};
use crate::{Parser, ParserError, ParserErrorKind};

#[derive(Debug, AstTree)]
pub struct Fragment<'a> {
    pub nodes: Vec<FragmentNode<'a>>,
}

impl<'a> Fragment<'a> {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }
}

#[derive(Debug, AstTree, OxcSpan)]
pub enum FragmentNode<'a> {
    Text(Text<'a>),
    Element(Box<Element<'a>>),
    Tag(Tag<'a>),
}

impl<'a> Parser<'a> {
    pub fn parse_fragment(&mut self) -> Result<Fragment<'a>, ParserError> {
        let mut result = vec![];
        while self.offset_u() < self.source.len() && !self.match_str("</") {
            if let Some(node) = self.parse_fragment_node()? {
                result.push(node)
            }
        }
        Ok(Fragment { nodes: result })
    }

    pub fn parse_fragment_node(&mut self) -> Result<Option<FragmentNode<'a>>, ParserError> {
        let node = if self.match_str("<") {
            let element = self.parse_element()?;
            if self.is_parent_root() {
                if let Element::Script(script) = element {
                    match &script.context {
                        &ScriptContext::Default => {
                            if self.instance.is_some() {
                                return Err(ParserError::new(
                                    script.span,
                                    ParserErrorKind::ScriptDuplicate,
                                ));
                            }
                            self.instance = Some(script)
                        }
                        &ScriptContext::Module => {
                            if self.module.is_some() {
                                return Err(ParserError::new(
                                    script.span,
                                    ParserErrorKind::ScriptDuplicate,
                                ));
                            }
                            self.module = Some(script)
                        }
                    }
                    return Ok(None);
                }
                if let Element::StyleSheet(style_sheet) = element {
                    if self.css.is_some() {
                        return Err(ParserError::new(
                            style_sheet.span,
                            ParserErrorKind::StyleDuplicate,
                        ));
                    }
                    self.css = Some(style_sheet);
                    return Ok(None);
                }
            }
            FragmentNode::Element(Box::new(element))
        } else if self.match_str("{") {
            FragmentNode::Tag(self.parse_tag()?)
        } else {
            FragmentNode::Text(self.parse_text())
        };

        Ok(Some(node))
    }
}
