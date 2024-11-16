use crate::Parser;

#[derive(Debug, Clone, Default)]
pub struct Context {
    is_parent_root: bool,
}

impl Context {
    pub fn root_context() -> Self {
        Self {
            is_parent_root: true,
            ..Default::default()
        }
    }
}

impl Context {}

impl Parser<'_> {
    pub fn is_parent_root(&self) -> bool {
        self.peek_context().map_or(false, |c| c.is_parent_root)
    }

    pub fn peek_context(&self) -> Option<&Context> {
        self.context_stack.last()
    }

    pub fn push_context(&mut self, context: Context) {
        self.context_stack.push(context);
    }

    pub fn pop_context(&mut self) -> Option<Context> {
        self.context_stack.pop()
    }
}
