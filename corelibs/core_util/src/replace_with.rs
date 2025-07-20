pub trait ReplaceWith: Sized {
	fn replace_with<F: FnOnce(Self) -> Self>(&mut self, f: F);
}

impl<T> ReplaceWith for T
where
	T: Copy,
{
	#[inline]
	fn replace_with<F: FnOnce(Self) -> Self>(&mut self, f: F) {
		*self = f(*self)
	}
}
