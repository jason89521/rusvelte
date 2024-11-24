use oxc_span::{GetSpan, Span};
use rusvelte_ast::ast::Fragment;

/// Return the fragment's span if it has at least one node.
pub fn disallow_children(fragment: &Fragment) -> Option<Span> {
    if fragment.nodes.is_empty() {
        None
    } else {
        Some(Span::new(
            fragment.nodes.first().unwrap().span().start,
            fragment.nodes.last().unwrap().span().end,
        ))
    }
}
