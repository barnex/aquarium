use std::cell::RefCell;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct CVec<T>(RefCell<Vec<T>>);
