use oxc_span::Span;
use regex::Regex;
use rusvelte_ast::ast::{Block, Fragment, FragmentNode, IfBlock};
use std::sync::LazyLock;

use crate::{
    context::Context,
    error::{ParserError, ParserErrorKind},
    Parser,
};

static REGEX_START_NEXT_BLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\{\s*:"#).unwrap());
static REGEX_START_CLOSE_BLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\{\s*\/"#).unwrap());

impl<'a> Parser<'a> {
    pub fn parse_block(&mut self, start: u32) -> Result<Block<'a>, ParserError> {
        self.expect('#')?;
        if self.eat_str("if") {
            let (is_closed, if_block) = self.parse_if_block(start)?;
            if is_closed {
                Ok(Block::IfBlock(if_block))
            } else {
                Err(self.error_at(start, ParserErrorKind::BlockUnclosed))
            }
        } else {
            unimplemented!()
        }
    }

    /// return whether the if block if closed and the if block
    fn parse_if_block(&mut self, start: u32) -> Result<(bool, IfBlock<'a>), ParserError> {
        self.expect_whitespace()?;
        let test = self.parse_expression()?;
        self.skip_whitespace();
        self.expect('}')?;

        self.push_context(Context::block_context("if"));
        let consequent = self.parse_fragment()?;
        let mut result = IfBlock {
            span: Span::empty(start),
            elseif: false,
            test,
            consequent,
            alternate: None,
        };
        let alternate_start = self.offset;
        let mut is_closed = false;
        if self.eat_regex(&REGEX_START_NEXT_BLOCK).is_some() {
            if !self.eat_str("else") {
                return Err(self.error(ParserErrorKind::ExpectedToken(
                    "{:else} or {:else if}".to_string(),
                )));
            }
            if self.eat_str("if") {
                return Err(self.error(ParserErrorKind::BlockInvalidElseif));
            }

            self.skip_whitespace();
            if self.eat_str("if") {
                // :else if
                let (is_closed_by_else, mut alternate) = self.parse_if_block(alternate_start)?;
                is_closed = is_closed_by_else;
                alternate.elseif = true;
                result.alternate = Some(Fragment {
                    nodes: vec![FragmentNode::Block(Block::IfBlock(alternate))],
                });
            } else {
                // :else
                self.skip_whitespace();
                self.expect('}')?;
                // TODO: should push context?
                let alternate = self.parse_fragment()?;
                result.alternate = Some(alternate);
            }
        }
        if self.eat_regex(&REGEX_START_CLOSE_BLOCK).is_some() {
            self.expect_str("if")?;
            self.skip_whitespace();
            self.expect('}')?;
        }
        let _ctx = self.pop_context().expect("Expected a if context");
        result.span.end = self.offset;

        Ok((is_closed, result))
    }
}
