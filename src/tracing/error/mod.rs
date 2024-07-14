pub trait TracingError {
    fn trace(&self);
}

impl<T: Dispatch> TracingError for T {
    fn trace(&self) {
        self.dispatch();
    }
}

pub use thisslime_core::tracing::error::dispatch::Dispatch;
