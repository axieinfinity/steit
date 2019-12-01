#[macro_export]
macro_rules! test_case {
    ($name:ident : $assert:expr ; $($args:expr),+) => {
        #[test]
        fn $name() {
            $assert($($args),+);
        }
    };

    ($name:ident : $assert:expr ; $($input:expr),+ => $($output:expr),+) => {
        test_case!($name : $assert ; $($input),+, $($output),+);
    };
}
