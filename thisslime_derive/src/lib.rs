#[macro_use]
mod macros;

#[cfg(feature = "tracing")]
mod tracing_error;

#[cfg(feature = "tracing")]
#[proc_macro_derive(TracingError, attributes(event, field, span))]
pub fn derive_tracing_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    tracing_error::derive(input)
}
