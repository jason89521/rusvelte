use oxc_allocator::Vec;
use oxc_ast::ast::Program;
use oxc_span::Span;
use rusvelte_derive::{AstTree, OxcSpan};
use serde::Serialize;

use super::{attribute::Attribute, Comment};

#[derive(Debug, AstTree, OxcSpan)]
pub struct Script<'a> {
    pub span: Span,
    pub context: ScriptContext,
    pub content: Program<'a>,
    pub attributes: Vec<'a, Attribute<'a>>,
    /// svelte store the comment into the Program, but I think it is not necessary to store there.
    pub leading_comment: Option<Comment<'a>>,
}

#[derive(Debug)]
pub enum ScriptContext {
    Default,
    Module,
}

impl Serialize for ScriptContext {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ScriptContext::Default => serializer.serialize_str("default"),
            ScriptContext::Module => serializer.serialize_str("module"),
        }
    }
}
