use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// TODO: Finish this thing

#[proc_macro_derive(Commands, attributes(help, help_long))]
pub fn commands_derive_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let attrs = input.attrs;
    let output = quote! {
        impl #name {
            fn handle() {
                println!("Hey! We are alive!");
            }
        }
    };
    output.into()
}