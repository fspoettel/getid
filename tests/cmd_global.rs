mod common;
use common::get_std_out;
use std::process::Command;

#[test]
fn with_no_args() {
    let stdout_str = get_std_out(&mut Command::new("./target/debug/getid"));
    assert_eq!(true, stdout_str.contains("getid --"));
    assert_eq!(true, stdout_str.contains("Show this help again."));
}

#[test]
fn with_help() {
    let stdout_str = get_std_out(Command::new("./target/debug/getid").arg("-h"));
    assert_eq!(true, stdout_str.contains("getid --"));
    assert_eq!(true, stdout_str.contains("Show this help again."));
}

#[test]
fn with_version() {
    let stdout_str = get_std_out(Command::new("./target/debug/getid").arg("-v"));
    assert_eq!(true, stdout_str.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn with_unknown_command() {
    let cmd = Command::new("./target/debug/getid")
        .arg("does_not_exist")
        .output()
        .unwrap();
    assert_eq!(cmd.status.code(), Some(1));
    assert_eq!(
        true,
        String::from_utf8(cmd.stderr)
            .unwrap()
            .contains("unknown subcommand")
    );
}

#[test]
fn with_unknown_flag() {
    let cmd = Command::new("./target/debug/getid")
        .arg("cuid")
        .arg("--unknown_flag")
        .output()
        .unwrap();
    assert_eq!(
        true,
        String::from_utf8(cmd.stderr)
            .unwrap()
            .contains("unknown argument")
    );
}
