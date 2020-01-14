#[cfg(test)]
mod tests {
    use std::env;

    use steit::{
        gen::{generators::CSharpGenerator, *},
        log::loggers::PrintLogger,
        steitize,
        types::List,
        Runtime,
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

    #[steitize(State)]
    struct Hello {
        #[steit(tag = 0)]
        numbers: List<i32>,
    }

    #[steitize(State)]
    pub struct Action {
        #[steit(tag = 0)]
        inner: ActionInner,
    }

    #[steitize(State)]
    pub struct ActionInner {
        #[steit(tag = 0)]
        action: Box<Action>,
    }

    #[test]
    fn test() {
        let out_dir = env::var("CSHARP_OUT_DIR").unwrap();
        let generator = CSharpGenerator::new("Steit.Test1", out_dir);

        generator.generate::<Outer>().unwrap();
        generator.generate::<Multicase>().unwrap();
        generator.generate::<Hello>().unwrap();
        generator.generate::<Action>().unwrap();

        let logger = PrintLogger::with_stdout();
        let runtime = Runtime::with_logger(Box::new(logger));

        let mut outer = Outer::new(runtime);

        outer.set_foo(127).set_bar(true).set_inner_with(|runtime| {
            let mut inner = Inner::new(runtime);
            inner.set_foo(22).set_bar(true);
            inner
        });

        outer.inner.set_foo(160);
        outer.set_inner_with(Inner::new);

        let logger = PrintLogger::with_stdout();
        let runtime = Runtime::with_logger(Box::new(logger));

        let mut multicase = Multicase::new(runtime);

        multicase.set_second_case_foo(68);

        let logger = PrintLogger::with_stdout();
        let runtime = Runtime::with_logger(Box::new(logger));

        let mut list = List::new(runtime);

        list.push_with(|runtime| {
            let mut inner = Inner::new(runtime);
            inner.set_foo(6);
            inner
        });

        list.push_with(|runtime| {
            let mut inner = Inner::new(runtime);
            inner.set_foo(77).set_bar(true);
            inner
        });

        list.push_with(Inner::new);
        list.get_mut(1).unwrap().set_foo(68);
        list.remove(0);

        let logger = PrintLogger::with_stdout();
        let runtime = Runtime::with_logger(Box::new(logger));

        let mut list = List::new(runtime);
        list.push(10i8);
        list.push(11);
        list.push(0);
        list.remove(1);
    }
}
