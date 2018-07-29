// Copyright (c) 2018 Toby Smith <toby@tismith.id.au>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate assert_cmd;
extern crate predicates;
use assert_cmd::prelude::*;
use predicates::prelude::*;

macro_rules! test_body {
    ($name:expr, $matcher:expr) => {
        let bin = assert_cmd::cargo::cargo_example_path($name).unwrap();
        let pred = predicates::str::contains($matcher).from_utf8();
        std::process::Command::new(bin)
            .assert()
            .failure()
            .stderr(pred);
    };
    ($name:expr) => {
        test_body!(
            $name,
            "Error: this is some context\nInfo: caused by root cause failure"
        )
    };
}

#[test]
fn test_example() {
    test_body!("example");
}

#[test]
fn test_context() {
    test_body!("context");
}

#[test]
fn test_display() {
    test_body!("display", "Error: this is an error message");
}

#[test]
fn test_no_backtrace() {
    let bin = assert_cmd::cargo::cargo_example_path("example").unwrap();
    let pred = predicates::str::contains("stack backtrace").from_utf8();
    std::process::Command::new(bin)
        .env_remove("RUST_BACKTRACE")
        .assert()
        .failure()
        .stderr(pred.not());
}

#[test]
fn test_backtrace() {
    let bin = assert_cmd::cargo::cargo_example_path("example").unwrap();
    let pred = predicates::str::contains("stack backtrace").from_utf8();
    std::process::Command::new(bin)
        .env("RUST_BACKTRACE", "1")
        .assert()
        .failure()
        .stderr(pred);
}
