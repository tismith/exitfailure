# exitfailure - convenient newtype wrapper for failure::Error

[![Build Status](https://travis-ci.org/tismith/exitfailure.svg?branch=master)](https://travis-ci.org/tismith/exitfailure)
[![codecov](https://codecov.io/gh/tismith/exitfailure/branch/master/graph/badge.svg)](https://codecov.io/gh/tismith/exitfailure)
[![](http://meritbadge.herokuapp.com/exitfailure)](https://crates.io/crates/exitfailure)

`exitfailure` provides a newtype wrapper around `failure::Error` that will print a formatted list of error causes in it's `Debug` trait implementation.

It is intended to be used with rust 1.26 and above's "? in main()" feature (see the [tracking issue here](https://github.com/rust-lang/rust/issues/43301)).

Example:
```rust
#[macro use] extern crate failure;
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
```

This will print, when executed:
```ignore
Error: this is some context
caused by: root cause failure
```

If the environment variable RUST_BACKTRACE=1 is set, then the printing will
include whatever backtrace information is provided by the `failure::Error`
that is being wrapped.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
