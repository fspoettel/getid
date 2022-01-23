use std::process::Command;

pub fn get_std_out(cmd: &mut Command) -> String {
    let output = cmd.output().unwrap();
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}
