use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use oxc_allocator::Vec;
use oxc_ast::ast::Expression;
use oxc_span::Span;

use crate::ast::*;

use super::AstBuilder;

impl<'a> AstBuilder<'a> {
    pub fn fragment(self, nodes: Vec<'a, FragmentNode<'a>>, transparent: bool) -> Fragment<'a> {
        Fragment {
            nodes,
            scope_id: Cell::new(None),
            metadata: RefCell::new(FragmentMetadata {
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

    pub fn expression_tag(self, span: Span, expression: Expression<'a>) -> ExpressionTag<'a> {
        ExpressionTag {
            span,
            expression,
            expression_metadata: Rc::new(RefCell::new(ExpressionMetadataInner::default())),
        }
    }

    pub fn normal_attribute(
        self,
        span: Span,
        name: &'a str,
        value: AttributeValue<'a>,
    ) -> NormalAttribute<'a> {
        NormalAttribute {
            span,
            name,
            value,
            expression_metadata: Rc::new(RefCell::new(ExpressionMetadataInner::default())),
        }
    }
}
