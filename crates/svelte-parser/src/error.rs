use oxc_diagnostics::OxcDiagnostic;

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
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
