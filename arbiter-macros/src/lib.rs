extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Fields};

#[proc_macro_derive(Behaviors)]
pub fn create_behavior_from_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident; // The name of the enum

    let enum_data = if let Data::Enum(DataEnum { variants, .. }) = input.data {
        variants
    } else {
        // Not an enum, so panic or handle as needed
        panic!("CreateBehaviorFromEnum is only defined for enums");
    };

    let match_arms = enum_data.into_iter().map(|variant| {
        let variant_name = variant.ident;
        let inner_type = if let Fields::Unnamed(fields) = variant.fields {
            fields.unnamed.first().unwrap().ty.clone()
        } else {
            panic!("Expected unnamed fields in enum variant");
        };

        quote! {
            #name::#variant_name(inner) => {
                Box::new(Engine::new(inner))
            }
        }
    });

    let expanded = quote! {

        impl CreateStateMachine for #name {
            fn create_state_machine(self) -> Box<dyn StateMachine> {
                match self {
                    #(#match_arms,)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
