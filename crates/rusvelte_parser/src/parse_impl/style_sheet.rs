use std::sync::LazyLock;

use oxc_allocator::Vec;
use oxc_span::Span;
use regex::Regex;

use crate::{
    error::{ParserError, ParserErrorKind},
    Parser,
};

use rusvelte_ast::ast::{
    AtRule, Attribute, AttributeSelector, BlockChild, CSSBlock, ClassSelector, Combinator,
    ComplexSelector, Declaration, IdSelector, NestingSelector, Nth, Percentage,
    PseudoClassSelector, PseudoElementSelector, Rule, SelectorList, SimpleSelector, StyleSheet,
    StyleSheetChild, StyleSheetContent, TypeSelector,
};

static REGEX_START_WITH_CLOSING_STYLE_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^<\/style\s*>"#).unwrap());
static REGEX_LEADING_HYPHEN_OR_DIGIT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^[-?|\d]"#).unwrap());
static REGEX_MATCHER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^[~^$*|]?="#).unwrap());
static REGEX_ATTRIBUTE_FLAGS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^[a-zA-Z]+"#).unwrap());
static REGEX_PERCENTAGE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\d+(\.\d+)?%"#).unwrap());
static REGEX_COMBINATOR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^(\+|~|>|\|\|)"#).unwrap());
static REGEX_WHITESPACE_OR_COLON: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"[\s:]"#).unwrap());
static REGEX_NTH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(even|odd|\+?(\d+|\d*n(\s*[+-]\s*\d+)?)|-\d*n(\s*\+\s*\d+))((\s+of\s+)|\s*[,)])")
        .unwrap()
});

impl<'a> Parser<'a> {
    pub fn parse_style_sheet(
        &mut self,
        start: u32,
        attributes: Vec<'a, Attribute<'a>>,
    ) -> Result<StyleSheet<'a>, ParserError> {
        let content_start = self.offset;
        let children = self.parser_style_sheet_body()?;
        let content_end = self.offset;
        self.expect_regex(&REGEX_START_WITH_CLOSING_STYLE_TAG)?;

        Ok(StyleSheet {
            span: Span::new(start, self.offset),
            attributes,
            children,
            content: StyleSheetContent {
                span: Span::new(content_start, content_end),
                styles: &self.source[content_start as usize..content_end as usize],
                comment: None,
            },
        })
    }

    fn parser_style_sheet_body(&mut self) -> Result<Vec<'a, StyleSheetChild<'a>>, ParserError> {
        let mut children = self.ast.vec([]);
        self.skip_comment_or_whitespace();
        while self.offset_usize() < self.source.len() {
            self.skip_comment_or_whitespace();

            if self
                .match_regex(&REGEX_START_WITH_CLOSING_STYLE_TAG)
                .is_some()
            {
                return Ok(children);
            }

            if self.match_ch('@') {
                children.push(StyleSheetChild::AtRule(self.parse_css_at_rule()?))
            } else {
                children.push(StyleSheetChild::Rule(self.parse_css_rule()?))
            }
        }

        Err(ParserError::new(
            Span::empty(self.offset),
            ParserErrorKind::ExpectedStr(String::from("</style")),
        ))
    }

    fn parse_css_at_rule(&mut self) -> Result<AtRule<'a>, ParserError> {
        let start = self.offset;
        self.expect('@')?;
        let name = self.parse_css_identifier()?;
        let prelude = self.parse_css_value()?;
        let mut block = None;

        if self.match_ch('{') {
            // e.g. `@media (...) {...}`
            block = Some(self.parse_css_block()?);
        } else {
            // e.g. `@import '...'`
            self.expect(';')?;
        }

        Ok(AtRule {
            span: Span::new(start, self.offset),
            name,
            prelude,
            block,
        })
    }

    fn parse_css_rule(&mut self) -> Result<Rule<'a>, ParserError> {
        let start = self.offset;
        let prelude = self.parse_selector_list(false)?;
        let block = self.parse_css_block()?;

        Ok(Rule {
            span: Span::new(start, self.offset),
            prelude,
            block,
        })
    }

    fn parse_css_block(&mut self) -> Result<CSSBlock<'a>, ParserError> {
        let start = self.offset;
        self.expect('{')?;
        let mut children = self.ast.vec([]);
        while self.peek().is_some() {
            self.skip_comment_or_whitespace();
            if self.match_ch('}') {
                break;
            } else {
                children.push(self.parse_block_child()?);
            }
        }
        self.expect('}')?;

        Ok(CSSBlock {
            span: Span::new(start, self.offset),
            children,
        })
    }

    fn parse_block_child(&mut self) -> Result<BlockChild<'a>, ParserError> {
        if self.match_ch('@') {
            return Ok(BlockChild::AtRule(self.parse_css_at_rule()?));
        }

        let start = self.offset;
        self.parse_css_value()?;
        let ch = self
            .peek()
            .ok_or(self.error(ParserErrorKind::UnexpectedEOF))?;
        self.offset = start;

        if ch == '{' {
            Ok(BlockChild::Rule(self.parse_css_rule()?))
        } else {
            Ok(BlockChild::Declaration(self.parse_css_declaration()?))
        }
    }

    fn parse_css_declaration(&mut self) -> Result<Declaration<'a>, ParserError> {
        let start = self.offset;
        let property = self.eat_until(&REGEX_WHITESPACE_OR_COLON);
        self.skip_whitespace();
        self.eat(':');
        let index = self.offset;
        self.skip_whitespace();

        let value = self.parse_css_value()?;
        if value.is_empty() && !property.starts_with("--") {
            return Err(ParserError {
                span: Span::new(start, index),
                kind: ParserErrorKind::CssEmptyDeclaration,
            });
        }

        let end = self.offset;
        if !self.match_ch('}') {
            self.expect(';')?;
        }

        Ok(Declaration {
            span: Span::new(start, end),
            property,
            value,
        })
    }

    fn parse_selector_list(
        &mut self,
        inside_pseudo_class: bool,
    ) -> Result<SelectorList<'a>, ParserError> {
        let mut children = self.ast.vec([]);
        self.skip_comment_or_whitespace();
        let start = self.offset;
        while self.peek().is_some() {
            children.push(self.parse_selector(inside_pseudo_class)?);
            let end = self.offset;
            self.skip_comment_or_whitespace();
            let should_return = if inside_pseudo_class {
                self.match_ch(')')
            } else {
                self.match_ch('{')
            };
            if should_return {
                return Ok(SelectorList {
                    span: Span::new(start, end),
                    children,
                });
            }
            self.expect(',')?;
            self.skip_comment_or_whitespace();
        }

        Err(self.error(ParserErrorKind::UnexpectedEOF))
    }

    fn parse_selector(
        &mut self,
        inside_pseudo_class: bool,
    ) -> Result<ComplexSelector<'a>, ParserError> {
        let list_start = self.offset;
        let mut children = self.ast.vec([]);
        let mut relative_selector = self.ast.relative_selector(None, list_start);
        while self.peek().is_some() {
            let start = self.offset;
            if self.eat('&') {
                relative_selector
                    .selectors
                    .push(SimpleSelector::NestingSelector(NestingSelector {
                        span: Span::new(start, self.offset),
                        name: "&",
                    }));
            } else if self.eat('*') {
                let mut name = "*";
                if self.eat('|') {
                    name = self.parse_css_identifier()?;
                }
                relative_selector
                    .selectors
                    .push(SimpleSelector::TypeSelector(TypeSelector {
                        span: Span::new(start, self.offset),
                        name,
                    }));
            } else if self.eat('#') {
                let name = self.parse_css_identifier()?;
                relative_selector
                    .selectors
                    .push(SimpleSelector::IdSelector(IdSelector {
                        span: Span::new(start, self.offset),
                        name,
                    }))
            } else if self.eat('.') {
                let name = self.parse_css_identifier()?;
                relative_selector
                    .selectors
                    .push(SimpleSelector::ClassSelector(ClassSelector {
                        span: Span::new(start, self.offset),
                        name,
                    }))
            } else if self.eat_str("::") {
                let name = self.parse_css_identifier()?;
                relative_selector
                    .selectors
                    .push(SimpleSelector::PseudoElementSelector(
                        PseudoElementSelector {
                            span: Span::new(start, self.offset),
                            name,
                        },
                    ));
                // We read the inner selectors of a pseudo element to ensure it parses correctly,
                // but we don't do anything with the result.
                if self.eat('(') {
                    self.parse_selector_list(true)?;
                    self.expect(')')?;
                }
            } else if self.eat(':') {
                let name = self.parse_css_identifier()?;
                let mut args = None;
                if self.eat('(') {
                    args = Some(self.parse_selector_list(true)?);
                    self.expect(')')?;
                }
                relative_selector
                    .selectors
                    .push(SimpleSelector::PseudoClassSelector(PseudoClassSelector {
                        span: Span::new(start, self.offset),
                        args,
                        name,
                    }))
            } else if self.eat('[') {
                self.skip_whitespace();
                let name = self.parse_css_identifier()?;
                self.skip_whitespace();
                let mut value = None;
                let matcher = self.eat_regex(&REGEX_MATCHER);
                if matcher.is_some() {
                    self.skip_whitespace();
                    value = Some(self.parse_css_attribute_value()?);
                }
                self.skip_whitespace();
                let flags = self.eat_regex(&REGEX_ATTRIBUTE_FLAGS);
                self.skip_whitespace();
                self.expect(']')?;
                relative_selector
                    .selectors
                    .push(SimpleSelector::AttributeSelector(AttributeSelector {
                        span: Span::new(start, self.offset),
                        name,
                        matcher,
                        value,
                        flags,
                    }))
            } else if inside_pseudo_class && self.match_regex(&REGEX_NTH).is_some() {
                let value = self.parse_nth_regex().expect("Nth regex failed");
                relative_selector.selectors.push(SimpleSelector::Nth(Nth {
                    span: Span::new(start, self.offset),
                    value,
                }));
            } else if self.match_regex(&REGEX_PERCENTAGE).is_some() {
                let value = self.eat_regex(&REGEX_PERCENTAGE).unwrap();
                relative_selector
                    .selectors
                    .push(SimpleSelector::Percentage(Percentage {
                        span: Span::new(start, self.offset),
                        value,
                    }))
            } else if self.match_regex(&REGEX_COMBINATOR).is_none() {
                let mut name = self.parse_css_identifier()?;
                if self.eat('|') {
                    name = self.parse_css_identifier()?;
                }
                relative_selector
                    .selectors
                    .push(SimpleSelector::TypeSelector(TypeSelector {
                        span: Span::new(start, self.offset),
                        name,
                    }))
            }

            let index = self.offset;
            self.skip_comment_or_whitespace();

            let should_rewind = if inside_pseudo_class {
                self.match_ch(')')
            } else {
                self.match_ch('{')
            } || self.match_ch(',');
            if should_rewind {
                self.offset = index;
                relative_selector.span.end = index;
                children.push(relative_selector);

                return Ok(ComplexSelector {
                    span: Span::new(list_start, index),
                    children,
                });
            }

            self.offset = index;
            let combinator = self.parse_css_combinator()?;
            if let Some(combinator) = combinator {
                if !relative_selector.selectors.is_empty() {
                    relative_selector.span.end = index;
                    children.push(relative_selector)
                }

                let combinator_start = combinator.span.start;
                relative_selector = self
                    .ast
                    .relative_selector(Some(combinator), combinator_start);
                self.skip_whitespace();

                let css_selector_invalid = self.match_ch(',')
                    || if inside_pseudo_class {
                        self.match_ch(')')
                    } else {
                        self.match_ch('{')
                    };
                if css_selector_invalid {
                    return Err(ParserError::new(
                        Span::empty(self.offset),
                        ParserErrorKind::CssSelectorInvalid,
                    ));
                }
            }
        }

        Err(self.error(ParserErrorKind::UnexpectedEOF))
    }

    fn parse_css_combinator(&mut self) -> Result<Option<Combinator<'a>>, ParserError> {
        let start = self.offset;
        self.skip_whitespace();

        let index = self.offset;
        let name = self.eat_regex(&REGEX_COMBINATOR);
        if let Some(name) = name {
            let end = self.offset;
            self.skip_whitespace();

            return Ok(Some(Combinator {
                span: Span::new(index, end),
                name,
            }));
        }

        if self.offset != start {
            return Ok(Some(Combinator {
                span: Span::new(start, self.offset),
                name: " ",
            }));
        }

        Ok(None)
    }

    fn parse_css_identifier(&mut self) -> Result<&'a str, ParserError> {
        let start = self.offset;
        if self.match_str("--") || self.match_regex(&REGEX_LEADING_HYPHEN_OR_DIGIT).is_some() {
            return Err(ParserError::new(
                Span::empty(start),
                ParserErrorKind::CssExpectedIdentifier,
            ));
        }

        while let Some(ch) = self.peek() {
            if !(ch == '\\'
                || !ch.is_ascii()
                || matches!(ch, '0'..='9' | 'a'..='z' | 'A'..='Z' | '-' | '_' ))
            {
                break;
            }
            self.next();
        }

        let result = &self.source[start as usize..self.offset_usize()];
        if result.is_empty() {
            return Err(ParserError::new(
                Span::empty(start),
                ParserErrorKind::CssExpectedIdentifier,
            ));
        }

        Ok(result)
    }

    fn parse_css_value(&mut self) -> Result<&'a str, ParserError> {
        let mut in_url = false;
        let mut quote_mark: Option<char> = None;
        let start = self.offset_usize();

        // TODO
        // I don't know what the original svelte parser doing in this function
        // Use @identifier \\; and see the ast's prelude.
        while let Some(ch) = self.peek() {
            match ch {
                ch if quote_mark.map_or(false, |q| q == ch) => {
                    quote_mark.take();
                }
                ')' => {
                    in_url = false;
                }
                '\'' | '"' if quote_mark.is_none() => {
                    quote_mark = Some(ch);
                }
                '(' if &self.source[self.offset_usize() - 3..self.offset_usize()] == "url" => {
                    in_url = true;
                }
                ';' | '{' | '}' if !in_url && quote_mark.is_none() => {
                    return Ok(self.source[start..self.offset_usize()].trim());
                }
                _ => (),
            }
            self.next();
        }

        Err(ParserError::new(
            Span::empty(self.offset),
            ParserErrorKind::UnexpectedEOF,
        ))
    }

    fn parse_css_attribute_value(&mut self) -> Result<&'a str, ParserError> {
        let quote_mark = {
            if self.eat('\'') {
                Some('\'')
            } else if self.eat('"') {
                Some('"')
            } else {
                None
            }
        };
        let start = self.offset_usize();
        while let Some(ch) = self.peek() {
            if quote_mark.map_or_else(|| ch.is_whitespace() || ch == ']', |m| m == ch) {
                if let Some(m) = quote_mark {
                    self.expect(m)?;
                }

                return Ok(self.source[start..self.offset_usize()].trim());
            }
            self.next();
        }

        Err(ParserError::new(
            Span::empty(self.offset),
            ParserErrorKind::UnexpectedEOF,
        ))
    }

    fn parse_nth_regex(&mut self) -> Option<&'a str> {
        let value = REGEX_NTH.captures(self.remain()).and_then(|caps| {
            if caps.get(6).is_some() {
                // found " of "
                caps.get(0).map(|mat| mat.as_str())
            } else {
                caps.get(1).map(|mat| mat.as_str())
            }
        })?;
        self.offset += value.len() as u32;
        Some(value)
    }
}
