use oxc_allocator::{Allocator, Vec};
use oxc_ast::AstBuilder as OxcBuilder;

mod js;
mod svelte;

#[derive(Clone, Copy)]
pub struct AstBuilder<'a> {
    allocator: &'a Allocator,
    builder: OxcBuilder<'a>,
}

impl<'a> AstBuilder<'a> {
    pub fn new(allocator: &'a Allocator) -> Self {
        let builder = OxcBuilder::new(allocator);
        Self { allocator, builder }
    }

    pub fn vec<T, const N: usize>(self, array: [T; N]) -> Vec<'a, T> {
        Vec::from_array_in(array, self.allocator)
    }

    pub fn vec_from_iter<T>(self, iter: impl IntoIterator<Item = T>) -> Vec<'a, T> {
        Vec::from_iter_in(iter, self.allocator)
    }
}
