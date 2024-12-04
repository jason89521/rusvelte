use crate::ast::{Fragment, FragmentNode, RegularElement};

pub trait TakeFragmentNodes<'a> {
    fn take_fragment_nodes(&mut self) -> Vec<FragmentNode<'a>>;
}

impl<'a> TakeFragmentNodes<'a> for Fragment<'a> {
    fn take_fragment_nodes(&mut self) -> Vec<FragmentNode<'a>> {
        std::mem::take(&mut self.nodes)
    }
}

impl<'a> TakeFragmentNodes<'a> for RegularElement<'a> {
    fn take_fragment_nodes(&mut self) -> Vec<FragmentNode<'a>> {
        std::mem::take(&mut self.fragment.nodes)
    }
}
