use derive_macro::{AstTree, OxcSpan};

use super::{Comment, Element, ParseElementReturn, ScriptContext, Tag, Text};
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

enum ParseFragmentNodeReturn<'a> {
    Nodes(Vec<FragmentNode<'a>>),
    Node(FragmentNode<'a>),
    None,
}

#[derive(Debug, AstTree, OxcSpan)]
pub enum FragmentNode<'a> {
    Text(Text<'a>),
    Element(Box<Element<'a>>),
    Tag(Tag<'a>),
    Comment(Comment<'a>),
}

impl<'a> Parser<'a> {
    pub fn parse_fragment(&mut self) -> Result<Fragment<'a>, ParserError> {
        let mut result = vec![];
        while self.offset_u() < self.source.len() && !self.match_str("</") {
            match self.parse_fragment_node()? {
                ParseFragmentNodeReturn::Nodes(mut nodes) => {
                    result.append(&mut nodes);
                }
                ParseFragmentNodeReturn::Node(node) => {
                    result.push(node);
                }
                ParseFragmentNodeReturn::None => (),
            }
        }
        Ok(Fragment { nodes: result })
    }

    fn parse_fragment_node(&mut self) -> Result<ParseFragmentNodeReturn<'a>, ParserError> {
        let node = if self.match_str("<") {
            let parse_element_return = self.parse_element()?;
            match parse_element_return {
                ParseElementReturn::Element(element) => FragmentNode::Element(Box::new(element)),
                ParseElementReturn::Comment(comment) => FragmentNode::Comment(comment),
                ParseElementReturn::Script(script) => {
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
                    return Ok(ParseFragmentNodeReturn::None);
                }
                ParseElementReturn::StyleSheet(style_sheet) => {
                    if self.css.is_some() {
                        return Err(ParserError::new(
                            style_sheet.span,
                            ParserErrorKind::StyleDuplicate,
                        ));
                    }
                    self.css = Some(style_sheet);
                    return Ok(ParseFragmentNodeReturn::None);
                }
                ParseElementReturn::Nodes(nodes) => {
                    return Ok(ParseFragmentNodeReturn::Nodes(nodes))
                }
            }
        } else if self.match_str("{") {
            FragmentNode::Tag(self.parse_tag()?)
        } else {
            FragmentNode::Text(self.parse_text())
        };

        Ok(ParseFragmentNodeReturn::Node(node))
    }
}
