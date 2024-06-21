mod construct;

pub fn list(input: &syn::DeriveInput) -> syn::Result<Vec<syn::ImplItemFn>> {
    let mut methods = Vec::new();

    methods.push(construct::quote(input)?);

    Ok(methods)
}
