use derive_macro::{AstTree, OxcSpan};
use oxc_span::Span;

use crate::{
    ast::Fragment,
    error::ParserError,
    regex_pattern::{
        REGEX_CLOSING_COMMENT, REGEX_LESS_THEN, REGEX_WHITESPACE_OR_SLASH_OR_CLOSING_TAG,
    },
    Meta, Parser,
};

use super::{attribute::Attribute, Script};

#[derive(Debug, AstTree, OxcSpan)]
pub enum Element<'a> {
    RegularElement(RegularElement<'a>),
    Comment(Comment<'a>),
    Script(Script<'a>),
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

        if self.eat_str("!--") {
            let data = self.eat_until(&REGEX_CLOSING_COMMENT);
            self.expect_str("-->")?;

            return Ok(Element::Comment(Comment {
                span: Span::new(start, self.offset),
                data,
            }));
        }

        let name = self.eat_until(&REGEX_WHITESPACE_OR_SLASH_OR_CLOSING_TAG);
        self.skip_whitespace();
        let is_root_script = self.meta().is_parent_root && name == "script";
        let attributes = self.parse_attributes(is_root_script)?;

        if is_root_script {
            self.expect('>')?;
            let script = self.parse_script(start, attributes)?;

            return Ok(Element::Script(script));
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
        self.meta_stack.push(Meta {
            is_parent_root: false,
        });
        let fragment = self.parse_fragment()?;
        self.meta_stack.pop();
        self.eat_until(&REGEX_LESS_THEN);
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
