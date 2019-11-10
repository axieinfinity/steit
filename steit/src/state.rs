use super::{de::Deserialize, ser::Serialize};

pub trait State: Serialize + Deserialize {}
