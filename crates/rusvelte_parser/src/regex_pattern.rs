use std::sync::LazyLock;

use regex::Regex;

pub static REGEX_WHITESPACE_OR_SLASH_OR_CLOSING_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\s|\/|>)").unwrap());
pub static REGEX_CLOSING_COMMENT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"-->").unwrap());
pub static REGEX_NON_WHITESPACE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\S").unwrap());
pub static REGEX_START_WHITESPACE_WITH_CLOSING_CURLY_BRACE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\s*}"#).unwrap());
