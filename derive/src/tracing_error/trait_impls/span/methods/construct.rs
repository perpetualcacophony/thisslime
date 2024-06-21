use attribute_derive::FromAttr;

pub fn quote(input: &syn::DeriveInput) -> syn::Result<syn::ImplItemFn> {
    let span = core::tracing::attrs::Span::from_attributes(&input.attrs)?;
    let level = span.level;
    let span_name = span.name(&input.ident);

    Ok(syn::parse_quote!(
        fn construct(&self) -> ::tracing::Span {
            ::tracing::span!(#level, #span_name)
        }
    ))
}
