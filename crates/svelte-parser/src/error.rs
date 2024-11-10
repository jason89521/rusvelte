use oxc_diagnostics::OxcDiagnostic;

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Parse expression error: {0:#?}.")]
    ParseExpression(Vec<OxcDiagnostic>),
    #[error(r#"Expect a "{expected}", but found a "{found}"."#)]
    ExpectChar { expected: char, found: char },
    #[error(r#"Expect a "{0}" str."#)]
    ExpectStr(String),
    #[error("Unexpected EOF. {0}")]
    UnexpectedEOF(char),
}
