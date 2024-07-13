pub trait TracingError {
    fn trace(&self);
}

impl<T: dispatch::Dispatch> TracingError for T {
    fn trace(&self) {
        self.dispatch();
    }
}

pub mod dispatch;
