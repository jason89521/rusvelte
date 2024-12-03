pub use oxc_ast::AstKind as JsAstKind;
pub use oxc_ast::AstType;

use crate::ast::*;

#[derive(Debug, Clone, Copy)]
pub enum SvelteAstKind<'a> {
    Root(&'a Root<'a>),
    Fragment(&'a Fragment<'a>),
    Script(&'a Script<'a>),
    Text(&'a Text<'a>),
    RegularElement(&'a RegularElement<'a>),
    ExpressionTag(&'a ExpressionTag<'a>),
    NormalAttribute(&'a NormalAttribute<'a>),
}

#[derive(Debug, Clone, Copy)]
pub enum AstKind<'a> {
    Svelte(SvelteAstKind<'a>),
    Js(JsAstKind<'a>),
}

impl<'a> AstKind<'a> {
    pub fn as_js(&self) -> Option<JsAstKind<'a>> {
        if let AstKind::Js(kind) = self {
            Some(*kind)
        } else {
            None
        }
    }

    pub fn as_svelte(&self) -> Option<SvelteAstKind<'a>> {
        if let AstKind::Svelte(kind) = self {
            Some(*kind)
        } else {
            None
        }
    }
}
