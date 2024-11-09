mod element;
mod root;
mod span_offset;
mod style_sheet;
mod tag;
mod text;

pub use element::*;
pub use root::*;
pub use span_offset::SpanOffset;
pub use style_sheet::*;
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
