use std::sync::LazyLock;

use regex::Regex;

pub static REGEX_WHITESPACE_OR_SLASH_OR_CLOSING_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\s|\/|>)").unwrap());
pub static REGEX_CLOSING_COMMENT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"-->").unwrap());
pub static REGEX_LESS_THEN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r">").unwrap());
pub static REGEX_NON_WHITESPACE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\S").unwrap());
