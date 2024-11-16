use std::{collections::HashSet, sync::LazyLock};

use ast::{Fragment, Root, Script, SpanOffset, StyleSheet};
use context::Context;
use error::{ParserError, ParserErrorKind};
use oxc_allocator::Allocator;
use oxc_ast::{
    ast::{Expression, Program},
    VisitMut,
};
use oxc_parser::Parser as OxcParser;
use oxc_span::{GetSpan, SourceType, Span};
use oxc_syntax::identifier::is_identifier_name;
use regex::Regex;
use regex_pattern::REGEX_NON_WHITESPACE;

mod ast;
mod context;
mod error;
mod regex_pattern;

static REGEX_LANG_ATTRIBUTE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<script\s+.*?lang="ts".*?\s*>"#).unwrap());
static REGEX_START_WHOLE_COMMENT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(^<!--.*?-->)|(^\/\*.*?\*\/)"#).unwrap());

pub struct Parser<'a> {
    source: &'a str,
    offset: u32,
    allocator: &'a Allocator,
    source_type: SourceType,
    instance: Option<Script<'a>>,
    module: Option<Script<'a>>,
    css: Option<StyleSheet<'a>>,
    context_stack: Vec<Context>,
    meta_tags: HashSet<&'a str>,
    pub fragments: Vec<Fragment<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, allocator: &'a Allocator) -> Self {
        let source = source.trim_end();
        let offset = 0;
        let fragments = vec![Fragment::new()];
        let source_type = if REGEX_LANG_ATTRIBUTE.is_match(source) {
            SourceType::ts()
        } else {
            SourceType::mjs()
        };

        Self {
            source,
            offset,
            fragments,
            allocator,
            source_type,
            instance: None,
            module: None,
            css: None,
            meta_tags: HashSet::new(),
            context_stack: vec![],
        }
    }

    fn offset_u(&self) -> usize {
        self.offset as usize
    }

    pub fn parse(&mut self) -> Result<Root<'a>, ParserError> {
        self.context_stack.push(Context::root_context());
        let fragment = self.parse_fragment()?;
        let start = fragment.nodes.iter().next().map_or(0, |node| {
            let mut start = node.span().start;
            let mut chars = self.source[start as usize..].chars();
            while chars.next().map_or(false, char::is_whitespace) {
                start += 1;
            }
            start
        });
        let end = fragment.nodes.iter().last().map_or(0, |node| {
            let mut end = node.span().end;
            if end == 0 {
                return end;
            }
            let mut chars = self.source[..end as usize].chars().rev();
            while chars.next().map_or(false, char::is_whitespace) && end > 0 {
                end -= 1;
            }
            end
        });
        self.context_stack.pop();
        Ok(Root {
            span: Span::new(start, end),
            fragment,
            instance: self.instance.take(),
            module: self.module.take(),
            css: self.css.take(),
        })
    }

    pub fn parse_expression(&mut self) -> Result<Expression<'a>, ParserError> {
        let mut expr = OxcParser::new(&self.allocator, &self.remain(), self.source_type)
            .parse_expression()
            .map_err(|e| {
                ParserError::new(
                    Span::empty(self.offset),
                    ParserErrorKind::ParseExpression(e),
                )
            })?;
        let mut span_offset = SpanOffset(self.offset);
        span_offset.visit_expression(&mut expr);
        let end = expr.span().end;
        self.offset = end;

        Ok(expr)
    }

    fn parse_expression_in(&mut self, text: &'a str) -> Result<Expression<'a>, ParserError> {
        let mut expr = OxcParser::new(&self.allocator, text, self.source_type)
            .parse_expression()
            .map_err(|e| {
                ParserError::new(
                    Span::empty(self.offset),
                    ParserErrorKind::ParseExpression(e),
                )
            })?;
        let mut span_offset = SpanOffset(self.offset);
        span_offset.visit_expression(&mut expr);
        let end = expr.span().end;
        self.offset = end;

        Ok(expr)
    }

    fn parse_program(&self, data: &'a str, start: u32) -> Result<Program<'a>, ParserError> {
        let parser_return = OxcParser::new(&self.allocator, &data, self.source_type).parse();
        if parser_return.errors.len() != 0 {
            return Err(ParserError::new(
                Span::new(start, self.offset),
                ParserErrorKind::ParseProgram(parser_return.errors),
            ));
        }
        let mut program = parser_return.program;
        let mut span_offset = SpanOffset(start);
        span_offset.visit_program(&mut program);
        Ok(program)
    }

    fn meta_tag_exist(&self, meta_tag: &'a str) -> bool {
        self.meta_tags.contains(meta_tag)
    }

    pub fn remain(&self) -> &'a str {
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
                    Err(ParserError::new(
                        Span::empty(self.offset),
                        ParserErrorKind::ExpectedChar { expected, found: c },
                    ))
                }
            }
            None => Err(ParserError::new(
                Span::empty(self.offset),
                ParserErrorKind::UnexpectedEOFWithChar(expected),
            )),
        }
    }

    fn expect_str(&mut self, s: &'a str) -> Result<&'a str, ParserError> {
        if self.eat_str(s) {
            Ok(s)
        } else {
            Err(ParserError::new(
                Span::empty(self.offset),
                ParserErrorKind::ExpectedStr(s.to_string()),
            ))
        }
    }

    fn expect_regex(&mut self, re: &regex::Regex) -> Result<&'a str, ParserError> {
        if let Some(mat) = re.find(&self.remain()) {
            self.offset += mat.len() as u32;
            Ok(mat.as_str())
        } else {
            Err(ParserError::new(
                Span::empty(self.offset),
                ParserErrorKind::ExpectedStr(re.to_string()),
            ))
        }
    }

    fn peek(&self) -> Option<char> {
        let Self { source, .. } = self;
        if self.offset_u() < source.len() {
            self.source[self.offset_u()..].chars().next()
        } else {
            None
        }
    }

    fn eat(&mut self, ch: char) -> bool {
        match &self.remain().chars().next() {
            Some(c) if *c == ch => {
                self.offset += 1;
                true
            }
            _ => false,
        }
    }

    fn eat_until(&mut self, re: &regex::Regex) -> &'a str {
        if let Some(mat) = re.find(self.remain()) {
            let end = mat.start();
            let result = &self.remain()[..end];
            self.offset += end as u32;
            result
        } else {
            ""
        }
    }

    fn eat_str(&mut self, s: &'a str) -> bool {
        let end = self.offset_u() + s.len();
        if end > self.source.len() {
            return false;
        }

        if &self.source[self.offset_u()..end] == s {
            self.offset = end as u32;
            true
        } else {
            false
        }
    }

    fn eat_identifier(&mut self) -> Result<(&'a str, Expression<'a>), ParserError> {
        let mut i = 1;
        let remain = self.remain();
        while i < remain.len() && is_identifier_name(&remain[..i]) {
            i += 1;
        }

        let name = &remain[..i - 1];
        if name == "" {
            return Err(ParserError::new(
                Span::empty(self.offset),
                ParserErrorKind::AttributeEmptyShorthand,
            ));
        }
        // TODO: handle unexpected_reserved_word
        let expr = self.parse_expression_in(name)?;
        Ok((name, expr))
    }

    fn eat_regex(&mut self, re: &regex::Regex) -> Option<&'a str> {
        let value = re.find(&self.remain()).map(|mat| mat.as_str())?;
        self.offset += value.len() as u32;
        Some(value)
    }

    fn match_ch(&self, ch: char) -> bool {
        self.peek().map_or(false, |c| c == ch)
    }

    fn match_str(&self, s: &str) -> bool {
        let len = s.len();
        let end = if self.offset_u() + len > self.source.len() {
            self.source.len()
        } else {
            self.offset_u() + len
        };

        &self.source[self.offset_u()..end] == s
    }

    fn match_regex(&self, re: &regex::Regex) -> Option<&'a str> {
        re.find(&self.remain()).map(|mat| mat.as_str())
    }

    fn skip_whitespace(&mut self) {
        if let Some(mat) = REGEX_NON_WHITESPACE.find(&self.remain()) {
            self.offset += mat.start() as u32;
        }
    }

    fn skip_comment_or_whitespace(&mut self) {
        self.skip_whitespace();
        while let Some(s) = self.match_regex(&REGEX_START_WHOLE_COMMENT) {
            self.offset += s.len() as u32;
            self.skip_whitespace();
        }
    }
}