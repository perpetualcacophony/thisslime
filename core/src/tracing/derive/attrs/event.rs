use crate::tracing::derive;

#[derive(attribute_derive::FromAttr)]
#[attribute(ident = event)]
pub struct Event {
    #[attribute(optional)]
    pub level: derive::model::Level,
}
