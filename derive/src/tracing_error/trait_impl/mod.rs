mod methods;

fn quote(input: &syn::DeriveInput) -> syn::Result<syn::ItemImpl> {
    let ident = &input.ident;

    let methods = methods::quote(input)?;

    Ok(syn::parse_quote!(
        impl ::thisslime::TracingError for #ident {
            #(#methods)*
        }
    ))
}

pub fn add_to_items(items: &mut Vec<syn::Item>, input: &syn::DeriveInput) -> syn::Result<()> {
    let item = self::quote(input).map(syn::Item::Impl)?;
    items.push(item);
    Ok(())
}
