use std::cell::Cell;

use oxc_allocator::Vec;
use oxc_span::Span;

use crate::ast::*;

use super::AstBuilder;

impl<'a> AstBuilder<'a> {
    pub fn fragment(self, nodes: Vec<'a, FragmentNode<'a>>, transparent: bool) -> Fragment<'a> {
        Fragment {
            nodes,
            scope_id: Cell::new(None),
            metadata: Cell::new(FragmentMetadata {
                transparent,
                dynamic: false,
            }),
        }
    }

    pub fn move_fragment_nodes(self, fragment: &mut Fragment<'a>) -> Vec<'a, FragmentNode<'a>> {
        std::mem::replace(&mut fragment.nodes, self.vec([]))
    }

    pub fn relative_selector(
        self,
        combinator: Option<Combinator<'a>>,
        start: u32,
    ) -> RelativeSelector<'a> {
        RelativeSelector {
            span: Span::empty(start),
            combinator,
            selectors: self.vec([]),
        }
    }

    pub fn svelte_options(self, span: Span) -> SvelteOptions<'a> {
        SvelteOptions {
            span,
            runes: None,
            immutable: None,
            accessors: None,
            attributes: self.vec([]),
            custom_element: None,
            namespace: None,
            css: None,
            preserve_whitespace: None,
        }
    }
}
