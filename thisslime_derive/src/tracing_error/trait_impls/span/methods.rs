mod construct;
mod inner;

pub fn list(input: &syn::DeriveInput) -> syn::Result<Vec<syn::ImplItemFn>> {
    let mut methods = Vec::new();

    methods.push(construct::quote(input)?);
    methods.push(inner::quote(input)?);

    Ok(methods)
}
