use crate::tracing::derive;
use heck::ToSnekCase;

#[derive(attribute_derive::FromAttr)]
#[attribute(ident = span)]
pub struct Span {
    #[attribute(optional)]
    pub level: derive::model::Level,

    name: Option<String>,
}

impl Span {
    pub fn name(self, ident: &syn::Ident) -> String {
        self.name
            .unwrap_or_else(|| ident.to_string().to_snek_case())
    }
}
