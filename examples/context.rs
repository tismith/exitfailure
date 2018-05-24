extern crate exitfailure;
extern crate failure;

use exitfailure::ExitFailure;
use failure::ResultExt;

fn main() -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg("root cause failure"));
    Ok(error.context("this is some context".to_string())?)
}
