use crate::{Deserialize, Serialize};

pub trait State: Serialize + Deserialize {}
