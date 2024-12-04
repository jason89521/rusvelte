use oxc_allocator::IntoIn;
use rusvelte_ast::{
    ast::{Block, Element, FragmentNode, Tag},
    ast_kind::SvelteAstType,
    traits::{get_ast_type::GetAstType, get_name::GetName, take_fragment_nodes::TakeFragmentNodes},
};
use rusvelte_utils::regex_pattern::{REGEX_ENDS_WITH_WHITESPACES, REGEX_NOT_WHITESPACE};

use crate::Transformer;

pub struct CleanNodesReturn<'a> {
    pub hoisted: Vec<FragmentNode<'a>>,
    pub trimmed: Vec<FragmentNode<'a>>,
    pub is_standalone: bool,
    pub is_text_first: bool,
}

impl<'a> Transformer<'a> {
    pub fn clean_nodes<T>(&self, parent: &mut T) -> CleanNodesReturn<'a>
    where
        T: TakeFragmentNodes<'a> + GetAstType + GetName,
    {
        let mut hoisted = vec![];
        let mut regular = vec![];
        for node in parent.take_fragment_nodes() {
            match &node {
                FragmentNode::Element(element)
                    if matches!(
                        element.as_ref(),
                        Element::SvelteBody(_)
                            | Element::SvelteWindow(_)
                            | Element::SvelteDocument(_)
                            | Element::SvelteHead(_)
                            | Element::TitleElement(_)
                    ) =>
                {
                    hoisted.push(node);
                }
                FragmentNode::Tag(tag) if matches!(tag, Tag::ConstTag(_) | Tag::DebugTag(_)) => {
                    hoisted.push(node);
                }
                FragmentNode::Comment(_) => todo!("should check preserve_comment option"),
                FragmentNode::Block(block) if matches!(block, Block::SnippetBlock(_)) => {
                    hoisted.push(node);
                }
                _ => {
                    regular.push(node);
                }
            }
        }

        // TODO: check option.preserve_whitespace
        let first_not_whitespace_node_pos = regular
            .iter()
            .position(|node| {
                if let FragmentNode::Text(text) = node {
                    REGEX_NOT_WHITESPACE.is_match(&text.data)
                } else {
                    true
                }
            })
            .unwrap_or(regular.len());
        let last_not_whitespace_node_pos = regular
            .iter()
            .rposition(|node| {
                if let FragmentNode::Text(text) = node {
                    REGEX_NOT_WHITESPACE.is_match(&text.data)
                } else {
                    true
                }
            })
            .unwrap_or(0);
        if first_not_whitespace_node_pos <= last_not_whitespace_node_pos {
            regular = regular
                .into_iter()
                .skip(first_not_whitespace_node_pos)
                .take(last_not_whitespace_node_pos - first_not_whitespace_node_pos + 1)
                .collect();
        }

        if let Some(text) = regular.first_mut().and_then(FragmentNode::as_text_mut) {
            text.raw = REGEX_ENDS_WITH_WHITESPACES
                .replace(&text.raw, "")
                .into_in(self.allocator);
            text.data = REGEX_ENDS_WITH_WHITESPACES
                .replace(&text.data, "")
                .to_string()
                .into();
        }

        if let Some(text) = regular.last_mut().and_then(FragmentNode::as_text_mut) {
            text.raw = REGEX_ENDS_WITH_WHITESPACES
                .replace(&text.raw, "")
                .into_in(self.allocator);
            text.data = REGEX_ENDS_WITH_WHITESPACES
                .replace(&text.data, "")
                .to_string()
                .into();
        }

        // TODO: check namespace
        let can_remove_entirely = parent.ast_type() == SvelteAstType::RegularElement
            && matches!(
                parent.name(),
                "select" | "tr" | "table" | "tbody" | "thead" | "tfoot" | "colgroup" | "datalist"
            );
        let mut trimmed = vec![];
        regular.reverse();
        while let Some(mut node) = regular.pop() {
            let prev = trimmed.last();
            let next = regular.first();
            if let FragmentNode::Text(text) = &mut node {
                if !prev.map_or(false, FragmentNode::is_expression_tag) {
                    let whitespace = if prev.map_or(false, |node| {
                        if let FragmentNode::Text(text) = node {
                            REGEX_ENDS_WITH_WHITESPACES.is_match(&text.data)
                        } else {
                            false
                        }
                    }) {
                        ""
                    } else {
                        " "
                    };
                    text.data = REGEX_ENDS_WITH_WHITESPACES
                        .replace(&text.data, whitespace)
                        .to_string()
                        .into();
                    text.raw = REGEX_ENDS_WITH_WHITESPACES
                        .replace(&text.raw, whitespace)
                        .into_in(self.allocator);
                }
                if next.map_or(true, FragmentNode::is_expression_tag) {
                    text.data = REGEX_ENDS_WITH_WHITESPACES
                        .replace(&text.data, " ")
                        .to_string()
                        .into();
                    text.raw = REGEX_ENDS_WITH_WHITESPACES
                        .replace(&text.raw, " ")
                        .into_in(self.allocator);
                }
                if !text.data.is_empty() && (text.data != " " || !can_remove_entirely) {
                    trimmed.push(node);
                }
            } else {
                trimmed.push(node);
            }
        }

        // TODO: handle a long script tag case

        let first = trimmed.first();
        // TODO: align with the original code
        // is_standalone:
        //   trimmed.length === 1 &&
        //   ((first.type === 'RenderTag' && !first.metadata.dynamic) ||
        //     (first.type === 'Component' &&
        //       !state.options.hmr &&
        //       !first.metadata.dynamic &&
        //       !first.attributes.some(
        //         (attribute) => attribute.type === 'Attribute' && attribute.name.startsWith('--')
        //       ))),
        let is_standalone = if let Some(first) = first {
            trimmed.len() == 1
                && ((first.ast_type() == SvelteAstType::RenderTag/* && !first.metadata.dynamic */)
                    || (first.ast_type() == SvelteAstType::Component/* &&
                    !state.options.hmr &&
                    !first.metadata.dynamic &&
                    !first.attributes.some(
                      (attribute) => attribute.type === 'Attribute' && attribute.name.startsWith('--')
                    ) */))
        } else {
            false
        };

        let is_text_first = matches!(
            parent.ast_type(),
            SvelteAstType::Fragment
                | SvelteAstType::SnippetBlock
                | SvelteAstType::EachBlock
                | SvelteAstType::SvelteComponent
                | SvelteAstType::Component
                | SvelteAstType::SvelteSelf
        ) && first.map_or(false, |node| {
            matches!(
                node.ast_type(),
                SvelteAstType::Text | SvelteAstType::ExpressionTag
            )
        });

        CleanNodesReturn {
            hoisted,
            trimmed,
            is_standalone,
            is_text_first,
        }
    }
}
