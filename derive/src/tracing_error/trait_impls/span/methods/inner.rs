use attribute_derive::FromAttr;
use syn::spanned::Spanned;

pub fn quote(input: &syn::DeriveInput) -> syn::Result<syn::ImplItemFn> {
    let span = core::tracing::attrs::Span::from_attributes(&input.attrs)?;

    let body: Vec<syn::Stmt> = match input.data {
        syn::Data::Enum(ref data) => {
            let match_arms: Vec<syn::Arm> = data
                .variants
                .iter()
                .map(|variant| {
                    let name = &variant.ident;

                    syn::parse_quote!(
                        Self::#name(ref inner) => inner
                    )
                })
                .collect();

            let match_body: syn::Stmt = syn::parse_quote!(
                match self {
                    #(#match_arms),*
                }
            );

            vec![match_body]
        }
        syn::Data::Struct(..) => {
            return Err(syn::Error::new(
                input.span(),
                "deriving Span trait not currently possible for structs",
            ))
        }
        syn::Data::Union(..) => unimplemented!(),
    };

    Ok(syn::parse_quote!(
        fn inner(&self) -> &dyn ::thisslime::tracing::ToSpanOrEvent {
            #(#body)*
        }
    ))
}
