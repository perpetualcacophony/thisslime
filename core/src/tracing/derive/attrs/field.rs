use crate::tracing::derive;

#[derive(attribute_derive::FromAttr)]
#[attribute(ident = field)]
pub struct Field {
    #[attribute(optional)]
    pub print: derive::model::TracingPrintLevel,

    pub rename: Option<String>,
}
