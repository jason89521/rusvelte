mod element;
mod span_offset;
mod tag;
mod text;

pub use element::*;
pub use span_offset::SpanOffset;
pub use tag::*;
pub use text::*;

#[derive(Debug)]
pub struct Fragment<'a> {
    pub nodes: Vec<FragmentNode<'a>>,
}

impl<'a> Fragment<'a> {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }
}

#[derive(Debug)]
pub enum FragmentNode<'a> {
    Text(Text<'a>),
    Element(Box<Element<'a>>),
    Tag(Tag<'a>),
}
