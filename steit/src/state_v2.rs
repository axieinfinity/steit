use super::{de_v2::DeserializeV2, rt::RuntimeV2, ser_v2::SerializeV2};

pub trait StateV2: SerializeV2 + DeserializeV2 {
    fn with_runtime_v2(runtime: RuntimeV2) -> Self;
    fn runtime_v2(&self) -> &RuntimeV2;
    fn set_runtime_v2(&mut self, runtime: RuntimeV2);
}
