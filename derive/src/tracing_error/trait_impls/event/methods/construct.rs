use attribute_derive::FromAttr;
use quote::ToTokens;
use syn::spanned::Spanned;

pub fn quote(input: &syn::DeriveInput) -> syn::Result<syn::ImplItemFn> {
    match input.data {
        syn::Data::Struct(ref data) => {
            let attr = core::tracing::derive::attrs::Event::from_attributes(&input.attrs)?;

            let tracing_fields = data
                .fields
                .iter()
                .map(core::tracing::derive::model::Field::try_from)
                .collect::<Result<_, syn::Error>>();

            if let Ok(tracing_fields) = tracing_fields {
                let event =
                    core::tracing::derive::model::Event::new(attr.level, tracing_fields, true);
                let tracing_event = event.into_macro_call();

                Ok(syn::parse_quote!(
                    fn construct(&self) {
                        #tracing_event;
                    }
                ))
            } else {
                Ok(syn::parse_quote!(
                    fn construct(&self) {
                        self.default_event();
                    }
                ))
            }
        }
        syn::Data::Enum(..) => Err(syn::Error::new(
            input.span(),
            "currently can't derive Event for enums",
        )),
        syn::Data::Union(..) => unimplemented!(),
    }
}
