use attribute_derive::FromAttr;
use quote::ToTokens;
use syn::spanned::Spanned;

use core::tracing::attrs;
use core::tracing::model as tracing;

pub fn quote(input: &syn::DeriveInput) -> syn::Result<syn::ImplItemFn> {
    let body: Vec<syn::Stmt> = match input.data {
        syn::Data::Struct(ref data) => {
            let attr = attrs::Event::from_attributes(&input.attrs)?;

            let tracing_fields = data
                .fields
                .iter()
                .map(tracing::Field::try_from)
                .collect::<Result<_, _>>();

            if let Ok(tracing_fields) = tracing_fields {
                let event = tracing::Event::new(attr.level, tracing_fields, true);
                let stmt: syn::Stmt = syn::parse2(event.into_macro_call().into_token_stream())?;
                vec![stmt]
            } else {
                vec![syn::parse_quote!(self.default_event())]
            }
        }
        syn::Data::Enum(ref data) => {
            use heck::ToSnakeCase;

            let span = attrs::Span::from_attributes(&input.attrs)?;
            let level = span.level;

            data.variants
                .iter()
                .map(|variant| {
                    let span_name = variant.ident.to_string().to_snake_case();

                    use syn::Fields;
                    match &variant.fields {
                        Fields::Unnamed(fields) => {
                            assert!(fields.unnamed.len() == 1);
                        }
                        _ => unimplemented!(),
                    }

                    let ident = &variant.ident;

                    let match_return: syn::Expr = if let Some(attr) = variant
                        .attrs
                        .iter()
                        .find(|attr| attr.path().is_ident("event"))
                    {
                        let attr = attrs::Event::from_attribute(attr)?;
                        let event = tracing::Event::new_custom(
                            attr.level,
                            Vec::default(),
                            syn::parse_str("err").unwrap(),
                        );
                        let tracing_event = event.into_macro_call();

                        syn::parse_quote! { #tracing_event }
                    } else {
                        syn::parse_quote! { TracingError::event(err) }
                    };

                    Ok(syn::parse_quote! {
                        #ident(err) => {
                            let span = ::tracing::span!(#level, #span_name);
                            let _enter = span.enter();

                            #match_return
                        }
                    })
                })
                .collect::<Result<Vec<_>, syn::Error>>()?
        }
        syn::Data::Union(_) => return Err(syn::Error::new(input.span(), "not supporting enums")),
    };

    Ok(syn::parse_quote!(
        fn event(&self) {
            #(#body)*
        }
    ))
}
