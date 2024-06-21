//use crate::tracing_error as derive;

pub fn quote(input: &syn::DeriveInput) -> syn::Result<Option<syn::ImplItemFn>> {
    let body: Option<Vec<syn::Stmt>> = match input.data {
        syn::Data::Enum(ref data) => {
            let mut match_arms = Vec::with_capacity(data.variants.len());

            for variant in data.variants.iter().filter(|variant| {
                !variant
                    .attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("event"))
            }) {
                let ident = &variant.ident;

                let match_arm: syn::Arm = syn::parse_quote! {
                    Self::#ident(ref err) => Some(err)
                };

                match_arms.push(match_arm);
            }

            Some(syn::parse_quote! {
                match self {
                    #(#match_arms,)*
                    _ => None
                }
            })
        }
        _ => None,
    };

    if let Some(body) = body {
        Ok(Some(syn::parse_quote!(
            fn source(&self) -> Option<&(dyn TracingError + 'static)> {
                #(#body)*
            }
        )))
    } else {
        Ok(None)
    }
}
