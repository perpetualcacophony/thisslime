#[derive(attribute_derive::FromAttr)]
#[attribute(ident = span)]
pub struct Span {
    #[attribute(optional)]
    pub level: crate::tracing::model::Level,
}
