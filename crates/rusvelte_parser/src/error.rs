use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

use crate::Parser;

#[derive(thiserror::Error)]
pub struct ParserError {
    pub kind: ParserErrorKind,
    pub span: Span,
}

impl ParserError {
    pub fn new(span: Span, kind: ParserErrorKind) -> Self {
        Self { span, kind }
    }
}

impl std::fmt::Debug for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]: {}", self.span.start, self.span.end, self.kind)
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error[{}, {}]: {}",
            self.span.start, self.span.end, self.kind
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParserErrorKind {
    #[error("Parse program error: {0:#?}.")]
    ParseProgram(Vec<OxcDiagnostic>),
    #[error("Parse expression error: {0:#?}.")]
    ParseExpression(Vec<OxcDiagnostic>),
    #[error(r#"Expected a "{expected}", but found a "{found}"."#)]
    ExpectedChar { expected: char, found: char },
    #[error(r#"Expected a "{0}" str."#)]
    ExpectedStr(String),
    #[error("Unexpected EOF. {0}")]
    UnexpectedEOFWithChar(char),
    #[error(r#"Expected closing tag."#)]
    ExpectedClosingTag,
    #[error(r#"ClassDirective should have expression"#)]
    ClassDirectiveShouldHaveExpression,
    #[error(r#"UseDirective should have expression"#)]
    UseDirectiveShouldHaveExpression,

    // From svelte
    #[error("Unexpected EOF.")]
    UnexpectedEOF,
    #[error("Attribute shorthand cannot be empty")]
    AttributeEmptyShorthand,
    #[error("`<{0}>` was left open`")]
    ElementUnclosed(String),
    #[error("The `${0}` attribute is reserved and cannot be used")]
    ScriptReservedAttribute(String),
    #[error("If the `${0}` attribute is supplied, it must be a boolean attribute")]
    ScriptInvalidAttributeValue(String),
    #[error("If the context attribute is supplied, its value must be \"module\"")]
    ScriptInvalidContext,
    #[error("Expected attribute value")]
    ExpectedAttributeValue,
    #[error("Expected token {0}")]
    ExpectedToken(char),
    #[error("A component can have a single top-level `<script>` element and/or a single top-level `<script module>` element")]
    ScriptDuplicate,
    #[error("Expected a valid CSS identifier")]
    CssExpectedIdentifier,
    #[error("A component can have a single top-level `<style>` element")]
    StyleDuplicate,
    #[error("Invalid selector")]
    CssSelectorInvalid,
    #[error("Declaration cannot be empty")]
    CssEmptyDeclaration,
    #[error("Valid `<svelte:...>` tag names are {0}")]
    SvelteMetaInvalidTag(String),
    #[error("Expected a valid element or component name. Components must have a valid variable name or dot notation expression")]
    TagInvalidName,
    #[error("A component can only have one `<{0}>` element")]
    SvelteMetaDuplicate(String),
    #[error("`<{0}>` tags cannot be inside elements or blocks")]
    SvelteMetaInvalidPlacement(String),
    #[error("`</${name}>` attempted to close element that was already automatically closed by `<{reason}>` (cannot nest `<{reason}>` inside `<{name}>`)")]
    ElementInvalidClosingTagAutoClosed { reason: String, name: String },
    #[error("`</{0}>` attempted to close an element that was not open")]
    ElementInvalidClosingTag(String),
    #[error("{{#{name} ...}} block cannot be {location}")]
    BlockInvalidPlacement { name: String, location: String },
    #[error("{{@{name} ...}} tag cannot be {location}")]
    TagInvalidPlacement { name: String, location: String },
    #[error("`${0}` name cannot be empty")]
    DirectiveMissingName(String),
    #[error("Directive value must be a JavaScript expression enclosed in curly braces")]
    DirectiveInvalidValue,
    #[error("Attributes need to be unique")]
    AttributeDuplicate,
}

impl Parser<'_> {
    pub fn error(&self, kind: ParserErrorKind) -> ParserError {
        ParserError {
            span: Span::empty(self.offset),
            kind,
        }
    }

    pub fn error_at(&self, at: u32, kind: ParserErrorKind) -> ParserError {
        ParserError {
            span: Span::empty(at),
            kind,
        }
    }
}
