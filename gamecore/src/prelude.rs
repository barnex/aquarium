pub use crate::*;

pub fn default<T: Default>() -> T {
    T::default()
}
