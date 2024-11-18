use oxc_span::Span;
use rusvelte_derive::AstTree;

use super::{style_sheet::StyleSheet, Fragment, Script};

#[derive(Debug, AstTree)]
pub struct Root<'a> {
    pub css: Option<StyleSheet<'a>>,
    pub span: Span,
    pub fragment: Fragment<'a>,
    pub module: Option<Script<'a>>,
    pub instance: Option<Script<'a>>,
}
