#![doc(html_root_url = "https://docs.rs/wasm-bindgen-macro/0.2")]
#![allow(warnings)]
use proc_macro2::TokenStream;
use quote::quote;
#[no_mangle]
pub extern "C" fn wasm_bindgen(
    attr: proc_macro2::TokenStream,
    input: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    match wasm_bindgen_macro_support::expand(attr.into(), input.into()) {
        Ok(tokens) => {
            if cfg!(feature = "xxx_debug_only_print_generated_code") {
                println!("{}", tokens);
            }
            tokens.into()
        }
        Err(diagnostic) => (quote! { # diagnostic }).into(),
    }
}
#[no_mangle]
pub extern "C" fn __wasm_bindgen_class_marker(
    attr: proc_macro2::TokenStream,
    input: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    match wasm_bindgen_macro_support::expand_class_marker(attr.into(), input.into()) {
        Ok(tokens) => {
            if cfg!(feature = "xxx_debug_only_print_generated_code") {
                println!("{}", tokens);
            }
            tokens.into()
        }
        Err(diagnostic) => (quote! { # diagnostic }).into(),
    }
}
