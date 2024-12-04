use crate::ast::{Fragment, RegularElement};

pub trait GetName {
    fn name(&self) -> &str;
}

impl GetName for Fragment<'_> {
    fn name(&self) -> &str {
        "Fragment"
    }
}

impl GetName for RegularElement<'_> {
    fn name(&self) -> &str {
        self.name
    }
}
