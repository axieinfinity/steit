use crate::varint::Varint;

use super::runtime::Runtime;

pub trait Runtimed {
    fn with_runtime(runtime: Runtime) -> Self;
    fn runtime(&self) -> &Runtime;
}

impl<T: Default + Varint> Runtimed for T {
    #[inline]
    fn with_runtime(_runtime: Runtime) -> Self {
        Default::default()
    }

    #[inline]
    fn runtime(&self) -> &Runtime {
        panic!("cannot get a `Runtime` from a varint")
    }
}
