pub mod error;
pub use error::TracingError;

pub use thisslime_core::tracing::{Event, Span};

pub mod span_or_event;
pub use span_or_event::{SpanOrEvent, ToSpanOrEvent};
