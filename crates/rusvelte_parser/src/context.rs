use crate::Parser;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Context<'a> {
    Block {
        name: &'a str,
    },
    RegularElement {
        name: &'a str,
        auto_closed: bool,
    },
    #[default]
    Root,
}

impl<'a> Context<'a> {
    pub fn root_context() -> Self {
        Self::default()
    }

    pub fn regular_element_context(name: &'a str) -> Self {
        Self::RegularElement {
            name,
            auto_closed: false,
        }
    }

    pub fn block_context(name: &'a str) -> Self {
        Self::Block { name }
    }

    pub fn auto_closed(&self) -> bool {
        if let Self::RegularElement { auto_closed, .. } = self {
            *auto_closed
        } else {
            false
        }
    }

    pub fn name(&self) -> &'a str {
        match self {
            Self::Block { name, .. } => name,
            Self::RegularElement { name, .. } => name,
            Self::Root => "Root",
        }
    }
}

impl<'a> Parser<'a> {
    pub fn is_parent_root(&self) -> bool {
        self.expect_context() == &Context::Root
    }

    pub fn is_parent_regular_element(&self) -> bool {
        matches!(self.expect_context(), Context::RegularElement { .. })
    }

    pub fn parent_name(&self) -> &'a str {
        self.expect_context().name()
    }

    pub fn push_context(&mut self, context: Context<'a>) {
        self.context_stack.push(context);
    }

    pub fn pop_context(&mut self) -> Option<Context<'a>> {
        self.context_stack.pop()
    }

    pub fn close_parent(&mut self) {
        if let Context::RegularElement { auto_closed, .. } = self.expect_context_mut() {
            *auto_closed = true
        }
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
