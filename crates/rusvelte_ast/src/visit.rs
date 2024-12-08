#![allow(unused_variables)]
pub use oxc_ast::Visit as JsVisit;

use crate::{ast::*, ast_kind::SvelteAstKind};
use walk::*;

pub use oxc_ast::VisitMut as JsVisitMut;

pub trait Visit<'a>: JsVisit<'a> {
    fn enter_svelte_node(&mut self, kind: SvelteAstKind<'a>) {}
    fn leave_svelte_node(&mut self, kind: SvelteAstKind<'a>) {}
    fn enter_svelte_scope(&mut self) {}
    fn leave_svelte_scope(&mut self) {}

    fn visit_root(&mut self, it: &Root<'a>) {
        walk_root(self, it);
    }
    fn visit_fragment(&mut self, it: &Fragment<'a>) {
        walk_fragment(self, it);
    }
    fn visit_script(&mut self, it: &Script<'a>) {
        walk_script(self, it);
    }
    fn visit_fragment_node(&mut self, it: &FragmentNode<'a>) {
        walk_fragment_node(self, it);
    }
    fn visit_text(&mut self, it: &Text<'a>) {
        walk_text(self, it);
    }
    fn visit_element(&mut self, it: &Element<'a>) {
        walk_element(self, it);
    }
    fn visit_regular_element(&mut self, it: &RegularElement<'a>) {
        walk_regular_element(self, it);
    }
    fn visit_tag(&mut self, it: &Tag<'a>) {
        walk_tag(self, it);
    }
    fn visit_comment(&mut self, it: &Comment<'a>) {}
    fn visit_block(&mut self, it: &Block<'a>) {}
    fn visit_expression_tag(&mut self, it: &ExpressionTag<'a>) {
        walk_expression_tag(self, it);
    }
    fn visit_attributes(&mut self, it: &[Attribute<'a>]) {
        walk_attributes(self, it);
    }
    fn visit_attribute(&mut self, it: &Attribute<'a>) {
        walk_attribute(self, it);
    }
    fn visit_normal_attribute(&mut self, it: &NormalAttribute<'a>) {
        walk_normal_attribute(self, it);
    }
    fn visit_attribute_value(&mut self, it: &AttributeValue<'a>) {
        walk_attribute_value(self, it)
    }
    /// We cannot use `visit_directive` because it conflicts with the JS's directive
    fn visit_svelte_directive(&mut self, it: &Directive<'a>) {
        walk_directive(self, it);
    }
    fn visit_animate_directive(&mut self, it: &AnimateDirective<'a>) {
        walk_animate_directive(self, it);
    }
    fn visit_bind_directive(&mut self, it: &BindDirective<'a>) {
        walk_bind_directive(self, it);
    }
    fn visit_class_directive(&mut self, it: &ClassDirective<'a>) {
        walk_class_directive(self, it);
    }
    fn visit_let_directive(&mut self, it: &LetDirective<'a>) {
        walk_let_directive(self, it);
    }
    fn visit_on_directive(&mut self, it: &OnDirective<'a>) {
        walk_on_directive(self, it);
    }
    fn visit_style_directive(&mut self, it: &StyleDirective<'a>) {
        walk_style_directive(self, it);
    }
    fn visit_transition_directive(&mut self, it: &TransitionDirective<'a>) {
        walk_transition_directive(self, it);
    }
    fn visit_use_directive(&mut self, it: &UseDirective<'a>) {
        walk_use_directive(self, it);
    }
}

pub mod walk {
    use super::*;
    pub fn walk_root<'a, V: Visit<'a>>(visitor: &mut V, it: &Root<'a>) {
        let kind = SvelteAstKind::Root(visitor.alloc(it));
        visitor.enter_svelte_node(kind);
        // TODO: visit module, comment, options
        if let Some(script) = it.instance.as_ref() {
            visitor.visit_script(script);
        }
        visitor.visit_fragment(&it.fragment);
        visitor.leave_svelte_node(kind);
    }

    pub fn walk_fragment<'a, V: Visit<'a>>(visitor: &mut V, it: &Fragment<'a>) {
        let kind = SvelteAstKind::Fragment(visitor.alloc(it));
        visitor.enter_svelte_node(kind);
        for node in it.nodes.iter() {
            visitor.visit_fragment_node(node);
        }
        visitor.leave_svelte_node(kind);
    }

    pub fn walk_script<'a, V: Visit<'a>>(visitor: &mut V, it: &Script<'a>) {
        let kind = SvelteAstKind::Script(visitor.alloc(it));
        visitor.enter_svelte_node(kind);
        visitor.visit_program(&it.content);
        visitor.leave_svelte_node(kind);
    }

    pub fn walk_fragment_node<'a, V: Visit<'a>>(visitor: &mut V, it: &FragmentNode<'a>) {
        match it {
            FragmentNode::Text(text) => visitor.visit_text(text),
            FragmentNode::Element(element) => visitor.visit_element(element),
            FragmentNode::Tag(tag) => visitor.visit_tag(tag),
            FragmentNode::Comment(comment) => visitor.visit_comment(comment),
            FragmentNode::Block(block) => visitor.visit_block(block),
        }
    }

    pub fn walk_text<'a, V: Visit<'a>>(visitor: &mut V, it: &Text<'a>) {
        let kind = SvelteAstKind::Text(visitor.alloc(it));
        visitor.enter_svelte_node(kind);
        visitor.leave_svelte_node(kind);
    }

    pub fn walk_element<'a, V: Visit<'a>>(visitor: &mut V, it: &Element<'a>) {
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

    pub fn walk_regular_element<'a, V: Visit<'a>>(visitor: &mut V, it: &RegularElement<'a>) {
        let kind = SvelteAstKind::RegularElement(visitor.alloc(it));
        visitor.enter_svelte_node(kind);
        visitor.visit_fragment(&it.fragment);
        visitor.leave_svelte_node(kind);
    }

    pub fn walk_tag<'a, V: Visit<'a>>(visitor: &mut V, it: &Tag<'a>) {
        match it {
            Tag::ExpressionTag(it) => visitor.visit_expression_tag(it),
            Tag::HtmlTag(it) => todo!(),
            Tag::DebugTag(it) => todo!(),
            Tag::ConstTag(it) => todo!(),
            Tag::RenderTag(it) => todo!(),
        }
    }

    pub fn walk_expression_tag<'a, V: Visit<'a>>(visitor: &mut V, it: &ExpressionTag<'a>) {
        let kind = SvelteAstKind::ExpressionTag(visitor.alloc(it));
        visitor.enter_svelte_node(kind);
        visitor.visit_expression(&it.expression);
        visitor.leave_svelte_node(kind);
    }

    pub fn walk_attributes<'a, V: Visit<'a>>(visitor: &mut V, it: &[Attribute<'a>]) {
        for attr in it.iter() {
            visitor.visit_attribute(attr);
        }
    }

    pub fn walk_attribute<'a, V: Visit<'a>>(visitor: &mut V, it: &Attribute<'a>) {
        match it {
            Attribute::NormalAttribute(it) => visitor.visit_normal_attribute(it),
            Attribute::SpreadAttribute(it) => todo!(),
            Attribute::Directive(it) => visitor.visit_svelte_directive(it),
        }
    }

    pub fn walk_normal_attribute<'a, V: Visit<'a>>(visitor: &mut V, it: &NormalAttribute<'a>) {
        let kind = SvelteAstKind::NormalAttribute(visitor.alloc(it));
        visitor.enter_svelte_node(kind);
        visitor.visit_attribute_value(&it.value);
        visitor.leave_svelte_node(kind);
    }

    pub fn walk_attribute_value<'a, V: Visit<'a>>(visitor: &mut V, it: &AttributeValue<'a>) {
        match it {
            AttributeValue::ExpressionTag(it) => visitor.visit_expression_tag(it),
            AttributeValue::Quoted(it) => todo!(),
            AttributeValue::True => todo!(),
        }
    }

    pub fn walk_directive<'a, V: Visit<'a>>(visitor: &mut V, it: &Directive<'a>) {
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

    pub fn walk_animate_directive<'a, V: Visit<'a>>(visitor: &mut V, it: &AnimateDirective<'a>) {
        let kind = SvelteAstKind::AnimateDirective(visitor.alloc(it));
        visitor.enter_svelte_node(kind);
        if let Some(expression) = &it.expression {
            visitor.visit_expression(expression);
        }
        visitor.leave_svelte_node(kind);
    }

    pub fn walk_bind_directive<'a, V: Visit<'a>>(visitor: &mut V, it: &BindDirective<'a>) {
        let kind = SvelteAstKind::BindDirective(visitor.alloc(it));
        visitor.enter_svelte_node(kind);
        visitor.visit_expression(&it.expression);
        visitor.leave_svelte_node(kind);
    }

    pub fn walk_class_directive<'a, V: Visit<'a>>(visitor: &mut V, it: &ClassDirective<'a>) {
        let kind = SvelteAstKind::ClassDirective(visitor.alloc(it));
        visitor.enter_svelte_node(kind);
        visitor.visit_expression(&it.expression);
        visitor.leave_svelte_node(kind);
    }

    pub fn walk_let_directive<'a, V: Visit<'a>>(visitor: &mut V, it: &LetDirective<'a>) {
        let kind = SvelteAstKind::LetDirective(visitor.alloc(it));
        visitor.enter_svelte_node(kind);
        if let Some(expression) = &it.expression {
            visitor.visit_expression(expression);
        }
        visitor.leave_svelte_node(kind);
    }

    pub fn walk_on_directive<'a, V: Visit<'a>>(visitor: &mut V, it: &OnDirective<'a>) {
        let kind = SvelteAstKind::OnDirective(visitor.alloc(it));
        visitor.enter_svelte_node(kind);
        if let Some(expression) = &it.expression {
            visitor.visit_expression(expression);
        }
        visitor.leave_svelte_node(kind);
    }

    pub fn walk_style_directive<'a, V: Visit<'a>>(visitor: &mut V, it: &StyleDirective<'a>) {
        let kind = SvelteAstKind::StyleDirective(visitor.alloc(it));
        visitor.enter_svelte_node(kind);
        visitor.visit_attribute_value(&it.value);
        visitor.leave_svelte_node(kind);
    }

    pub fn walk_transition_directive<'a, V: Visit<'a>>(
        visitor: &mut V,
        it: &TransitionDirective<'a>,
    ) {
        let kind = SvelteAstKind::TransitionDirective(visitor.alloc(it));
        visitor.enter_svelte_node(kind);
        if let Some(expression) = &it.expression {
            visitor.visit_expression(expression);
        }
        visitor.leave_svelte_node(kind);
    }

    pub fn walk_use_directive<'a, V: Visit<'a>>(visitor: &mut V, it: &UseDirective<'a>) {
        let kind = SvelteAstKind::UseDirective(visitor.alloc(it));
        visitor.enter_svelte_node(kind);
        if let Some(expression) = &it.expression {
            visitor.visit_expression(expression);
        }
        visitor.leave_svelte_node(kind);
    }
}
