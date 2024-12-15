use std::{cell::RefCell, collections::HashSet, rc::Rc};

mod attribute;
mod block;
mod directive;
mod element;
mod fragment;
mod root;
mod script;
mod style_sheet;
mod tag;
mod text;

pub use attribute::*;
pub use block::*;
pub use directive::*;
pub use element::*;
pub use fragment::*;
use oxc_syntax::symbol::SymbolId;
pub use root::*;
pub use script::*;
pub use style_sheet::*;
pub use tag::*;
pub use text::*;

#[derive(Debug, Default)]
pub struct ExpressionMetadataInner {
    pub has_state: bool,
    pub has_call: bool,
    pub dependencies: HashSet<SymbolId>,
}

pub type ExpressionMetadata = Rc<RefCell<ExpressionMetadataInner>>;
