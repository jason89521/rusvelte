use derive_macro::AstTree;
use oxc_span::Span;

use super::{Fragment, Script};

#[derive(Debug, AstTree)]
pub struct Root<'a> {
    pub span: Span,
    pub fragment: Fragment<'a>,
    pub module: Option<Script<'a>>,
    pub instance: Option<Script<'a>>,
}
