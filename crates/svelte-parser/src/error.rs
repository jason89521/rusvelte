use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

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
    #[error(r#"Expect a "{expected}", but found a "{found}"."#)]
    ExpectChar { expected: char, found: char },
    #[error(r#"Expect a "{0}" str."#)]
    ExpectStr(String),

    // From svelte
    #[error("Unexpected EOF. {0}")]
    UnexpectedEOF(char),
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
}
