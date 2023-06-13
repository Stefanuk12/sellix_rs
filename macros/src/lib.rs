// Dependencies
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Ensures an API model has an api_key and merchant field.
/// Used for do_request.
#[proc_macro_derive(WithAPIKey)]
pub fn with_api_key_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct we're deriving the trait for.
    let name = input.ident;

    // Generate the implementation of the trait.
    let expanded = quote! {  
        impl WithAPIKey for #name {
            fn api_key(&self) -> String {
                self.api_key.clone()
            }
            fn merchant(&self) -> Option<String> {
                self.merchant.clone()
            }
        }
    };

    // Return the generated code as tokens.
    TokenStream::from(expanded)
}

/// Implements `DoRequest` for a struct.
/// 
/// `impl DoRequest for #name {}`
#[proc_macro_derive(WithDoRequest)]
pub fn with_do_request_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let gen = quote! {
        impl DoRequest for #name {}
    };

    gen.into()
}