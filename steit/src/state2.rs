use crate::{Deserialize2, Runtime2, Serialize2};

pub trait State: Serialize2 + Deserialize2 {}
