use oxc_ast::ast::Expression;
use oxc_span::{GetSpan, Span};
use regex::Regex;
use rusvelte_ast::ast::{Block, EachBlock, Fragment, FragmentNode, IfBlock};
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
        } else if self.eat_str("each") {
            Ok(Block::EachBlock(self.parse_each_block(start)?))
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
            is_closed = true;
        }
        let _ctx = self.pop_context().expect("Expected a if context");
        result.span.end = self.offset;

        Ok((is_closed, result))
    }

    fn parse_each_block(&mut self, start: u32) -> Result<EachBlock<'a>, ParserError> {
        self.expect_whitespace()?;
        let expression_start = self.offset;
        let mut text = self.remain();
        let mut expression = self.parse_expression_in(text, expression_start);
        while let Err(e) = expression {
            if let Some(as_idx) = text.rfind("as") {
                text = &text[..as_idx];
                expression = self.parse_expression_in(text, expression_start);
            } else {
                return Err(e);
            }
        }

        let mut expression = expression?;
        self.offset += expression.span().size();
        self.skip_whitespace();
        // this could be a TypeScript assertion that was erroneously eaten.
        if !self.match_str("as") {
            let orig_expr_len = expression.span().size() as usize;
            // {#each todos as todo, id(todo.id)}
            if let Expression::SequenceExpression(mut seq_expr) = expression {
                expression = seq_expr.expressions.swap_remove(0);
            }
            // can has many `as` and parenthesis, so using while here
            // TODO: I don't know whether this a good approach, should research after we complete the transformer
            loop {
                match expression {
                    Expression::TSAsExpression(expr) => expression = expr.unbox().expression,
                    Expression::ParenthesizedExpression(expr) => {
                        expression = expr.unbox().expression
                    }
                    _ => break,
                }
            }

            while let Expression::ParenthesizedExpression(expr) = expression {
                expression = expr.unbox().expression
            }

            self.offset = expression_start
                + text[..orig_expr_len]
                    .rfind("as")
                    .expect("Expected a 'as' token") as u32;
        }

        self.expect_str("as")?;
        self.expect_whitespace()?;
        let context = self.parse_binding_pattern()?;
        self.skip_whitespace();

        let mut index = None;
        if self.eat(',') {
            self.skip_whitespace();
            index = self.eat_identifier()?.map(|(name, _)| name);
            if index.is_none() {
                return Err(self.error(ParserErrorKind::ExpectedIdentifier));
            }
            self.skip_whitespace();
        }

        let key = if self.eat('(') {
            self.skip_whitespace();
            let key = Some(self.parse_expression()?);
            self.skip_whitespace();
            self.expect(')')?;
            self.skip_whitespace();
            key
        } else {
            None
        };

        self.expect('}')?;
        self.push_context(Context::Block { name: "each" });
        let body = self.parse_fragment()?;
        let mut fallback = None;
        if self.eat_regex(&REGEX_START_NEXT_BLOCK).is_some() {
            if !self.eat_str("else") {
                return Err(self.error(ParserErrorKind::ExpectedToken("{:else}".to_string())));
            }
            self.skip_whitespace();
            self.expect('}')?;
            fallback = Some(self.parse_fragment()?);
        }

        self.expect_regex(&REGEX_START_CLOSE_BLOCK)?;
        self.expect_str("each")?;
        self.skip_whitespace();
        self.expect('}')?;

        Ok(EachBlock {
            span: Span::new(start, self.offset),
            expression,
            context,
            body,
            fallback,
            index,
            key,
        })
    }
}
