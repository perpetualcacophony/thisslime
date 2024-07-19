pub fn quote(input: &syn::DeriveInput) -> syn::Result<syn::ImplItemFn> {
    let variant: syn::Ident = if super::super::is_span(input) {
        syn::parse_quote!(Span)
    } else {
        syn::parse_quote!(Event)
    };

    Ok(syn::parse_quote!(
        fn to_span_or_event(&self) -> thisslime::tracing::SpanOrEvent<'_> {
            thisslime::tracing::SpanOrEvent::#variant(self)
        }
    ))
}
