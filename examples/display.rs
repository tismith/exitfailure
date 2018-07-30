extern crate exitfailure;

use exitfailure::ExitDisplay;

fn main() -> Result<(), ExitDisplay> {
    Ok(some_fn()?)
}

fn some_fn() -> Result<(), impl std::fmt::Display> {
    Err("this is an error message")
}
