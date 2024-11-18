use derive_macro::{AstTree, OxcSpan};
use oxc_ast::ast::Program;
use oxc_span::Span;
use serde::Serialize;

use super::attribute::Attribute;

#[derive(Debug, AstTree, OxcSpan)]
pub struct Script<'a> {
    pub span: Span,
    pub context: ScriptContext,
    pub content: Program<'a>,
    pub attributes: Vec<Attribute<'a>>,
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
