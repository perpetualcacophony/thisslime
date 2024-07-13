use std::fmt::Display;

pub trait ToSpanOrEvent {
    fn to_span_or_event(&self) -> SpanOrEvent<'_>;
}

impl<T: ToSpanOrEvent + ?Sized> super::error::dispatch::Dispatch for T {
    fn dispatch(&self) {
        self.to_span_or_event().dispatch()
    }
}

pub struct AdHocEvent<T> {
    err: T,
    level: model::Level,
}

impl<T: std::fmt::Display> AdHocEvent<T> {
    fn new(err: T, attr: attrs::Event) -> Self {
        Self(err)
    }
}

impl<T: std::fmt::Display> Event for AdHocEvent<T> {
    fn construct(&self) {
        tracing::event!("{}", self.to_string())
    }
}

impl<T> ToSpanOrEvent for AdHocEvent<T> {
    fn to_span_or_event(&self) -> SpanOrEvent<'_> {
        SpanOrEvent::Event(self)
    }
}

pub enum SpanOrEvent<'a> {
    Span(&'a dyn super::Span),
    Event(&'a dyn super::Event),
}

impl<'a> super::error::dispatch::Dispatch for SpanOrEvent<'a> {
    fn dispatch(&self) {
        match self {
            Self::Span(span) => {
                let tracing_span = span.construct();
                let _enter = tracing_span.enter();

                span.inner().dispatch();
            }
            Self::Event(event) => event.construct(),
        }
    }
}
