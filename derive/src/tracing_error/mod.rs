use proc_macro2::TokenStream;
use quote::TokenStreamExt;

mod trait_impls;

pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    expanded(&input).into()
}

fn expanded(input: &syn::DeriveInput) -> TokenStream {
    items(input)
        .map(|items| {
            let mut tokens = TokenStream::new();
            tokens.append_all(items);
            tokens
        })
        .unwrap_or_else(syn::Error::into_compile_error)
}

fn items(input: &syn::DeriveInput) -> syn::Result<Vec<syn::Item>> {
    let mut items = Vec::new();

    trait_impls::add_to_items(&mut items, input)?;

    Ok(items)
}
