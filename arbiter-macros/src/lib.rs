extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Fields, ItemFn};

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
        let _inner_type = if let Fields::Unnamed(fields) = variant.fields {
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

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    // Ensure the function is named "main" if you want strict naming.
    if input_fn.sig.ident != "main" {
        return syn::Error::new_spanned(
            input_fn.sig.ident,
            "expected the function to be named `main`",
        )
        .to_compile_error()
        .into();
    }

    // Generate CLI setup and the tokio::main attribute application
    let output = quote! {
        #[tokio::main]
        async fn main() {
            // Basic CLI setup (you might want to expand this with actual argument parsing)
            let args: Vec<String> = std::env::args().collect();
            println!("Received arguments: {:?}", args);

            // Original function body execution
            #input_fn
        }
    };

    output.into()
}
