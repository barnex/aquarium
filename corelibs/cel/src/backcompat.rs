/// Pretend a number is a cel.
pub trait CelLike<T: Copy> {
    fn set(&mut self, v: T);
}
