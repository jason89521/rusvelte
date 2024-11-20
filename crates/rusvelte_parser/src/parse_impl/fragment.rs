use crate::{Parser, ParserError, ParserErrorKind};
use rusvelte_ast::ast::{Fragment, FragmentNode, ScriptContext};

use super::element::ParseElementReturn;

enum ParseFragmentNodeReturn<'a> {
    Node(FragmentNode<'a>),
    ClosePrev,
    None,
}

impl<'a> Parser<'a> {
    pub fn parse_fragment(&mut self) -> Result<Fragment<'a>, ParserError> {
        let mut nodes = vec![];
        while self.offset_u() < self.source.len() && !self.match_str("</") {
            match self.parse_fragment_node()? {
                ParseFragmentNodeReturn::Node(node) => {
                    nodes.push(node);
                }
                ParseFragmentNodeReturn::ClosePrev => return Ok(Fragment { nodes }),
                ParseFragmentNodeReturn::None => (),
            }
        }
        Ok(Fragment { nodes })
    }

    fn parse_fragment_node(&mut self) -> Result<ParseFragmentNodeReturn<'a>, ParserError> {
        let node = if self.match_str("<") {
            let parse_element_return = self.parse_element()?;
            match parse_element_return {
                ParseElementReturn::Element(element) => FragmentNode::Element(Box::new(element)),
                ParseElementReturn::Comment(comment) => FragmentNode::Comment(comment),
                ParseElementReturn::Script(script) => {
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
                ParseElementReturn::ClosePrev => return Ok(ParseFragmentNodeReturn::ClosePrev),
            }
        } else if self.match_str("{") {
            FragmentNode::Tag(self.parse_tag()?)
        } else {
            FragmentNode::Text(self.parse_text())
        };

        Ok(ParseFragmentNodeReturn::Node(node))
    }
}
