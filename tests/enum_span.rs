use thisslime::TracingError;

#[derive(thisslime::TracingError)]
struct MyOwnError;

impl std::fmt::Display for MyOwnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("MyOwnError")
    }
}

#[derive(thisslime::TracingError)]
enum MyErrorEnum {
    Mine(MyOwnError),

    #[event(level = ERROR)]
    Std(std::io::Error),
}

#[test]
#[tracing_test::traced_test]
fn my_error_enum() {
    let err = MyErrorEnum::Mine(MyOwnError);
    err.trace();

    let err = MyErrorEnum::Std(std::io::Error::from_raw_os_error(1));
    err.trace();
}
