use derive_macro::{AstTree, OxcSpan};
use oxc_ast::ast::Expression;
use oxc_span::{Span, SPAN};

use super::{directive::Directive, ExpressionTag, Text};

#[derive(Debug, AstTree, OxcSpan)]
pub enum Attribute<'a> {
    NormalAttribute(NormalAttribute<'a>),
    SpreadAttribute(SpreadAttribute<'a>),
    Directive(Directive<'a>),
}

#[derive(Debug, AstTree, OxcSpan)]
#[ast_tree(type = "Attribute")]
pub struct NormalAttribute<'a> {
    pub span: Span,
    pub name: &'a str,
    pub value: AttributeValue<'a>,
}

#[derive(Debug, AstTree)]
pub enum AttributeValue<'a> {
    ExpressionTag(ExpressionTag<'a>),
    Quoted(Vec<QuotedAttributeValue<'a>>),
    True,
}

impl oxc_span::GetSpan for AttributeValue<'_> {
    fn span(&self) -> Span {
        match self {
            AttributeValue::ExpressionTag(expression_tag) => expression_tag.span,
            AttributeValue::Quoted(vec) => {
                let start = vec.iter().next().unwrap().span().start;
                let end = vec.iter().last().unwrap().span().end;
                return Span::new(start, end);
            }
            AttributeValue::True => SPAN,
        }
    }
}

#[derive(Debug, AstTree, OxcSpan)]
pub enum QuotedAttributeValue<'a> {
    ExpressionTag(ExpressionTag<'a>),
    Text(Text<'a>),
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct SpreadAttribute<'a> {
    pub span: Span,
    pub expression: Expression<'a>,
}

impl<'a> Attribute<'a> {
    pub fn is_text_attribute(&self) -> bool {
        match self {
            Attribute::NormalAttribute(attribute) => attribute.value.is_text(),
            _ => false,
        }
    }
}

impl<'a> AttributeValue<'a> {
    pub fn is_true(&self) -> bool {
        match self {
            AttributeValue::True => true,
            _ => false,
        }
    }

    pub fn is_text(&self) -> bool {
        if let AttributeValue::Quoted(values) = self {
            values.len() == 1
                && if let QuotedAttributeValue::Text(_) = &values[0] {
                    true
                } else {
                    false
                }
        } else {
            false
        }
    }

    pub fn as_text(&self) -> Option<&str> {
        if !self.is_text() {
            return None;
        }
        if let AttributeValue::Quoted(values) = self {
            if let QuotedAttributeValue::Text(text) = &values[0] {
                return Some(&text.data);
            }
        }
        None
    }
}
