use oxc_ast::ast::Expression;
use oxc_span::{Span, SPAN};
use rusvelte_derive::{AstTree, OxcSpan};

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
                Span::new(start, end)
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

    pub fn is_expression_attribute(&self) -> bool {
        if let Attribute::NormalAttribute(attr) = self {
            attr.value.is_expression_tag()
        } else {
            false
        }
    }

    pub fn get_expression_tag(&self) -> Option<&ExpressionTag<'_>> {
        if let Attribute::NormalAttribute(attr) = self {
            attr.value.as_expression_tag()
        } else {
            None
        }
    }

    pub fn name(&self) -> &'a str {
        match self {
            Attribute::NormalAttribute(normal_attribute) => normal_attribute.name,
            Attribute::SpreadAttribute(_) => "",
            Attribute::Directive(directive) => directive.name(),
        }
    }
}

impl<'a> AttributeValue<'a> {
    pub fn is_true(&self) -> bool {
        matches!(self, AttributeValue::True)
    }

    pub fn is_text(&self) -> bool {
        if let AttributeValue::Quoted(values) = self {
            values.len() == 1 && matches!(&values[0], QuotedAttributeValue::Text(_))
        } else {
            false
        }
    }

    pub fn as_raw_text(&self) -> Option<&'a str> {
        if !self.is_text() {
            return None;
        }
        if let AttributeValue::Quoted(values) = self {
            if let QuotedAttributeValue::Text(text) = &values[0] {
                return Some(text.raw);
            }
        }
        None
    }

    pub fn is_expression_tag(&self) -> bool {
        matches!(self, AttributeValue::ExpressionTag(_))
    }

    pub fn as_expression_tag(&self) -> Option<&ExpressionTag<'a>> {
        if let AttributeValue::ExpressionTag(tag) = self {
            Some(tag)
        } else {
            None
        }
    }

    pub fn get_static_value(&self) -> Option<&'a str> {
        match self {
            AttributeValue::ExpressionTag(expression_tag) => expression_tag.get_static_value(),
            AttributeValue::Quoted(vec) if vec.len() <= 1 => {
                if let Some(value) = vec.first() {
                    match value {
                        QuotedAttributeValue::ExpressionTag(expression_tag) => {
                            expression_tag.get_static_value()
                        }
                        QuotedAttributeValue::Text(text) => Some(text.raw),
                    }
                } else {
                    Some("true")
                }
            }
            AttributeValue::True => Some("true"),
            _ => None,
        }
    }

    /// Svelte only accept boolean literal, but for convenient, we consider all "true" as boolean value
    /// TODO: align the Svelte's design when we publish this crate
    pub fn get_boolean_value(&self) -> bool {
        self.get_static_value().map_or(false, |v| v == "true")
    }
}
