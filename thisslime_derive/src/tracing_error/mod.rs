use proc_macro2::TokenStream;
use quote::TokenStreamExt;

use thisslime_core::tracing::derive;

mod event;
mod span;
mod to_span_or_event;

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

    items.push(to_span_or_event::quote(input)?);

    if is_span(input) {
        items.push(span::quote(input)?);
    } else {
        items.push(event::quote(input)?);
    }

    Ok(items)
}

pub fn is_span(input: &syn::DeriveInput) -> bool {
    if any_attr(&input.attrs, "span") {
        true
    } else {
        !any_attr(&input.attrs, "event") && matches!(&input.data, syn::Data::Enum(..))
    }
}

fn any_attr(attrs: &[syn::Attribute], ident: &str) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident(ident))
}
