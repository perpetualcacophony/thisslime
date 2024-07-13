#[derive(attribute_derive::FromAttr)]
#[attribute(ident = event)]
pub struct Event {
    #[attribute(optional)]
    pub level: crate::tracing::model::Level,
}
