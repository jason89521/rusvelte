use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod ast_tree;
mod oxc_span;

#[proc_macro_derive(AstTree, attributes(ast_tree))]
pub fn ast_tree_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    ast_tree::ast_tree_builder(input)
        .expect("Cannot build ast tree derive")
        .into()
}

#[proc_macro_derive(OxcSpan)]
pub fn oxc_span_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    oxc_span::oxc_span_builder(input).into()
}
