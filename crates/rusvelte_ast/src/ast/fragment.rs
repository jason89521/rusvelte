use rusvelte_derive::{AstTree, OxcSpan};

use super::{Block, Comment, Element, Tag, Text};

#[derive(Debug, AstTree, Default)]
pub struct Fragment<'a> {
    pub nodes: Vec<FragmentNode<'a>>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub enum FragmentNode<'a> {
    Text(Text<'a>),
    Element(Box<Element<'a>>),
    Tag(Tag<'a>),
    Comment(Comment<'a>),
    Block(Block<'a>),
}
