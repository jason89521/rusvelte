use oxc_ast::ast::{BindingIdentifier, Expression};
use oxc_span::{GetSpan, Span};
use regex::Regex;
use rusvelte_ast::ast::{
    AwaitBlock, Block, EachBlock, Fragment, FragmentNode, IfBlock, KeyBlock, SnippetBlock,
};
use std::{cell::Cell, sync::LazyLock};

use crate::{
    context::Context,
    error::{ParserError, ParserErrorKind},
    regex_pattern::REGEX_START_WHITESPACE_WITH_CLOSING_CURLY_BRACE,
    Parser,
};

static REGEX_START_NEXT_BLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\{\s*:"#).unwrap());
static REGEX_START_CLOSE_BLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\{\s*\/"#).unwrap());

const IF_STR: &str = "if";
const EACH_STR: &str = "each";
const AWAIT_STR: &str = "await";
const KEY_STR: &str = "key";
const SNIPPET_STR: &str = "snippet";

impl<'a> Parser<'a> {
    pub fn parse_block(&mut self, start: u32) -> Result<Block<'a>, ParserError> {
        self.expect('#')?;
        if self.eat_str(IF_STR) {
            let (is_closed, if_block) = self.parse_if_block(start)?;
            if is_closed {
                Ok(Block::IfBlock(if_block))
            } else {
                Err(self.error_at(start, ParserErrorKind::BlockUnclosed))
            }
        } else if self.eat_str(EACH_STR) {
            Ok(Block::EachBlock(self.parse_each_block(start)?))
        } else if self.eat_str(AWAIT_STR) {
            Ok(Block::AwaitBlock(self.parse_await_block(start)?))
        } else if self.eat_str(KEY_STR) {
            Ok(Block::KeyBlock(self.parse_key_block(start)?))
        } else if self.eat_str(SNIPPET_STR) {
            Ok(Block::SnippetBlock(self.parse_snippet_block(start)?))
        } else {
            Err(self.error(ParserErrorKind::ExpectedBlockType))
        }
    }

    /// return whether the if block if closed and the if block
    fn parse_if_block(&mut self, start: u32) -> Result<(bool, IfBlock<'a>), ParserError> {
        self.expect_whitespace()?;
        let test = self.parse_expression()?;
        self.skip_whitespace();
        self.expect('}')?;

        self.push_context(Context::block_context(IF_STR));
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
            if self.eat_str(IF_STR) {
                return Err(self.error(ParserErrorKind::BlockInvalidElseif));
            }

            self.skip_whitespace();
            if self.eat_str(IF_STR) {
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
            self.expect_str(IF_STR)?;
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
        // we have to do this loop because `{#each x as { y = z }}` fails to parse —
        // the `as { y = z }` is treated as an Expression but it's actually a Pattern.
        // the 'fix' is to backtrack and hide everything from the `as` onwards, until
        // we get a valid expression
        let mut expression = loop {
            match self.parse_expression_in(text, expression_start) {
                Ok(expr) => {
                    break expr;
                }
                Err(e) => {
                    if let Some(as_idx) = text.rfind("as") {
                        text = &text[..as_idx];
                    } else {
                        return Err(e);
                    }
                }
            }
        };

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
        self.push_context(Context::Block { name: EACH_STR });
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

        self.pop_context();
        self.expect_close_block(EACH_STR)?;

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

    fn parse_await_block(&mut self, start: u32) -> Result<AwaitBlock<'a>, ParserError> {
        self.expect_whitespace()?;
        let expression = self.parse_expression()?;
        self.skip_whitespace();

        let mut value = None;
        let mut error = None;
        let mut pending = None;
        let mut then = None;
        let mut catch = None;
        if self.eat_str("then") {
            // {#await expr then}
            if self
                .match_regex(&REGEX_START_WHITESPACE_WITH_CLOSING_CURLY_BRACE)
                .is_some()
            {
                self.skip_whitespace();
            } else {
                // {#await expr then pattern}
                self.expect_whitespace()?;
                value = Some(self.parse_binding_pattern()?);
                self.skip_whitespace();
            }
        } else if self.eat_str("catch") {
            // {#await expr catch}
            if self
                .match_regex(&REGEX_START_WHITESPACE_WITH_CLOSING_CURLY_BRACE)
                .is_some()
            {
                self.skip_whitespace();
            } else {
                // {#await expr catch err}
                self.expect_whitespace()?;
                error = Some(self.parse_binding_pattern()?);
                self.skip_whitespace();
            }
        }

        self.expect('}')?;
        self.push_context(Context::Block { name: AWAIT_STR });
        if value.is_some() {
            then = Some(self.parse_fragment()?);
        } else if error.is_some() {
            catch = Some(self.parse_fragment()?)
        } else {
            pending = Some(self.parse_fragment()?)
        }

        let mut parse_next = |parser: &mut Self| {
            if parser.eat_str("then") {
                if then.is_some() {
                    return Err(
                        parser.error(ParserErrorKind::BlockDuplicateClause("{:then}".to_string()))
                    );
                }

                if !parser.eat('}') {
                    parser.expect_whitespace()?;
                    value = Some(parser.parse_binding_pattern()?);
                    parser.skip_whitespace();
                    parser.expect('}')?;
                }

                then = Some(parser.parse_fragment()?)
            } else if parser.eat_str("catch") {
                if catch.is_some() {
                    return Err(parser.error(ParserErrorKind::BlockDuplicateClause(
                        "{:catch}".to_string(),
                    )));
                }

                if !parser.eat('}') {
                    parser.expect_whitespace()?;
                    error = Some(parser.parse_binding_pattern()?);
                    parser.skip_whitespace();
                    parser.expect('}')?;
                }

                catch = Some(parser.parse_fragment()?)
            } else {
                return Err(parser.error(ParserErrorKind::ExpectedToken(
                    "{:then ...} or {:catch ...}".to_string(),
                )));
            }

            Ok(())
        };

        while self.eat_regex(&REGEX_START_NEXT_BLOCK).is_some() {
            parse_next(self)?
        }

        self.pop_context();
        self.expect_close_block(AWAIT_STR)?;

        Ok(AwaitBlock {
            span: Span::new(start, self.offset),
            expression,
            value,
            error,
            pending,
            then,
            catch,
        })
    }

    fn parse_key_block(&mut self, start: u32) -> Result<KeyBlock<'a>, ParserError> {
        self.expect_whitespace()?;
        let expression = self.parse_expression()?;
        self.skip_whitespace();
        self.expect('}')?;
        self.push_context(Context::block_context(KEY_STR));
        let fragment = self.parse_fragment()?;
        self.pop_context();

        self.expect_close_block(KEY_STR)?;

        Ok(KeyBlock {
            span: Span::new(start, self.offset),
            expression,
            fragment,
        })
    }

    fn parse_snippet_block(&mut self, start: u32) -> Result<SnippetBlock<'a>, ParserError> {
        self.expect_whitespace()?;
        let expression = if let Some((.., ident)) = self.eat_identifier()? {
            BindingIdentifier {
                span: ident.span,
                name: ident.name,
                symbol_id: Cell::default(),
            }
        } else {
            return Err(self.error(ParserErrorKind::ExpectedIdentifier));
        };
        self.skip_whitespace();
        self.expect('(')?;
        self.skip_whitespace();
        let mut parameters = vec![];
        while !self.eat(')') {
            parameters.push(self.parse_binding_pattern()?);
            self.skip_whitespace();
            self.eat(',');
            self.skip_whitespace();
        }
        self.skip_whitespace();
        self.expect('}')?;

        self.push_context(Context::block_context(SNIPPET_STR));
        let body = self.parse_fragment()?;
        self.pop_context();
        self.expect_close_block(SNIPPET_STR)?;

        Ok(SnippetBlock {
            span: Span::new(start, self.offset),
            expression,
            parameters,
            body,
        })
    }

    fn expect_close_block(&mut self, name: &'a str) -> Result<(), ParserError> {
        self.expect_regex(&REGEX_START_CLOSE_BLOCK)?;
        self.expect_str(name)?;
        self.skip_whitespace();
        self.expect('}')?;
        Ok(())
    }
}
