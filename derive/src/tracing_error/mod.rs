use proc_macro2::TokenStream;
use quote::TokenStreamExt;

mod trait_impl;

#[derive(attribute_derive::FromAttr)]
#[attribute(ident = field)]
struct FieldAttribute {
    #[attribute(optional)]
    pub print: tracing::TracingPrintLevel,

    pub rename: Option<String>,
}

#[derive(attribute_derive::FromAttr)]
#[attribute(ident = event)]
struct EventAttribute {
    #[attribute(optional)]
    level: tracing::Level,
}

#[derive(attribute_derive::FromAttr)]
#[attribute(ident = span)]
struct SpanAttribute {
    #[attribute(optional)]
    level: tracing::Level,
}

pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    expanded(&input).into()
}

mod tracing;

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

    trait_impl::add_to_items(&mut items, input)?;

    Ok(items)
}
