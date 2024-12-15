pub use oxc_ast::AstKind as JsAstKind;
pub use oxc_ast::AstType as JsAstType;

use crate::ast::*;

// TODO: use codegen to generate this enum
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SvelteAstType {
    Text,
    Fragment,
    RegularElement,
    SvelteComponent,
    SvelteElement,
    SvelteBody,
    SvelteWindow,
    SvelteDocument,
    SvelteHead,
    SvelteFragment,
    SvelteSelf,
    TitleElement,
    SlotElement,
    Component,
    ExpressionTag,
    HtmlTag,
    DebugTag,
    ConstTag,
    RenderTag,
    Comment,
    IfBlock,
    EachBlock,
    AwaitBlock,
    KeyBlock,
    SnippetBlock,
    Root,
    Script,
    NormalAttribute,
    AnimateDirective,
    BindDirective,
    ClassDirective,
    LetDirective,
    OnDirective,
    StyleDirective,
    TransitionDirective,
    UseDirective,
}

pub enum AstType {
    Svelte(SvelteAstType),
    Js(JsAstType),
}

#[derive(Debug, Clone, Copy)]
pub enum SvelteAstKind<'a> {
    Root(&'a Root<'a>),
    Fragment(&'a Fragment<'a>),
    Script(&'a Script<'a>),
    Text(&'a Text<'a>),
    RegularElement(&'a RegularElement<'a>),
    ExpressionTag(&'a ExpressionTag<'a>),
    ConstTag(&'a ConstTag<'a>),
    NormalAttribute(&'a NormalAttribute<'a>),
    HtmlTag(&'a HtmlTag<'a>),
    DebugTag(&'a DebugTag<'a>),
    Comment(&'a Comment<'a>),
    IfBlock(&'a IfBlock<'a>),
    EachBlock(&'a EachBlock<'a>),
    AwaitBlock(&'a AwaitBlock<'a>),
    KeyBlock(&'a KeyBlock<'a>),
    SnippetBlock(&'a SnippetBlock<'a>),
    AnimateDirective(&'a AnimateDirective<'a>),
    BindDirective(&'a BindDirective<'a>),
    ClassDirective(&'a ClassDirective<'a>),
    LetDirective(&'a LetDirective<'a>),
    OnDirective(&'a OnDirective<'a>),
    StyleDirective(&'a StyleDirective<'a>),
    TransitionDirective(&'a TransitionDirective<'a>),
    UseDirective(&'a UseDirective<'a>),
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
