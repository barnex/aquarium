use std::fmt;

/// Extension trait providing `inspect_err`, to inspect/log errors inline.
pub trait InspectErr: Sized {
    type Error: fmt::Debug;

    /// Log error if `self` is an error, passthrough `self` unchanged
    fn log_err(self) -> Self;

    /// Log and ignore error.
    fn swallow_err(self) {
        let _ = self.log_err();
    }

    fn ignore_err(self) {}
}

impl<T, E> InspectErr for Result<T, E>
where
    E: fmt::Debug,
{
    type Error = E;

    fn log_err(self) -> Self {
        if let Err(e) = &self {
            log::error!("{e:#?}")
        }
        self
    }
}
