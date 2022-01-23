mod common;
use common::get_std_out;
use std::process::Command;

#[test]
fn with_no_args() {
    let stdout_str = get_std_out(Command::new("./target/debug/getid").arg("hostname"));
    assert_eq!(stdout_str.split('-').count(), 3);
}

#[test]
fn with_token_length() {
    let stdout_str = get_std_out(
        Command::new("./target/debug/getid")
            .arg("hostname")
            .arg("--token_length")
            .arg("0"),
    );
    assert_eq!(stdout_str.split('-').count(), 2);
}

#[test]
fn with_help() {
    let stdout_str = get_std_out(
        Command::new("./target/debug/getid")
            .arg("hostname")
            .arg("-h"),
    );
    assert_eq!(true, stdout_str.contains("getid hostname --"));
}

#[test]
fn with_alias() {
    let stdout_str = get_std_out(Command::new("./target/debug/getid").arg("heroku").arg("-h"));
    assert_eq!(true, stdout_str.contains("getid hostname --"));
}
