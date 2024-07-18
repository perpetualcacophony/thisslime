mod to_span_or_event;

pub fn list(input: &syn::DeriveInput) -> syn::Result<Vec<syn::ImplItemFn>> {
    let mut methods = Vec::new();

    methods.push(to_span_or_event::quote(input)?);

    Ok(methods)
}

#[allow(unused)]
pub fn add_to_impl(
    impl_items: &mut Vec<syn::ImplItem>,
    input: &syn::DeriveInput,
) -> syn::Result<()> {
    for method in self::list(input)? {
        impl_items.push(syn::ImplItem::Fn(method));
    }

    Ok(())
}
