use rusvelte_derive::{AstTree, OxcSpan};

use oxc_allocator::Vec;

use super::{Block, Comment, Element, ExpressionTag, Tag, Text};

#[derive(Debug, AstTree)]
pub struct Fragment<'a> {
    pub nodes: Vec<'a, FragmentNode<'a>>,
    #[ast_ignore]
    pub dynamic: bool,
}

#[derive(Debug, AstTree, OxcSpan)]
pub enum FragmentNode<'a> {
    Text(Text<'a>),
    Element(Box<Element<'a>>),
    Tag(Tag<'a>),
    Comment(Comment<'a>),
    Block(Block<'a>),
}

impl<'a> FragmentNode<'a> {
    pub fn is_regular_element(&self) -> bool {
        if let Self::Element(element) = self {
            matches!(element.as_ref(), Element::RegularElement(_))
        } else {
            false
        }
    }

    pub fn is_expression_tag(&self) -> bool {
        if let Self::Tag(tag) = self {
            tag.is_expression_tag()
        } else {
            false
        }
    }

    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text(_))
    }

    pub fn as_expression_tag(&self) -> Option<&ExpressionTag<'a>> {
        if let Self::Tag(Tag::ExpressionTag(tag)) = &self {
            Some(tag)
        } else {
            None
        }
    }

    pub fn as_text(&self) -> Option<&Text<'a>> {
        if let Self::Text(text) = self {
            Some(text)
        } else {
            None
        }
    }

    pub fn as_text_mut(&mut self) -> Option<&mut Text<'a>> {
        if let Self::Text(text) = self {
            Some(text)
        } else {
            None
        }
    }

    pub fn dynamic(&self) -> bool {
        match self {
            Self::Tag(Tag::RenderTag(tag)) => tag.dynamic,
            Self::Element(element) => {
                if let Element::Component(component) = element.as_ref() {
                    component.dynamic
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
