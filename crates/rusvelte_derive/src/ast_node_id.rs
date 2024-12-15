use proc_macro2::TokenStream;
use syn::{spanned::Spanned, DeriveInput, Error};

pub fn ast_node_id(ast: DeriveInput) -> TokenStream {
    let syn::Data::Struct(data_struct) = &ast.data else {
        return Error::new(ast.span(), "Only allow struct").to_compile_error();
    };

    unimplemented!()
}
