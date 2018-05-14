extern crate failure;
extern crate exitfailure;

use failure::ResultExt;
use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure> {
    Ok(some_fn()?)
}

fn some_fn() -> Result<(), failure::Error> {
    let error = Err(failure::err_msg("root cause failure"));
    Ok(error.context("this is some context".to_string())?)
}

