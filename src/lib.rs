// Copyright (c) 2018 Toby Smith <toby@tismith.id.au>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences
)]

//! Some newtype wrappers to help with using ? in main()
//!
//! The primary items exported by this library are:
//!
//! - `ExitFailure`: a wrapper around `failure::Error` to allow ? printing from main
//!    to present a nicer error message, including any available context and backtrace.
//!
//! - `ExitDisplay<E>`: a wrapper around `E: std::fmt::Display` to allow the error message
//!    from main to use `Display` and not `Debug`
//!
//! Basically, these types should only ever be used in the return type for
//! `main()`
//!
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
        let fail = self.0.as_fail();
        write!(f, "{}", fail)?;

        for cause in fail.iter_causes() {
            write!(f, "\nInfo: caused by {}", cause)?;
        }

        if let Ok(x) = std::env::var("RUST_BACKTRACE") {
            if x != "0" {
                write!(f, "\n{}", self.0.backtrace())?
            }
        }

        Ok(())
    }
}

impl<T: Into<failure::Error>> From<T> for ExitFailure {
    fn from(t: T) -> Self {
        ExitFailure(t.into())
    }
}

/// A newtype wrapper around `E: std::fmt::Display`
///
/// ```rust,should_panic
/// # extern crate exitfailure;
/// # use exitfailure::ExitDisplay;
/// fn main() -> Result<(), ExitDisplay<String>> {
///     Ok(some_fn()?)
/// }
///
/// fn some_fn() -> Result<(), String> {
///     Err("some error".into())
/// }
/// ```
pub struct ExitDisplay<E: std::fmt::Display>(E);

/// Prints the underlying error type, using `Display` and not `Debug`.
impl<E: std::fmt::Display> std::fmt::Debug for ExitDisplay<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<E: std::fmt::Display> From<E> for ExitDisplay<E> {
    fn from(e: E) -> Self {
        ExitDisplay(e)
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

    #[test]
    fn test_exitdisplay() {
        let mut buffer = String::new();
        let error = "some error".to_string();
        let exitdisplay: ExitDisplay<String> = error.into();
        write!(buffer, "{:?}", exitdisplay).unwrap();
        assert_eq!(buffer, "some error");
    }
}
