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
