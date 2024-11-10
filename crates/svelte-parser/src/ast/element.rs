use derive_macro::AstTree;
use oxc_span::Span;

use crate::{
    ast::Fragment,
    error::ParserError,
    regex_pattern::{CLOSING_COMMENT, LESS_THEN, WHITESPACE_OR_SLASH_OR_CLOSING_TAG},
    Meta, Parser,
};

use super::attribute::Attribute;

#[derive(Debug, AstTree)]
pub enum Element<'a> {
    RegularElement(RegularElement<'a>),
    Comment(Comment<'a>),
}

#[derive(Debug, AstTree)]
pub struct RegularElement<'a> {
    pub span: Span,
    pub name: &'a str,
    pub attributes: Vec<Attribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug, AstTree)]
pub struct Comment<'a> {
    pub span: Span,
    pub data: &'a str,
}

impl<'a> Parser<'a> {
    // now can only parse regular element
    pub fn parse_element(&mut self, _meta: &Meta) -> Result<Element<'a>, ParserError> {
        let start = self.offset;
        self.expect('<')?;

        if self.eat_str("!--") {
            let data = self.eat_until(&CLOSING_COMMENT);
            self.expect_str("-->")?;

            return Ok(Element::Comment(Comment {
                span: Span::new(start, self.offset),
                data,
            }));
        }

        let name = self.eat_until(&WHITESPACE_OR_SLASH_OR_CLOSING_TAG);
        self.skip_whitespace();
        // TODO: attributes, now ignore.
        let attributes = self.parse_attributes()?;

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
        let fragment = self.parse_fragment(&Meta {
            is_parent_root: false,
        })?;
        self.eat_until(&LESS_THEN);
        self.expect('>').unwrap();
        let element = RegularElement {
            fragment,
            name,
            span: Span::new(start, self.offset),
            attributes,
        };

        Ok(Element::RegularElement(element))
    }
}
