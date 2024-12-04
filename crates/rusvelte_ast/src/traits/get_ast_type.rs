// TODO: use codegen to generate this trait
use crate::{ast::*, ast_kind::SvelteAstType};

pub trait GetAstType {
    fn ast_type(&self) -> SvelteAstType;
}

impl GetAstType for Fragment<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::Fragment
    }
}

impl GetAstType for FragmentNode<'_> {
    fn ast_type(&self) -> SvelteAstType {
        match self {
            FragmentNode::Text(node) => node.ast_type(),
            FragmentNode::Element(node) => node.ast_type(),
            FragmentNode::Tag(node) => node.ast_type(),
            FragmentNode::Comment(node) => node.ast_type(),
            FragmentNode::Block(node) => node.ast_type(),
        }
    }
}

impl GetAstType for Text<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::Text
    }
}

impl GetAstType for Element<'_> {
    fn ast_type(&self) -> SvelteAstType {
        match self {
            Element::RegularElement(elem) => elem.ast_type(),
            Element::SvelteComponent(elem) => elem.ast_type(),
            Element::SvelteElement(elem) => elem.ast_type(),
            Element::SvelteBody(elem) => elem.ast_type(),
            Element::SvelteWindow(elem) => elem.ast_type(),
            Element::SvelteDocument(elem) => elem.ast_type(),
            Element::SvelteHead(elem) => elem.ast_type(),
            Element::SvelteFragment(elem) => elem.ast_type(),
            Element::SvelteSelf(elem) => elem.ast_type(),
            Element::TitleElement(elem) => elem.ast_type(),
            Element::SlotElement(elem) => elem.ast_type(),
            Element::Component(elem) => elem.ast_type(),
        }
    }
}

impl GetAstType for RegularElement<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::RegularElement
    }
}

impl GetAstType for SvelteComponent<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::SvelteComponent
    }
}

impl GetAstType for SvelteElement<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::SvelteElement
    }
}

impl GetAstType for SvelteBody<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::SvelteBody
    }
}

impl GetAstType for SvelteWindow<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::SvelteWindow
    }
}

impl GetAstType for SvelteDocument<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::SvelteDocument
    }
}

impl GetAstType for SvelteHead<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::SvelteHead
    }
}

impl GetAstType for SvelteFragment<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::SvelteFragment
    }
}

impl GetAstType for SvelteSelf<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::SvelteSelf
    }
}

impl GetAstType for TitleElement<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::TitleElement
    }
}

impl GetAstType for SlotElement<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::SlotElement
    }
}

impl GetAstType for Component<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::Component
    }
}

impl GetAstType for Tag<'_> {
    fn ast_type(&self) -> SvelteAstType {
        match self {
            Tag::ExpressionTag(tag) => tag.ast_type(),
            Tag::HtmlTag(tag) => tag.ast_type(),
            Tag::DebugTag(tag) => tag.ast_type(),
            Tag::ConstTag(tag) => tag.ast_type(),
            Tag::RenderTag(tag) => tag.ast_type(),
        }
    }
}

impl GetAstType for ExpressionTag<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::ExpressionTag
    }
}

impl GetAstType for HtmlTag<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::HtmlTag
    }
}

impl GetAstType for DebugTag<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::DebugTag
    }
}

impl GetAstType for ConstTag<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::ConstTag
    }
}

impl GetAstType for RenderTag<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::RenderTag
    }
}

impl GetAstType for Comment<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::Comment
    }
}

impl GetAstType for Block<'_> {
    fn ast_type(&self) -> SvelteAstType {
        match self {
            Block::IfBlock(block) => block.ast_type(),
            Block::EachBlock(block) => block.ast_type(),
            Block::AwaitBlock(block) => block.ast_type(),
            Block::KeyBlock(block) => block.ast_type(),
            Block::SnippetBlock(block) => block.ast_type(),
        }
    }
}

impl GetAstType for IfBlock<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::IfBlock
    }
}

impl GetAstType for EachBlock<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::EachBlock
    }
}

impl GetAstType for AwaitBlock<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::AwaitBlock
    }
}

impl GetAstType for KeyBlock<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::KeyBlock
    }
}

impl GetAstType for SnippetBlock<'_> {
    fn ast_type(&self) -> SvelteAstType {
        SvelteAstType::SnippetBlock
    }
}
