use oxc_span::Span;

use crate::{ast::Fragment, Parser};

#[derive(Debug)]
pub enum Element<'a> {
    RegularElement(RegularElement<'a>),
}

#[derive(Debug)]
pub struct RegularElement<'a> {
    pub span: Span,
    pub name: &'a str,
    // TODO:
    // pub attributes: Vec<Attribute>,
    pub fragment: Fragment<'a>,
}

impl<'a> Parser<'a> {
    // now can only parse regular element
    pub fn parse_element(&mut self) -> Element<'a> {
        let start = self.offset;
        self.next();
        let name = self.eat_until(|ch| ch.is_whitespace() || matches!(ch, '/' | '>'));

        // TODO: attributes
        self.eat_until(|ch| matches!(ch, '>' | '/'));
        match self.next() {
            Some(ch) if ch == '/' => {
                self.expect('>').unwrap();
                let span = Span::new(start, self.offset);
                let fragment = Fragment { nodes: vec![] };
                return Element::RegularElement(RegularElement {
                    span,
                    name,
                    fragment,
                });
            }
            _ => (),
        };

        let fragment = self.parse_fragment();
        self.eat_until(|ch| ch == '>');
        self.expect('>').unwrap();
        let element = RegularElement {
            fragment,
            name,
            span: Span::new(start, self.offset),
        };

        Element::RegularElement(element)
    }
}
