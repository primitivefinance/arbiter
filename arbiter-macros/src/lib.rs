extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Data, DataEnum, DeriveInput, Fields, Ident, ItemFn, Lit,
    Result as ParseResult, Type,
};

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

// Define a custom struct to parse our specific macro attributes
struct MacroArgs {
    name: String,
    about: String,
    behaviors: Type,
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut name = String::new();
        let mut about = String::new();
        let mut behaviors: Option<Type> = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(Ident) {
                let ident: Ident = input.parse()?;
                let _eq_token: syn::token::Eq = input.parse()?;
                if ident == "name" {
                    if let Lit::Str(lit_str) = input.parse()? {
                        name = lit_str.value();
                    }
                } else if ident == "about" {
                    if let Lit::Str(lit_str) = input.parse()? {
                        about = lit_str.value();
                    }
                } else if ident == "behaviors" {
                    behaviors = Some(input.parse()?);
                } else {
                    return Err(lookahead.error());
                }
            } else {
                return Err(lookahead.error());
            }

            // Parse `,`
            if !input.is_empty() {
                let _: syn::token::Comma = input.parse()?;
            }
        }

        let behaviors = behaviors.ok_or_else(|| input.error("missing behaviors"))?;

        Ok(MacroArgs {
            name,
            about,
            behaviors,
        })
    }
}

#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the macro attributes
    let args: MacroArgs = syn::parse(attr).expect("Failed to parse macro arguments");

    let name = args.name;
    let about = args.about;
    let behaviors = args.behaviors;

    // Parse the input TokenStream for the function
    let _input_fn = parse_macro_input!(item as ItemFn);

    // Generate the CLI and logging setup code with async and tokio::main
    let expanded = quote! {
        #[tokio::main]
        #[allow(unused_must_use)]
        async fn main() -> Result<(), Box<dyn std::error::Error>> {
            use clap::{Parser, Subcommand, ArgAction, CommandFactory};
            use tracing::Level;
            use arbiter_engine::world::World;

            #[derive(Parser)]
            #[clap(name = #name)]
            #[clap(version = env!("CARGO_PKG_VERSION"))]
            #[clap(about = #about, long_about = None)]
            #[clap(author)]
            struct Args {
                #[command(subcommand)]
                command: Option<Commands>,

                #[clap(short, long, global = true, required = false, action = ArgAction::Count, value_parser = clap::value_parser!(u8))]
                verbose: Option<u8>,
            }

            #[derive(Subcommand)]
            enum Commands {
                Simulate {
                    #[clap(index = 1)]
                    config_path: String,
                },
            }

            let args = Args::parse();

            let log_level = match args.verbose.unwrap_or(0) {
                0 => Level::ERROR,
                1 => Level::WARN,
                2 => Level::INFO,
                3 => Level::DEBUG,
                _ => Level::TRACE,
            };
            tracing_subscriber::fmt().with_max_level(log_level).init();

            match &args.command {
                Some(Commands::Simulate { config_path }) => {
                    println!("Simulating configuration: {}", config_path);
                    let mut world = World::new("world");
                    world.from_config::<#behaviors>(config_path);
                    world.run().await?;
                },
                None => {
                    // Handle displaying help message if no command is provided
                    Args::command().print_help()?;
                    println!(); // Ensure newline after help output
                },
            }

            Ok(())
        }
    };

    // Convert the generated code back into a TokenStream
    TokenStream::from(expanded)
}
