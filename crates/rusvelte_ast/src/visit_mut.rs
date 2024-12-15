#![allow(unused_variables)]
pub use oxc_ast::VisitMut as JsVisitMut;

use oxc_allocator::Vec as OxcVec;
use oxc_ast::ast::Statement;

use crate::{ast::*, ast_kind::SvelteAstType};
use walk_mut::*;

pub trait VisitMut<'a>: JsVisitMut<'a> {
    fn enter_svelte_node(&mut self, kind: SvelteAstType) {}
    fn leave_svelte_node(&mut self, kind: SvelteAstType) {}
    fn vec<T, const N: usize>(&self, array: [T; N]) -> OxcVec<'a, T>;

    fn visit_root(&mut self, it: &mut Root<'a>) -> OxcVec<'a, Statement<'a>> {
        walk_root(self, it)
    }
    fn visit_fragment(&mut self, it: &mut Fragment<'a>) -> OxcVec<'a, Statement<'a>> {
        walk_fragment(self, it)
    }
    fn visit_script(&mut self, it: &mut Script<'a>) -> OxcVec<'a, Statement<'a>> {
        walk_script(self, it)
    }
    fn visit_fragment_node(&mut self, it: &mut FragmentNode<'a>) -> OxcVec<'a, Statement<'a>> {
        walk_fragment_node(self, it)
    }
    fn visit_text(&mut self, it: &mut Text<'a>) -> OxcVec<'a, Statement<'a>> {
        walk_text(self, it)
    }
    fn visit_element(&mut self, it: &mut Element<'a>) -> OxcVec<'a, Statement<'a>> {
        walk_element(self, it)
    }
    fn visit_regular_element(&mut self, it: &mut RegularElement<'a>) -> OxcVec<'a, Statement<'a>> {
        walk_regular_element(self, it)
    }
    fn visit_tag(&mut self, it: &mut Tag<'a>) -> OxcVec<'a, Statement<'a>> {
        walk_tag(self, it)
    }
    fn visit_comment(&mut self, it: &mut Comment<'a>) -> OxcVec<'a, Statement<'a>> {
        self.vec([])
    }
    fn visit_block(&mut self, it: &mut Block<'a>) -> OxcVec<'a, Statement<'a>> {
        self.vec([])
    }
    fn visit_expression_tag(&mut self, it: &mut ExpressionTag<'a>) -> OxcVec<'a, Statement<'a>> {
        walk_expression_tag(self, it)
    }
    fn visit_attributes(&mut self, it: &mut [Attribute<'a>]) -> OxcVec<'a, Statement<'a>> {
        walk_attributes(self, it)
    }
    fn visit_attribute(&mut self, it: &mut Attribute<'a>) -> OxcVec<'a, Statement<'a>> {
        walk_attribute(self, it)
    }
    fn visit_normal_attribute(
        &mut self,
        it: &mut NormalAttribute<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        walk_normal_attribute(self, it)
    }
    fn visit_attribute_value(&mut self, it: &mut AttributeValue<'a>) -> OxcVec<'a, Statement<'a>> {
        walk_attribute_value(self, it)
    }
    /// We cannot use `visit_directive` because it conflicts with the JS's directive
    fn visit_svelte_directive(&mut self, it: &mut Directive<'a>) -> OxcVec<'a, Statement<'a>> {
        walk_directive(self, it)
    }
    fn visit_animate_directive(
        &mut self,
        it: &mut AnimateDirective<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        walk_animate_directive(self, it)
    }
    fn visit_bind_directive(&mut self, it: &mut BindDirective<'a>) -> OxcVec<'a, Statement<'a>> {
        walk_bind_directive(self, it)
    }
    fn visit_class_directive(&mut self, it: &mut ClassDirective<'a>) -> OxcVec<'a, Statement<'a>> {
        walk_class_directive(self, it)
    }
    fn visit_let_directive(&mut self, it: &mut LetDirective<'a>) -> OxcVec<'a, Statement<'a>> {
        walk_let_directive(self, it)
    }
    fn visit_on_directive(&mut self, it: &mut OnDirective<'a>) -> OxcVec<'a, Statement<'a>> {
        walk_on_directive(self, it)
    }
    fn visit_style_directive(&mut self, it: &mut StyleDirective<'a>) -> OxcVec<'a, Statement<'a>> {
        walk_style_directive(self, it)
    }
    fn visit_transition_directive(
        &mut self,
        it: &mut TransitionDirective<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        walk_transition_directive(self, it)
    }
    fn visit_use_directive(&mut self, it: &mut UseDirective<'a>) -> OxcVec<'a, Statement<'a>> {
        walk_use_directive(self, it)
    }
}

pub mod walk_mut {
    use oxc_syntax::scope::ScopeFlags;

    use super::*;
    pub fn walk_root<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut Root<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        let kind = SvelteAstType::Root;
        visitor.enter_svelte_node(kind);
        // TODO: visit module, comment, options
        if let Some(script) = it.instance.as_mut() {
            visitor.visit_script(script);
        }
        let result = visitor.visit_fragment(&mut it.fragment);
        visitor.leave_svelte_node(kind);
        result
    }

    pub fn walk_fragment<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut Fragment<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        let kind = SvelteAstType::Fragment;
        visitor.enter_svelte_node(kind);
        visitor.enter_scope(ScopeFlags::empty(), &it.scope_id);
        let result = walk_fragment_nodes(visitor, &mut it.nodes);
        visitor.leave_scope();
        visitor.leave_svelte_node(kind);
        result
    }

    pub fn walk_fragment_nodes<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut [FragmentNode<'a>],
    ) -> OxcVec<'a, Statement<'a>> {
        let mut result = visitor.vec([]);
        for node in it.iter_mut() {
            result.append(&mut visitor.visit_fragment_node(node));
        }
        result
    }

    pub fn walk_script<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut Script<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        let kind = SvelteAstType::Script;
        visitor.enter_svelte_node(kind);
        visitor.visit_program(&mut it.content);
        visitor.leave_svelte_node(kind);
        visitor.vec([])
    }

    pub fn walk_fragment_node<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut FragmentNode<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        match it {
            FragmentNode::Text(text) => visitor.visit_text(text),
            FragmentNode::Element(element) => visitor.visit_element(element),
            FragmentNode::Tag(tag) => visitor.visit_tag(tag),
            FragmentNode::Comment(comment) => visitor.visit_comment(comment),
            FragmentNode::Block(block) => visitor.visit_block(block),
        }
    }

    pub fn walk_text<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut Text<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        let kind = SvelteAstType::Text;
        visitor.enter_svelte_node(kind);
        visitor.leave_svelte_node(kind);
        visitor.vec([])
    }

    pub fn walk_element<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut Element<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        match it {
            Element::RegularElement(it) => visitor.visit_regular_element(it),
            Element::SvelteComponent(it) => todo!(),
            Element::SvelteElement(it) => todo!(),
            Element::SvelteBody(it) => todo!(),
            Element::SvelteWindow(it) => todo!(),
            Element::SvelteDocument(it) => todo!(),
            Element::SvelteHead(it) => todo!(),
            Element::SvelteFragment(it) => todo!(),
            Element::SvelteSelf(it) => todo!(),
            Element::TitleElement(it) => todo!(),
            Element::SlotElement(it) => todo!(),
            Element::Component(it) => todo!(),
        }
    }

    pub fn walk_regular_element<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut RegularElement<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        let kind = SvelteAstType::RegularElement;
        let mut result = visitor.vec([]);
        visitor.enter_svelte_node(kind);
        visitor.enter_scope(ScopeFlags::empty(), &it.scope_id);
        result.append(&mut walk_attributes(visitor, &mut it.attributes));
        result.append(&mut visitor.visit_fragment(&mut it.fragment));
        visitor.leave_scope();
        visitor.leave_svelte_node(kind);
        result
    }

    pub fn walk_tag<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut Tag<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        match it {
            Tag::ExpressionTag(it) => visitor.visit_expression_tag(it),
            Tag::HtmlTag(it) => todo!(),
            Tag::DebugTag(it) => todo!(),
            Tag::ConstTag(it) => todo!(),
            Tag::RenderTag(it) => todo!(),
        }
    }

    pub fn walk_expression_tag<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut ExpressionTag<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        let kind = SvelteAstType::ExpressionTag;
        visitor.enter_svelte_node(kind);
        visitor.visit_expression(&mut it.expression);
        visitor.leave_svelte_node(kind);
        visitor.vec([])
    }

    pub fn walk_attributes<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut [Attribute<'a>],
    ) -> OxcVec<'a, Statement<'a>> {
        let mut result = visitor.vec([]);
        for attr in it.iter_mut() {
            result.append(&mut visitor.visit_attribute(attr));
        }
        result
    }

    pub fn walk_attribute<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut Attribute<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        match it {
            Attribute::NormalAttribute(it) => visitor.visit_normal_attribute(it),
            Attribute::SpreadAttribute(it) => todo!(),
            Attribute::Directive(it) => visitor.visit_svelte_directive(it),
        }
    }

    pub fn walk_normal_attribute<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut NormalAttribute<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        let kind = SvelteAstType::NormalAttribute;
        visitor.enter_svelte_node(kind);
        let result = visitor.visit_attribute_value(&mut it.value);
        visitor.leave_svelte_node(kind);
        result
    }

    pub fn walk_attribute_value<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut AttributeValue<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        match it {
            AttributeValue::ExpressionTag(it) => visitor.visit_expression_tag(it),
            AttributeValue::Quoted(it) => todo!(),
            AttributeValue::True => todo!(),
        }
    }

    pub fn walk_directive<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut Directive<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        match it {
            Directive::AnimateDirective(it) => visitor.visit_animate_directive(it),
            Directive::BindDirective(it) => visitor.visit_bind_directive(it),
            Directive::ClassDirective(it) => visitor.visit_class_directive(it),
            Directive::LetDirective(it) => visitor.visit_let_directive(it),
            Directive::OnDirective(it) => visitor.visit_on_directive(it),
            Directive::StyleDirective(it) => visitor.visit_style_directive(it),
            Directive::TransitionDirective(it) => visitor.visit_transition_directive(it),
            Directive::UseDirective(it) => visitor.visit_use_directive(it),
        }
    }

    pub fn walk_animate_directive<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut AnimateDirective<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        let kind = SvelteAstType::AnimateDirective;
        visitor.enter_svelte_node(kind);
        if let Some(expression) = &mut it.expression {
            visitor.visit_expression(expression);
        }
        visitor.leave_svelte_node(kind);
        visitor.vec([])
    }

    pub fn walk_bind_directive<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut BindDirective<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        let kind = SvelteAstType::BindDirective;
        visitor.enter_svelte_node(kind);
        visitor.visit_expression(&mut it.expression);
        visitor.leave_svelte_node(kind);
        visitor.vec([])
    }

    pub fn walk_class_directive<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut ClassDirective<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        let kind = SvelteAstType::ClassDirective;
        visitor.enter_svelte_node(kind);
        visitor.visit_expression(&mut it.expression);
        visitor.leave_svelte_node(kind);
        visitor.vec([])
    }

    pub fn walk_let_directive<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut LetDirective<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        let kind = SvelteAstType::LetDirective;
        visitor.enter_svelte_node(kind);
        if let Some(expression) = &mut it.expression {
            visitor.visit_expression(expression);
        }
        visitor.leave_svelte_node(kind);
        visitor.vec([])
    }

    pub fn walk_on_directive<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut OnDirective<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        let kind = SvelteAstType::OnDirective;
        visitor.enter_svelte_node(kind);
        if let Some(expression) = &mut it.expression {
            visitor.visit_expression(expression);
        }
        visitor.leave_svelte_node(kind);
        visitor.vec([])
    }

    pub fn walk_style_directive<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut StyleDirective<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        let kind = SvelteAstType::StyleDirective;
        visitor.enter_svelte_node(kind);
        visitor.visit_attribute_value(&mut it.value);
        visitor.leave_svelte_node(kind);
        visitor.vec([])
    }

    pub fn walk_transition_directive<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut TransitionDirective<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        let kind = SvelteAstType::TransitionDirective;
        visitor.enter_svelte_node(kind);
        if let Some(expression) = &mut it.expression {
            visitor.visit_expression(expression);
        }
        visitor.leave_svelte_node(kind);
        visitor.vec([])
    }

    pub fn walk_use_directive<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        it: &mut UseDirective<'a>,
    ) -> OxcVec<'a, Statement<'a>> {
        let kind = SvelteAstType::UseDirective;
        visitor.enter_svelte_node(kind);
        if let Some(expression) = &mut it.expression {
            visitor.visit_expression(expression);
        }
        visitor.leave_svelte_node(kind);
        visitor.vec([])
    }
}
