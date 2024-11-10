use std::sync::LazyLock;

use regex::Regex;

pub static WHITESPACE_OR_SLASH_OR_CLOSING_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\s|\/|>)").unwrap());
pub static CLOSING_COMMENT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"-->").unwrap());
pub static LESS_THEN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r">").unwrap());
pub static NON_WHITESPACE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\S").unwrap());
pub static VALID_IDENTIFIER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z_$][a-zA-Z_$0-9]*$").unwrap());
pub static TOKEN_ENDING_CHARACTER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"[\s=/>"']"#).unwrap());
