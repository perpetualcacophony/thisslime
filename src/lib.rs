#[cfg(feature = "tracing")]
pub mod tracing;

#[cfg(feature = "tracing")]
pub use tracing::error::TracingError;

#[cfg(feature = "tracing")]
#[cfg(feature = "derive")]
pub use derive::TracingError;
