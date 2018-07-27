extern crate exitfailure;

use exitfailure::ExitDisplay;

fn main() -> Result<(), ExitDisplay<String>> {
    Ok(some_fn()?)
}

fn some_fn() -> Result<(), String> {
    Err("this is an error message".into())
}
