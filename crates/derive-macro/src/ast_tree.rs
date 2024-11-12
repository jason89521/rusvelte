use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Fields, Ident, LitStr, Meta};

pub fn ast_tree_builder(input: DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;
    let type_value = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("ast_tree"))
        .and_then(|attr| {
            let mut type_value = name.to_string();
            if let Meta::List(meta_list) = &attr.meta {
                let _ = meta_list.parse_nested_meta(|meta| {
                    if meta.path.is_ident("type") {
                        let value = meta.value()?;
                        let s: LitStr = value.parse()?;
                        type_value = s.value();
                        Ok(())
                    } else {
                        Err(meta.error("Unknown attribute"))
                    }
                });
            };
            Some(type_value)
        })
        .unwrap_or(name.to_string());

    let body = match &input.data {
        Data::Struct(data_struct) => build_struct(&type_value, data_struct),
        Data::Enum(data_enum) => build_enum(name, data_enum),
        _ => {
            return Err(syn::Error::new(
                input.span(),
                "Only support struct and enum",
            ))
        }
    }?;

    Ok(quote! {
        impl serde::Serialize for #name<'_> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                #body
            }
        }
    })
}

fn build_struct(type_value: &str, data_struct: &DataStruct) -> syn::Result<TokenStream> {
    let fields = if let Fields::Named(fields_named) = &data_struct.fields {
        fields_named.named.iter().collect::<Vec<_>>()
    } else {
        return Err(syn::Error::new(
            data_struct.fields.span(),
            "AstTree can only be derived for structs with named fields",
        ));
    };
    let mut serialization_tokens = vec![];
    serialization_tokens.push(quote! {
      map.serialize_entry("type", #type_value)?;
    });
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        if field_name == "span" {
            serialization_tokens.push(quote! {
                let Span { start, end, .. } = self.span;
                map.serialize_entry("start", &start)?;
                map.serialize_entry("end", &end)?;
            });
        } else {
            let field_name_str = field_name.to_string();
            serialization_tokens.push(quote! {
              map.serialize_entry(#field_name_str, &self.#field_name)?;
            });
        }
    }

    Ok(quote! {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        #(#serialization_tokens)*
        map.end()
    })
}

fn build_enum(name: &Ident, data_enum: &DataEnum) -> syn::Result<TokenStream> {
    let match_arms = data_enum
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            match &variant.fields {
                Fields::Unit => {
                    let value = match variant_ident.to_string().to_lowercase().as_ref() {
                        "true" => quote! {
                            &true
                        },
                        v => quote! { #v },
                    };
                    Ok(quote! {
                        #name::#variant_ident => Serialize::serialize(#value, serializer)
                    })
                }
                Fields::Unnamed(fields_unnamed) if fields_unnamed.unnamed.len() == 1 => {
                    Ok(quote! {
                        #name::#variant_ident(x) => Serialize::serialize(x, serializer)
                    })
                }
                _ => {
                    return Err(syn::Error::new(
                        variant.span(),
                        "Only support unnamed field with exactly one field",
                    ))
                }
            }
        })
        .filter_map(|result| result.ok());

    Ok(quote! {
        use serde::ser::Serialize;
        match self {
            #(#match_arms,)*
        }
    })
}
