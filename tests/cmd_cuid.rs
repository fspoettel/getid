mod common;
use common::get_std_out;
use cuid::{is_cuid, is_slug};
use std::process::Command;

#[test]
fn with_no_args() {
    let stdout_str = get_std_out(Command::new("./target/debug/getid").arg("cuid"));
    assert_eq!(true, is_cuid(stdout_str));
}

#[test]
fn with_slug() {
    let stdout_str = get_std_out(
        Command::new("./target/debug/getid")
            .arg("cuid")
            .arg("--slug"),
    );
    assert_eq!(true, is_slug(stdout_str));
}

#[test]
fn with_help() {
    let stdout_str = get_std_out(Command::new("./target/debug/getid").arg("cuid").arg("-h"));
    assert_eq!(true, stdout_str.contains("getid cuid --"));
}
