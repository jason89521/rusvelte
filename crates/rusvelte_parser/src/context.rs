use crate::Parser;

#[derive(Debug, Clone, Default)]
pub struct Context<'a> {
    kind: ParentKind,
    name: &'a str,
    closed_at: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParentKind {
    RegularElement,
    Root,
}

impl Default for ParentKind {
    fn default() -> Self {
        ParentKind::Root
    }
}

impl<'a> Context<'a> {
    pub fn root_context() -> Self {
        Self {
            kind: ParentKind::Root,
            ..Default::default()
        }
    }

    pub fn regular_element_context(name: &'a str) -> Self {
        Self {
            kind: ParentKind::RegularElement,
            name,
            ..Default::default()
        }
    }

    pub fn closed_at(&self) -> Option<u32> {
        self.closed_at
    }
}

impl<'a> Parser<'a> {
    pub fn is_parent_root(&self) -> bool {
        self.expect_context().kind == ParentKind::Root
    }

    pub fn parent_kind(&self) -> ParentKind {
        self.expect_context().kind
    }

    pub fn parent_name(&self) -> &'a str {
        &self.expect_context().name
    }

    pub fn push_context(&mut self, context: Context<'a>) {
        self.context_stack.push(context);
    }

    pub fn pop_context(&mut self) -> Option<Context<'a>> {
        self.context_stack.pop()
    }

    pub fn set_parent_closed_at(&mut self, index: u32) {
        self.expect_context_mut().closed_at = Some(index)
    }

    fn expect_context(&self) -> &Context<'a> {
        self.context_stack.last().expect("Expected context.")
    }

    fn expect_context_mut(&mut self) -> &mut Context<'a> {
        self.context_stack
            .last_mut()
            .expect("Expected mut context.")
    }
}
