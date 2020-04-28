use super::rt::RuntimeV2;

pub trait StateV2 {
    fn with_runtime_v2(runtime: RuntimeV2) -> Self;
    fn runtime_v2(&self) -> &RuntimeV2;
    fn set_runtime_v2(&mut self, runtime: RuntimeV2);
}
