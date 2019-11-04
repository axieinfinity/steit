use crate::{Deserialize2, Runtime2, Serialize2};

pub trait State: Serialize2 + Deserialize2 {
    fn new_with_runtime(runtime: Runtime2) -> Self {
        // See `Deserialize` on why we use `Self::` instead of `Default::`
        let mut state = Self::default();
        state.set_runtime(runtime);
        state
    }

    fn set_runtime(&mut self, runtime: Runtime2) -> &mut Self;
}
