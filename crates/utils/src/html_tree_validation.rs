use std::{collections::HashMap, sync::LazyLock};

pub enum AutoClosingChild {
    Direct(Vec<&'static str>),
    Descendant {
        descendant: Vec<&'static str>,
        reset_by: Vec<&'static str>,
    },
}

static AUTO_CLOSING_CHILDREN: LazyLock<HashMap<&'static str, AutoClosingChild>> =
    LazyLock::new(|| {
        HashMap::from([
            ("li", AutoClosingChild::Direct(vec!["li"])),
            // https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dt#technical_summary
            (
                "dt",
                AutoClosingChild::Descendant {
                    descendant: vec!["dt", "dd"],
                    reset_by: vec!["dl"],
                },
            ),
            (
                "dd",
                AutoClosingChild::Descendant {
                    descendant: vec!["dt", "dd"],
                    reset_by: vec!["dl"],
                },
            ),
            (
                "p",
                AutoClosingChild::Descendant {
                    descendant: vec![
                        "address",
                        "article",
                        "aside",
                        "blockquote",
                        "div",
                        "dl",
                        "fieldset",
                        "footer",
                        "form",
                        "h1",
                        "h2",
                        "h3",
                        "h4",
                        "h5",
                        "h6",
                        "header",
                        "hgroup",
                        "hr",
                        "main",
                        "menu",
                        "nav",
                        "ol",
                        "p",
                        "pre",
                        "section",
                        "table",
                        "ul",
                    ],
                    reset_by: vec![],
                },
            ),
            (
                "rt",
                AutoClosingChild::Descendant {
                    descendant: vec!["rt", "rp"],
                    reset_by: vec![],
                },
            ),
            (
                "rp",
                AutoClosingChild::Descendant {
                    descendant: vec!["rt", "rp"],
                    reset_by: vec![],
                },
            ),
            (
                "optgroup",
                AutoClosingChild::Descendant {
                    descendant: vec!["optgroup"],
                    reset_by: vec![],
                },
            ),
            (
                "option",
                AutoClosingChild::Descendant {
                    descendant: vec!["option", "optgroup"],
                    reset_by: vec![],
                },
            ),
            ("thead", AutoClosingChild::Direct(vec!["tbody", "tfoot"])),
            ("tbody", AutoClosingChild::Direct(vec!["tbody", "tfoot"])),
            ("tfoot", AutoClosingChild::Direct(vec!["tbody"])),
            ("tr", AutoClosingChild::Direct(vec!["tr", "tbody"])),
            ("td", AutoClosingChild::Direct(vec!["td", "th", "tr"])),
            ("th", AutoClosingChild::Direct(vec!["td", "th", "tr"])),
        ])
    });

pub fn closing_tag_omitted(current: &str, next: &str) -> bool {
    if let Some(disallowed) = AUTO_CLOSING_CHILDREN.get(current) {
        if next.is_empty() {
            return true;
        }
        let check_vec = match disallowed {
            AutoClosingChild::Direct(vec) => vec,
            AutoClosingChild::Descendant { descendant, .. } => descendant,
        };
        check_vec.contains(&next)
    } else {
        false
    }
}
