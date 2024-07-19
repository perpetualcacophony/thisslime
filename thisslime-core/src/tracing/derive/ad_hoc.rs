use std::fmt::Display;

pub struct AdHocEvent<Err> {
    error: Err,
    level: super::model::Level,
}

impl<Err: Display> Display for AdHocEvent<Err> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(f)
    }
}

impl<Err: Display> crate::tracing::Event for AdHocEvent<Err> {
    fn construct(&self) {}
}
