pub trait SaturatingAddAssign {
	fn saturating_add_assign(&mut self, rhs: Self);
	fn saturating_sub_assign(&mut self, rhs: Self);
	fn saturating_inc(&mut self);
	fn saturating_dec(&mut self);
}

impl<T> SaturatingAddAssign for T
where
	T: num_traits::PrimInt,
{
	fn saturating_add_assign(&mut self, rhs: Self) {
		*self = (*self).saturating_add(rhs)
	}

	fn saturating_sub_assign(&mut self, rhs: Self) {
		*self = (*self).saturating_sub(rhs)
	}

	fn saturating_inc(&mut self) {
		self.saturating_add_assign(T::one())
	}

	fn saturating_dec(&mut self) {
		self.saturating_sub_assign(T::one())
	}
}
