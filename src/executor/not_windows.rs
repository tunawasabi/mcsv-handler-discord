use std::process::Command;

pub fn command_new(program: &str) -> Command {
    let mut cmd = Command::new("sh");
    cmd.args(["-c", program]);
    cmd
}
