use oxc_span::Span;

use super::{style_sheet::StyleSheet, Fragment};

pub struct Root<'a> {
    pub css: Option<StyleSheet>,
    pub span: Span,
    pub fragment: Fragment<'a>,
}
