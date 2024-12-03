use std::sync::LazyLock;

use regex::Regex;

/// Not \S because that also removes explicit whitespace defined through things like `&nbsp;`
pub static REGEX_NOT_WHITESPACE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("[^ \t\r\n]").unwrap());
