//! A simple newtype wrapper around failure::Error
//!
//! The primary items exported by this library are:
//!
//! - `ExitFailure`: a wrapper around `failure::Error` to allow ? printing from main
//!    present a nicer error message
//!
//! Basically, ExitFailure should only ever be used in the return type for
//! `main() -> Result<(), exitfailure::ExitFailure>`
extern crate failure;

/// The newtype wrapper around `failure::Error`
///
/// ```rust,should_panic
/// # extern crate failure;
/// # extern crate exitfailure;
/// # use failure::ResultExt;
/// # use exitfailure::ExitFailure;
/// fn main() -> Result<(), ExitFailure> {
///     Ok(some_fn()?)
/// }
///
/// fn some_fn() -> Result<(), failure::Error> {
///     let error = Err(failure::err_msg("root cause failure"));
///     Ok(error.context("this is some context".to_string())?)
/// }
/// ```
pub struct ExitFailure(failure::Error);

impl std::fmt::Debug for ExitFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use failure::Fail;
        let mut fail: &Fail = self.0.cause();
        write!(f, "{}", fail)?;
        while let Some(cause) = fail.cause() {
            write!(f, "\ncaused by: {}", cause)?;
            fail = cause;
        }
        Ok(())
    }
}

impl std::convert::From<failure::Error> for ExitFailure {
    fn from(failure: failure::Error) -> Self {
        ExitFailure(failure)
    }
}

impl<T> std::convert::From<failure::Context<T>> for ExitFailure
where
    T: std::fmt::Display + std::marker::Sync + std::marker::Send,
{
    fn from(failure: failure::Context<T>) -> Self {
        ExitFailure(failure.into())
    }
}
