use crate::ast::{Fragment, RegularElement};

pub trait GetName {
    fn name(&self) -> &str;
}

impl<'a> GetName for Fragment<'a> {
    fn name(&self) -> &str {
        "Fragment"
    }
}

impl<'a> GetName for RegularElement<'a> {
    fn name(&self) -> &str {
        self.name
    }
}
