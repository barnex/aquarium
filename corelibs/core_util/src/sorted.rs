/// Extension trait to get a Vec<T> in sorted order.
pub trait Sorted {
    fn sorted(self) -> Self;
}

impl<T: Ord> Sorted for Vec<T> {
    /// Sorts the elements of a Vec.
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}
