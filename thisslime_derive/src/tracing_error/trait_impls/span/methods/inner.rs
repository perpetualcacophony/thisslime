use attribute_derive::FromAttr;
use syn::spanned::Spanned;

use core::tracing::derive;

pub fn quote(input: &syn::DeriveInput) -> syn::Result<syn::ImplItemFn> {
    let span = derive::attrs::Span::from_attributes(&input.attrs)?;

    let body: Vec<syn::Stmt> = match input.data {
        syn::Data::Enum(ref data) => {
            let match_arms: Vec<syn::Arm> = data
                .variants
                .iter()
                .map(|variant| {
                    let name = &variant.ident;

                    if variant
                        .attrs
                        .iter()
                        .any(|attr| attr.path().is_ident("event"))
                    {
                        let event = derive::attrs::Event::from_attributes(&variant.attrs)?;
                        let level = event.level;
                        Ok::<syn::Arm, syn::Error>(syn::parse_quote!(
                            Self::#name(ref inner) => {
                                ::tracing::event!(#level, "{}", inner.to_string());
                                &::thisslime::thisslime_core::tracing::derive::DummyEvent as &dyn ::thisslime::tracing::ToSpanOrEvent
                            }
                        ))
                    } else {
                        Ok(syn::parse_quote!(
                            Self::#name(ref inner) => inner
                        ))
                    }
                })
                .collect::<syn::Result<Vec<syn::Arm>>>()?;

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
