# exitfailure - convienent newtype wrapper for failure::Error

`exitfailure` provides a newtype wrapper around `failure::Error` that will print a formatted list of error causes in it's `Debug` trait implementation.

It is intended to be used with rust 1.26 and above's "? in main()" feature (see the [tracking issue here][https://github.com/rust-lang/rust/issues/43301]).

Example:
```rust
#[macro use] extern crate failure;
extern crate exitfailure;

use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure> {
	let error = err_msg!("root cause failure");
	Ok(error.context("this is some context")?)
}
```

This will print, when executed:
```ignore
Error: this is some context
caused by: root cause failure
```
