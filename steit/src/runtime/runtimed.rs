use crate::varint;

use super::runtime::Runtime;

pub trait Runtimed {
    fn with_runtime(runtime: Runtime) -> Self;
    fn runtime(&self) -> &Runtime;
}

// TODO: Remove `varint::` after refactoring `Varint`
impl<T: Default + varint::Varint> Runtimed for T {
    #[inline]
    fn with_runtime(_runtime: Runtime) -> Self {
        Default::default()
    }

    #[inline]
    fn runtime(&self) -> &Runtime {
        panic!("cannot get a `Runtime` from a varint")
    }
}
