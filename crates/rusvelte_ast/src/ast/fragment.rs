use rusvelte_derive::{AstTree, OxcSpan};

use super::{Comment, Element, Tag, Text};

#[derive(Debug, AstTree)]
pub struct Fragment<'a> {
    pub nodes: Vec<FragmentNode<'a>>,
}

impl<'a> Fragment<'a> {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }
}

#[derive(Debug, AstTree, OxcSpan)]
pub enum FragmentNode<'a> {
    Text(Text<'a>),
    Element(Box<Element<'a>>),
    Tag(Tag<'a>),
    Comment(Comment<'a>),
}
