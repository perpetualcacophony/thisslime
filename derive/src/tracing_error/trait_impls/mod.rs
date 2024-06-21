pub mod event;
pub mod span;
pub mod to_span_or_event;

pub fn list(input: &syn::DeriveInput) -> syn::Result<Vec<syn::ItemImpl>> {
    let mut impls = Vec::new();

    if is_span(input) {
        impls.push(span::quote(input)?);
    } else {
        impls.push(event::quote(input)?);
    }

    impls.push(to_span_or_event::quote(input)?);

    Ok(impls)
}

pub fn add_to_items(items: &mut Vec<syn::Item>, input: &syn::DeriveInput) -> syn::Result<()> {
    let impls = self::list(input)?;

    for impl_trait in impls {
        items.push(syn::Item::Impl(impl_trait))
    }

    Ok(())
}

pub fn is_span(input: &syn::DeriveInput) -> bool {
    if any_attr(&input.attrs, "span") {
        true
    } else if !any_attr(&input.attrs, "event") && matches!(&input.data, syn::Data::Enum(..)) {
        true
    } else {
        false
    }
}

fn any_attr(attrs: &[syn::Attribute], ident: &str) -> bool {
    attrs.into_iter().any(|attr| attr.path().is_ident(ident))
}
