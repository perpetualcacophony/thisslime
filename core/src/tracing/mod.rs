#[cfg(feature = "derive")]
pub mod derive;

pub mod error;
pub use error::TracingError;

pub trait Event {
    fn construct(&self);
}

pub trait Span {
    fn construct(&self) -> tracing::Span;
    fn inner(&self) -> &dyn span_or_event::ToSpanOrEvent;
}

pub mod span_or_event;
pub use span_or_event::{SpanOrEvent, ToSpanOrEvent};

macro_rules! ad_hoc_event {
    ($name:ident, $error:ty, $level:expr) => {
        paste::paste! {
            struct ThisslimeAdHocError
        }
    };
}
