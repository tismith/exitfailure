//! A simple newtype wrapper around failure::Error
//!
//! The primary items exported by this library are:
//!
//! - `ExitFailure`: a wrapper around `failure::Error` to allow ? printing from main
//!    present a nicer error message
//!
//! Basically, ExitFailure should only every be used in the return type for
//! `main() -> Result<(), exitfailure::ExitFailure>`
extern crate failure;

/// The newtype wrapper around `failure::Error`
///
/// ```rust,should_panic
/// # extern crate failure;
/// # extern crate exitfailure;
/// # use failure::ResultExt;
/// fn main() -> Result<(), exitfailure::ExitFailure> {
///     let error = Err(failure::err_msg("root cause failure"));
///     Ok(error.context("this is some extra context".to_string())?)
/// }
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

impl<F: failure::Fail> std::convert::From<F> for ExitFailure {
    fn from(failure: F) -> Self {
        ExitFailure(failure.into())
    }
}
