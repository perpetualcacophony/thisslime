pub trait ToSpanOrEvent {
    fn to_span_or_event(&self) -> SpanOrEvent<'_>;
}

impl<T: ToSpanOrEvent + ?Sized> super::error::dispatch::Dispatch for T {
    fn dispatch(&self) {
        self.to_span_or_event().dispatch()
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

                span.inner().dispatch()
            }
            Self::Event(event) => event.construct(),
        }
    }
}
