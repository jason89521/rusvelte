use derive_macro::AstTree;
use oxc_span::Span;

use super::Fragment;

#[derive(Debug, AstTree)]
pub struct Root<'a> {
    pub span: Span,
    pub fragment: Fragment<'a>,
}
