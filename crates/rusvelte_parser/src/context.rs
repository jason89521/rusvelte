use crate::{
    constants::{
        SVELTE_BODY_TAG, SVELTE_COMPONENT_TAG, SVELTE_DOCUMENT_TAG, SVELTE_ELEMENT_TAG,
        SVELTE_FRAGMENT_TAG, SVELTE_HEAD_TAG, SVELTE_OPTIONS_TAG, SVELTE_SELF_TAG,
        SVELTE_WINDOW_TAG,
    },
    regex_pattern::REGEX_VALID_COMPONENT_NAME,
    Parser,
};

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
    SvelteComponent {
        name: &'a str,
    },
    SvelteElement {
        name: &'a str,
    },
    SvelteBody {
        name: &'a str,
    },
    SvelteWindow {
        name: &'a str,
    },
    SvelteDocument {
        name: &'a str,
    },
    SvelteHead {
        name: &'a str,
    },
    SvelteOptions {
        name: &'a str,
    },
    SvelteFragment {
        name: &'a str,
    },
    SvelteSelf {
        name: &'a str,
    },
    TitleElement {
        name: &'a str,
    },
    SlotElement {
        name: &'a str,
    },
    Component {
        name: &'a str,
    },
}

impl<'a> Context<'a> {
    pub fn root_context() -> Self {
        Self::default()
    }

    pub fn block_context(name: &'a str) -> Self {
        Self::Block { name }
    }

    pub fn element_context(name: &'a str) -> Self {
        match name {
            SVELTE_COMPONENT_TAG => Self::SvelteComponent { name },
            SVELTE_ELEMENT_TAG => Self::SvelteElement { name },
            SVELTE_HEAD_TAG => Self::SvelteHead { name },
            SVELTE_OPTIONS_TAG => Self::SvelteOptions { name },
            SVELTE_WINDOW_TAG => Self::SvelteWindow { name },
            SVELTE_DOCUMENT_TAG => Self::SvelteDocument { name },
            SVELTE_BODY_TAG => Self::SvelteBody { name },
            SVELTE_SELF_TAG => Self::SvelteSelf { name },
            SVELTE_FRAGMENT_TAG => Self::SvelteFragment { name },
            _ if REGEX_VALID_COMPONENT_NAME.is_match(name) => Self::Component { name },
            "title" => Self::TitleElement { name },
            "slot" => Self::SlotElement { name },
            _ => Self::RegularElement {
                name,
                auto_closed: false,
            },
        }
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
            Self::Block { name } => name,
            Self::RegularElement { name, .. } => name,
            Self::Root => "Root",
            Self::SvelteComponent { name } => name,
            Self::SvelteElement { name } => name,
            Self::SvelteBody { name } => name,
            Self::SvelteWindow { name } => name,
            Self::SvelteDocument { name } => name,
            Self::SvelteHead { name } => name,
            Self::SvelteOptions { name } => name,
            Self::SvelteFragment { name } => name,
            Self::SvelteSelf { name } => name,
            Self::TitleElement { name } => name,
            Self::SlotElement { name } => name,
            Self::Component { name } => name,
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
