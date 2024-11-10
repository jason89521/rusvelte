use ast::{Fragment, FragmentNode, Root, SpanOffset};
use error::ParserError;
use oxc_allocator::Allocator;
use oxc_ast::{ast::Expression, VisitMut};
use oxc_diagnostics::OxcDiagnostic;
use oxc_parser::Parser as OxcParser;
use oxc_span::{GetSpan, SourceType, Span};

mod ast;
mod error;
mod serialize;

pub struct Parser<'a> {
    source: &'a str,
    offset: u32,
    allocator: &'a Allocator,
    source_type: SourceType,
    pub fragments: Vec<Fragment<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, allocator: &'a Allocator) -> Self {
        let source = source.trim_end();
        let offset = 0;
        let fragments = vec![Fragment::new()];
        Self {
            source,
            offset,
            fragments,
            allocator,
            source_type: SourceType::default(),
        }
    }

    fn offset_u(&self) -> usize {
        self.offset as usize
    }

    pub fn parse(&mut self) -> anyhow::Result<Root<'a>> {
        let start = self.source.find(|c: char| !c.is_whitespace()).unwrap_or(0) as u32;
        Ok(Root {
            css: None,
            span: Span::new(start, self.source.len() as u32),
            fragment: self.parse_fragment(),
        })
    }

    pub fn match_str(&self, s: &str) -> bool {
        let len = s.len();
        let end = if self.offset_u() + len > self.source.len() {
            self.source.len()
        } else {
            self.offset_u() + len
        };

        &self.source[self.offset_u()..end] == s
    }

    pub fn parse_fragment(&mut self) -> Fragment<'a> {
        let mut result = vec![];
        while self.offset_u() < self.source.len() && !self.match_str("</") {
            result.push(
                self.parse_fragment_node()
                    .expect("Failed to parse fragment node"),
            );
        }
        Fragment { nodes: result }
    }

    pub fn parse_fragment_node(&mut self) -> Result<FragmentNode<'a>, ParserError> {
        let node = if self.match_str("<") {
            FragmentNode::Element(Box::new(self.parse_element()?))
        } else if self.match_str("{") {
            FragmentNode::Tag(self.parse_tag()?)
        } else {
            FragmentNode::Text(self.parse_text())
        };

        Ok(node)
    }

    pub fn parse_expression(&mut self) -> Result<Expression<'a>, Vec<OxcDiagnostic>> {
        let mut expr =
            OxcParser::new(&self.allocator, &self.remain(), self.source_type).parse_expression()?;
        let mut span_offset = SpanOffset(self.offset);
        span_offset.visit_expression(&mut expr);
        let end = expr.span().end;
        self.offset = end;

        Ok(expr)
    }

    fn remain(&self) -> &'a str {
        &self.source[self.offset_u()..]
    }

    fn next(&mut self) -> Option<char> {
        if self.offset_u() < self.source.len() {
            let result = self.source[self.offset_u()..].chars().next();
            self.offset += 1;
            result
        } else {
            None
        }
    }

    fn expect(&mut self, expected: char) -> Result<char, ParserError> {
        match self.next() {
            Some(c) => {
                if c == expected {
                    Ok(c)
                } else {
                    Err(ParserError::ExpectChar { expected, found: c })
                }
            }
            None => Err(ParserError::UnexpectedEOF(expected)),
        }
    }

    fn expect_str(&mut self, s: &'a str) -> Result<&'a str, ParserError> {
        self.eat_str(s).ok_or(ParserError::ExpectStr(s.to_string()))
    }

    fn peek(&self) -> Option<char> {
        let Self { source, .. } = self;
        if self.offset_u() < source.len() {
            self.source[self.offset_u()..].chars().next()
        } else {
            None
        }
    }

    fn eat_until(&mut self, re: &regex::Regex) -> Option<&'a str> {
        if let Some(mat) = re.find(self.remain()) {
            let end = mat.start();
            let result = &self.remain()[..end];
            self.offset += end as u32;
            Some(result)
        } else {
            None
        }
    }

    fn eat_str(&mut self, s: &'a str) -> Option<&'a str> {
        let end = self.offset_u() + s.len();
        if end > self.source.len() {
            return None;
        }

        if &self.source[self.offset_u()..end] == s {
            self.offset = end as u32;
            Some(s)
        } else {
            None
        }
    }
}
