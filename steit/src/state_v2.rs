use super::runtime::Runtime;

pub trait StateV2 {
    fn with_runtime(runtime: Runtime) -> Self;
    fn runtime(&self) -> &Runtime;
    fn set_runtime(&mut self, runtime: Runtime);
}
