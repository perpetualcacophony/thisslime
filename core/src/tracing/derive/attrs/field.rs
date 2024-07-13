#[derive(attribute_derive::FromAttr)]
#[attribute(ident = field)]
pub struct Field {
    #[attribute(optional)]
    pub print: crate::tracing::model::TracingPrintLevel,

    pub rename: Option<String>,
}
