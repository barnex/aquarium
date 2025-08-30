pub trait Ignore: Sized {
    fn ignore(self) {}
}

impl<T> Ignore for T {}
