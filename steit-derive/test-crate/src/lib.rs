#[cfg(test)]
mod tests {
    use std::env;

    use steit::{
        gen::{generators::CSharpGenerator, *},
        log::loggers::PrintLogger,
        steitize, Runtime,
    };

    #[steitize(State)]
    #[derive(Debug)]
    struct Outer {
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

    #[steitize(State)]
    #[derive(Debug)]
    enum Multicase {
        #[steit(tag = 0)]
        FirstCase {
            #[steit(tag = 0)]
            foo: i32,
            #[steit(tag = 1)]
            bar: bool,
        },
        #[steit(tag = 1)]
        SecondCase {
            #[steit(tag = 0)]
            foo: i32,
            #[steit(tag = 1)]
            bar: bool,
        },
    }

    #[test]
    fn test() {
        let out_dir = env::var("CSHARP_OUT_DIR").unwrap();
        let generator = CSharpGenerator::new("Steit.Test1", out_dir);

        generator.generate::<Outer>().unwrap();
        generator.generate::<Multicase>().unwrap();

        let logger = PrintLogger::with_stdout();
        let runtime = Runtime::with_logger(Box::new(logger));

        let mut outer = Outer::new(runtime);

        outer.set_foo(127).set_bar(true).set_inner_with(|runtime| {
            let mut inner = Inner::new(runtime);
            inner.set_foo(22).set_bar(true);
            inner
        });

        outer.inner.set_foo(160);

        let logger = PrintLogger::with_stdout();
        let runtime = Runtime::with_logger(Box::new(logger));

        let mut multicase = Multicase::new(runtime);

        multicase.set_second_case_foo(68);
    }
}
