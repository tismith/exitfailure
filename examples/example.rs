// Copyright (c) 2018 Toby Smith <toby@tismith.id.au>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate exitfailure;
extern crate failure;

use failure::ResultExt;

fn main() -> exitfailure::Result {
    Ok(some_fn()?)
}

fn some_fn() -> Result<(), failure::Error> {
    let error = Err(failure::err_msg("root cause failure"));
    Ok(error.context("this is some context".to_string())?)
}
