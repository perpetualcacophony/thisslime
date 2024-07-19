use crate::tracing_error::derive;
use attribute_derive::FromAttr;
use derive::{attrs, model};

pub fn quote(input: &syn::DeriveInput) -> syn::Result<syn::ImplItemFn> {
    match input.data {
        syn::Data::Struct(ref data) => {
            let attr = attrs::Event::from_attributes(&input.attrs)?;

            let tracing_fields = data
                .fields
                .iter()
                .map(model::Field::try_from)
                .collect::<Result<_, syn::Error>>();

            if let Ok(tracing_fields) = tracing_fields {
                let event = model::Event::new(attr.level, tracing_fields, true);
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
        syn::Data::Enum(_) => {
            let attr = attrs::Event::from_attributes(&input.attrs)?;
            let event = model::Event::new(attr.level, Vec::new(), true);
            let tracing_event = event.into_macro_call();

            Ok(syn::parse_quote!(
                fn construct(&self) {
                    #tracing_event;
                }
            ))
        }
        syn::Data::Union(..) => unimplemented!(),
    }
}
