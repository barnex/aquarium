use crate::*;

pub trait IntoCell<T>
where
    T: Copy,
{
    fn cel(self) -> Cel<T>;
}

impl<T: Copy> IntoCell<T> for T {
    fn cel(self) -> Cel<T> {
        Cel::new(self)
    }
}
