use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HOPhysicsEngine)]
pub fn ho_physics_engine(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let DeriveInput {
        ident: struct_name_ident,
        ..
    } = parse_macro_input!(input as DeriveInput);

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        // ...
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

#[test]
fn test_no_capabilities() {
    struct EmptyPhysEngine{}
    let empty_capabilities = bevy_hophy::PhysCapabilities::new(&[]);
}
