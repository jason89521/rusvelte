mod element;
mod root;
mod span_offset;
mod style_sheet;
mod tag;
mod text;

use derive_macro::AstTree;
pub use element::*;
pub use root::*;
pub use span_offset::SpanOffset;
mod attribute;
pub use attribute::*;
pub use tag::*;
pub use text::*;

#[derive(Debug, AstTree)]
pub struct Fragment<'a> {
    pub nodes: Vec<FragmentNode<'a>>,
}

impl<'a> Fragment<'a> {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }
}

#[derive(Debug, AstTree)]
pub enum FragmentNode<'a> {
    Text(Text<'a>),
    Element(Box<Element<'a>>),
    Tag(Tag<'a>),
}
