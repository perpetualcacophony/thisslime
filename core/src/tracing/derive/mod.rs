pub mod attrs;
pub mod model;

#[derive(Debug, Copy, Clone)]
pub struct DummyEvent;

impl super::Event for DummyEvent {
    fn construct(&self) {
        // no-op
    }
}

impl super::ToSpanOrEvent for DummyEvent {
    fn to_span_or_event(&self) -> super::SpanOrEvent<'_> {
        super::SpanOrEvent::Event(self)
    }
}

impl<'a> super::ToSpanOrEvent for &'a DummyEvent {
    fn to_span_or_event(&self) -> super::SpanOrEvent<'_> {
        super::SpanOrEvent::Event(*self)
    }
}
