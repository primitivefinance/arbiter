extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, DeriveInput, Fields, FieldsNamed, GenericArgument, Ident, PathArguments,
    Type, TypePath,
};

#[proc_macro_derive(Deploy)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;
    let new_struct_name = Ident::new(&format!("{}Deployed", struct_name), struct_name.span());

    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => named,
        _ => panic!("Only named fields are supported"),
    };

    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let new_field_types: Vec<_> = fields
        .iter()
        .map(|f| match &f.ty {
            Type::Path(TypePath { path, .. }) => {
                if let Some(segment) = path.segments.last() {
                    if let PathArguments::AngleBracketed(angle_bracketed_args) = &segment.arguments
                    {
                        if let Some(GenericArgument::Type(Type::Path(TypePath { path, .. }))) =
                            angle_bracketed_args.args.last()
                        {
                            return quote! { #path };
                        }
                    }
                }
                quote! { #f }
            }
            _ => quote! { #f },
        })
        .collect();
    let middleware_type = if let Type::Path(tp) = &fields.iter().next().unwrap().ty {
        if let Some(segment) = tp.path.segments.first() {
            if let PathArguments::AngleBracketed(angle_bracketed_args) = &segment.arguments {
                if let Some(GenericArgument::Type(Type::Path(type_path))) =
                    angle_bracketed_args.args.iter().nth(1)
                {
                    Some(&type_path.path)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    // Generate the code for new struct and impl
    let expanded = quote! {
        #[derive(Debug)]
        pub struct #new_struct_name {
            #( #field_names: #new_field_types ),*
        }

        impl #struct_name {
            pub async fn deploy(self) -> Result<#new_struct_name, ContractError<#middleware_type>> {
                Ok(#new_struct_name {
                    #(
                        #field_names: self.#field_names.send().await.unwrap(),
                    )*
                })
            }
        }
    };

    TokenStream::from(expanded)
}
