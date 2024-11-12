mod element;
mod root;
mod script;
mod span_offset;
mod style_sheet;
mod tag;
mod text;

use derive_macro::{AstTree, OxcSpan};
pub use element::*;
pub use root::*;
pub use span_offset::SpanOffset;
mod attribute;
pub use script::*;
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

#[derive(Debug, AstTree, OxcSpan)]
pub enum FragmentNode<'a> {
    Text(Text<'a>),
    Element(Box<Element<'a>>),
    Tag(Tag<'a>),
}
