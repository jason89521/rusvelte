use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, DataEnum, DeriveInput, Error};

pub fn oxc_span_builder(ast: DeriveInput) -> TokenStream {
    match &ast.data {
        syn::Data::Struct(_) => build_struct(&ast),
        syn::Data::Enum(data_enum) => build_enum(&ast, data_enum),
        syn::Data::Union(_) => {
            Error::new(ast.span(), "Only allow struct or enum").to_compile_error()
        }
    }
}

fn build_struct(ast: &syn::DeriveInput) -> TokenStream {
    let (impl_generics, ty_generics, ..) = ast.generics.split_for_impl();
    let name = &ast.ident;
    let gen = quote! {
        impl #impl_generics oxc_span::GetSpan for #name #ty_generics {
            fn span(&self) -> oxc_span::Span {
                self.span
            }
        }
    };
    gen
}

fn build_enum(ast: &DeriveInput, data_enum: &DataEnum) -> TokenStream {
    let (impl_generics, ty_generics, ..) = ast.generics.split_for_impl();
    let enum_ident = &ast.ident;
    let branches = data_enum.variants.iter().map(|var| {
        let variant_ident = &var.ident;
        quote! {
          #enum_ident::#variant_ident(expr) => expr.span()
        }
    });

    quote! {
      impl #impl_generics oxc_span::GetSpan for #enum_ident #ty_generics {
        fn span(&self) -> oxc_span::Span {
          match self {
            #(#branches),*
          }
        }
      }
    }
}
