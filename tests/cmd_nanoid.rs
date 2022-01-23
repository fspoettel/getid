mod common;
use common::get_std_out;
use std::process::Command;

#[test]
fn with_no_args() {
    let stdout_str = get_std_out(Command::new("./target/debug/getid").arg("nanoid"));
    assert_eq!(stdout_str.len(), 21);
}

#[test]
fn with_length() {
    let stdout_str = get_std_out(
        Command::new("./target/debug/getid")
            .arg("nanoid")
            .arg("--length")
            .arg("10"),
    );
    assert_eq!(stdout_str.len(), 10);
}

#[test]
fn with_help() {
    let stdout_str = get_std_out(Command::new("./target/debug/getid").arg("nanoid").arg("-h"));
    assert_eq!(true, stdout_str.contains("getid nanoid --"));
}

#[test]
fn with_alias() {
    let stdout_str = get_std_out(Command::new("./target/debug/getid").arg("nano").arg("-h"));
    assert_eq!(true, stdout_str.contains("getid nanoid --"));
}
