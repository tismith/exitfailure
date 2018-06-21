#![deny(
    missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
    trivial_numeric_casts, unreachable_pub, unsafe_code, unstable_features, unused_extern_crates,
    unused_import_braces, unused_qualifications, variant_size_differences
)]
//! A simple newtype wrapper around failure::Error
//!
//! The primary items exported by this library are:
//!
//! - `ExitFailure`: a wrapper around `failure::Error` to allow ? printing from main
//!    present a nicer error message
//!
//! Basically, ExitFailure should only ever be used in the return type for
//! `main() -> Result<(), exitfailure::ExitFailure>`
//!
//! Will also include the backtrace as prepared by the `failure` crate,
//! if the environment variable RUST_BACKTRACE=1 is set.
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

/// Prints a list of causes for this Error, along with any backtrace
/// information collected by the Error (if RUST_BACKTRACE=1).
impl std::fmt::Debug for ExitFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use failure::Fail;
        let mut fail: &Fail = self.0.cause();
        write!(f, "{}", fail)?;
        while let Some(cause) = fail.cause() {
            write!(f, "\nInfo: caused by {}", cause)?;
            fail = cause;
        }

        if let Ok(x) = std::env::var("RUST_BACKTRACE") {
            if x == "1" {
                write!(f, "\n{}", self.0.backtrace())?
            }
        }
        Ok(())
    }
}

impl<T: Into<failure::Error>> From<T> for ExitFailure {
    fn from(t: T) -> ExitFailure {
        ExitFailure(t.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fmt::Write;

    #[test]
    fn test_exitfailure() {
        let mut buffer = String::new();
        let error = failure::err_msg("some failure").context("some context");
        let exitfailure: ExitFailure = error.into();
        write!(buffer, "{:?}", exitfailure).unwrap();
        assert!(buffer.contains("some failure"));
        assert!(buffer.contains("some context"));
    }
}
