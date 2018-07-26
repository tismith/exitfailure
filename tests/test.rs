// Copyright (c) 2018 Toby Smith <toby@tismith.id.au>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate assert_cli;

//kcov doesn't play nice with assert_cli() see
//https://github.com/assert-rs/assert_cli/issues/101
use std::env;
fn get_cwd() -> String {
    env::current_dir().unwrap().to_str().unwrap().to_string()
}

#[test]
fn test_example() {
    let bin: &str = &format!("{}/target/debug/examples/example", get_cwd());
    assert_cli::Assert::command(&[bin])
        .fails()
        .stderr()
        .contains("Error: this is some context\nInfo: caused by root cause failure")
        .unwrap();
}

#[test]
fn test_context() {
    let bin: &str = &format!("{}/target/debug/examples/context", get_cwd());
    assert_cli::Assert::command(&[bin])
        .fails()
        .stderr()
        .contains("Error: this is some context\nInfo: caused by root cause failure")
        .unwrap();
}
