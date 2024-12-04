use crate::ast::{Fragment, RegularElement};

pub trait GetFragmentMut<'a> {
    fn get_fragment_mut(&mut self) -> &mut Fragment<'a>;
}

impl<'a> GetFragmentMut<'a> for Fragment<'a> {
    fn get_fragment_mut(&mut self) -> &mut Fragment<'a> {
        self
    }
}

impl<'a> GetFragmentMut<'a> for RegularElement<'a> {
    fn get_fragment_mut(&mut self) -> &mut Fragment<'a> {
        &mut self.fragment
    }
}
