mod common;
use common::get_std_out;
use std::{process::Command, str::FromStr};
use uuid::{Uuid, Version};

#[test]
fn with_no_args() {
    let stdout_str = get_std_out(Command::new("./target/debug/getid").arg("uuidv4"));
    let uuid = Uuid::from_str(&stdout_str).unwrap();
    assert_eq!(uuid.get_version().unwrap(), Version::Random);
}

#[test]
fn with_urn() {
    let stdout_str = get_std_out(
        Command::new("./target/debug/getid")
            .arg("uuidv4")
            .arg("--urn"),
    );
    let uuid = Uuid::from_str(&stdout_str).unwrap();
    assert_eq!(true, stdout_str.starts_with("urn:"));
    assert_eq!(uuid.get_version().unwrap(), Version::Random);
}

#[test]
fn with_simple() {
    let stdout_str = get_std_out(
        Command::new("./target/debug/getid")
            .arg("uuidv4")
            .arg("--simple"),
    );
    let uuid = Uuid::from_str(&stdout_str).unwrap();
    assert_eq!(false, stdout_str.contains('-'));
    assert_eq!(uuid.get_version().unwrap(), Version::Random);
}

#[test]
fn with_help() {
    let stdout_str = get_std_out(Command::new("./target/debug/getid").arg("uuidv4").arg("-h"));
    assert_eq!(true, stdout_str.contains("getid uuidv4 --"));
}

#[test]
fn with_alias() {
    let stdout_str = get_std_out(Command::new("./target/debug/getid").arg("uuid").arg("-h"));
    assert_eq!(true, stdout_str.contains("getid uuidv4 --"));
}
