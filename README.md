# exitfailure - convenient newtype wrappers for using ? in main()

[![Build Status](https://travis-ci.org/tismith/exitfailure.svg?branch=master)](https://travis-ci.org/tismith/exitfailure)
[![Build status](https://ci.appveyor.com/api/projects/status/2xhxwps2swlj3git/branch/master?svg=true)](https://ci.appveyor.com/project/tismith/exitfailure/branch/master)
[![codecov](https://codecov.io/gh/tismith/exitfailure/branch/master/graph/badge.svg)](https://codecov.io/gh/tismith/exitfailure)
[![](http://meritbadge.herokuapp.com/exitfailure)](https://crates.io/crates/exitfailure)

`exitfailure` provides some newtype wrappers to help with using ? in `main()`.

It is intended to be used with rust 1.26 and above's "? in main()" feature (see the [tracking issue here](https://github.com/rust-lang/rust/issues/43301)).

The primary items exported by this library are:

 - `ExitFailure`: a wrapper around `failure::Error` to allow ? printing from main
    to present a nicer error message, including any available context and backtrace.

 - `ExitDisplay<E>`: a wrapper around `E: std::fmt::Display` to allow the error message
    from main to use `Display` and not `Debug`.

For more information, including more details on the types, please see the [API Documentation](https://docs.rs/exitfailure/).

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
Info: caused by root cause failure
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
