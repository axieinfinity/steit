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

    impl HasMeta for Outer {
        const META: &'static Meta = &Meta::Struct(Struct {
            name: "Outer",
            fields: &[
                Field {
                    name: "foo",
                    ty: &FieldType::Primitive("i32"),
                    tag: 0,
                },
                Field {
                    name: "bar",
                    ty: &FieldType::Primitive("bool"),
                    tag: 1,
                },
                Field {
                    name: "inner",
                    ty: &FieldType::Meta(Inner::META),
                    tag: 2,
                },
            ],
        });
    }

    #[steitize(State)]
    #[derive(Debug)]
    struct Inner {
        #[steit(tag = 0)]
        foo: i32,
        #[steit(tag = 1)]
        bar: bool,
    }

    impl HasMeta for Inner {
        const META: &'static Meta = &Meta::Struct(Struct {
            name: "Inner",
            fields: &[
                Field {
                    name: "foo",
                    ty: &FieldType::Primitive("i32"),
                    tag: 0,
                },
                Field {
                    name: "bar",
                    ty: &FieldType::Primitive("bool"),
                    tag: 1,
                },
            ],
        });
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

    impl HasMeta for Multicase {
        const META: &'static Meta = &Meta::Enum(Enum {
            name: "Multicase",
            variants: &[
                Variant {
                    ty: &Struct {
                        name: "FirstCase",
                        fields: &[
                            Field {
                                name: "foo",
                                ty: &FieldType::Primitive("i32"),
                                tag: 0,
                            },
                            Field {
                                name: "bar",
                                ty: &FieldType::Primitive("bool"),
                                tag: 1,
                            },
                        ],
                    },
                    tag: 0,
                },
                Variant {
                    ty: &Struct {
                        name: "SecondCase",
                        fields: &[
                            Field {
                                name: "foo",
                                ty: &FieldType::Primitive("i32"),
                                tag: 0,
                            },
                            Field {
                                name: "bar",
                                ty: &FieldType::Primitive("bool"),
                                tag: 1,
                            },
                        ],
                    },
                    tag: 1,
                },
            ],
        });
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
    }
}
