use thisslime::TracingError;

#[tracing_test::traced_test]
#[test]
fn main() {
    let err = MyError {
        boop: String::from("im an error!"),
    };

    err.trace();
}

#[derive(thisslime::TracingError)]
#[event(level = ERROR)]
struct MyError {
    boop: String,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("my_error")
    }
}
