#[cfg(test)]
mod tests {
    use steit::{log::loggers::PrintLogger, steitize, Runtime};

    #[steitize(State)]
    #[derive(Debug)]
    struct Test {
        #[steit(tag = 0)]
        foo: i32,
        #[steit(tag = 1)]
        bar: bool,
        #[steit(tag = 2)]
        inner: Inner,
    }

    #[steitize(State)]
    #[derive(Debug)]
    struct Inner {
        #[steit(tag = 0)]
        foo: i32,
        #[steit(tag = 1)]
        bar: bool,
    }

    #[test]
    fn test() {
        let logger = PrintLogger::with_stdout();
        let runtime = Runtime::with_logger(Box::new(logger));

        let mut test = Test::new(runtime);

        test.set_foo(127).set_bar(true).set_inner_with(|runtime| {
            let mut inner = Inner::new(runtime);
            inner.set_foo(22).set_bar(true);
            inner
        });

        test.inner.set_foo(160);
    }
}
