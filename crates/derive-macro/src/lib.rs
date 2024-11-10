use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod ast_tree;

#[proc_macro_derive(AstTree, attributes(ast_tree))]
pub fn ast_tree_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    ast_tree::ast_tree_builder(input)
        .expect("Cannot build ast tree derive")
        .into()
}
