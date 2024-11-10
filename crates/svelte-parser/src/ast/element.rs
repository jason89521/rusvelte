use oxc_span::Span;

use crate::{ast::Fragment, error::ParserError, Parser};

use std::sync::LazyLock;

use regex::Regex;

static WHITESPACE_OR_SLASH_OR_CLOSING_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\s|\/|>)").unwrap());
static CLOSING_COMMENT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"-->").unwrap());

#[derive(Debug)]
pub enum Element<'a> {
    RegularElement(RegularElement<'a>),
    Comment(Comment<'a>),
}

#[derive(Debug)]
pub struct RegularElement<'a> {
    pub span: Span,
    pub name: &'a str,
    // TODO:
    // pub attributes: Vec<Attribute>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug)]
pub struct Comment<'a> {
    pub span: Span,
    pub data: &'a str,
}

impl<'a> Parser<'a> {
    // now can only parse regular element
    pub fn parse_element(&mut self) -> Result<Element<'a>, ParserError> {
        let start = self.offset;
        self.expect('<')?;

        if self.eat_str("!--").is_some() {
            let data = self.eat_until(&CLOSING_COMMENT).unwrap_or("");
            self.expect_str("-->")?;

            return Ok(Element::Comment(Comment {
                span: Span::new(start, self.offset),
                data,
            }));
        }

        let name = self.eat_until(&WHITESPACE_OR_SLASH_OR_CLOSING_TAG).unwrap();

        // // TODO: attributes, now ignore.
        self.eat_until(&Regex::new(r"(\s|\/|>)").unwrap());
        match self.next() {
            Some(ch) if ch == '/' => {
                self.expect('>').unwrap();
                let span = Span::new(start, self.offset);
                let fragment = Fragment { nodes: vec![] };
                return Ok(Element::RegularElement(RegularElement {
                    span,
                    name,
                    fragment,
                }));
            }
            _ => (),
        };

        let fragment = self.parse_fragment();
        self.eat_until(&Regex::new(r">").unwrap());
        self.expect('>').unwrap();
        let element = RegularElement {
            fragment,
            name,
            span: Span::new(start, self.offset),
        };

        Ok(Element::RegularElement(element))
    }
}
