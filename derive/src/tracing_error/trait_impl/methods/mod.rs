mod event;
mod source;

pub fn quote(input: &syn::DeriveInput) -> syn::Result<Vec<syn::ImplItemFn>> {
    let mut methods = Vec::new();

    methods.push(event::quote(input)?);

    if let Some(source) = source::quote(input)? {
        methods.push(source);
    }

    Ok(methods)
}

#[allow(unused)]
pub fn add_to_impl(
    impl_items: &mut Vec<syn::ImplItem>,
    input: &syn::DeriveInput,
) -> syn::Result<()> {
    for method in self::quote(input)? {
        impl_items.push(syn::ImplItem::Fn(method));
    }

    Ok(())
}
