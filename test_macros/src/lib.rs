use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn auto_db_cleanup(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    input.block.stmts.push(syn::parse_quote! {
        app.clean_up().await;
    });

    TokenStream::from(quote! {
        #input
    })
}
