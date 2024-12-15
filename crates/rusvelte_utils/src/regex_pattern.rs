use std::sync::LazyLock;

use regex::Regex;

/// Not \S because that also removes explicit whitespace defined through things like `&nbsp;`
pub static REGEX_NOT_WHITESPACE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("[^ \t\r\n]").unwrap());
pub static REGEX_ENDS_WITH_WHITESPACES: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("[ \t\r\n]+$").unwrap());
pub static REGEX_NOT_VALID_IDENTIFIER_CHAR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("[^a-zA-Z0-9_$]").unwrap());
