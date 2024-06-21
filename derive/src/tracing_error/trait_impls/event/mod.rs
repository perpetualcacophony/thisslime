mod methods;

pub fn quote(input: &syn::DeriveInput) -> syn::Result<syn::ItemImpl> {
    let ident = &input.ident;
    let methods = methods::list(input)?;

    Ok(syn::parse_quote!(
        impl ::thisslime::tracing::Event for #ident {
            #(#methods)*
        }
    ))
}
