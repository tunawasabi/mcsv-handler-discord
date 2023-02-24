use std::process::Command;

pub fn command_new(program: &str) -> Command {
    let mut cmd = Command::new("cmd");
    cmd.args(["/C", program]);
    cmd
}

fn firewall_process_new() -> Command {
    let mut cmd = command_new("netsh");
    cmd.arg("advfirewall").arg("firewall");
    cmd
}

fn rule_name_arg(port: u16) -> String {
    format!("name=mcsv-handler-discord-{}", port)
}

pub fn open_port(port: u16) {
    println!("ポートの開放");

    let add_port_rule = |port: u16| {
        let mut cmd = firewall_process_new();
        cmd.args(["add", "rule"])
            .arg(rule_name_arg(port))
            .arg("action=allow")
            .arg("protocol=TCP")
            .arg(format!("localport={}", port));
        cmd
    };

    add_port_rule(port).arg("dir=in").status().ok();
    add_port_rule(port).arg("dir=out").status().ok();
}

pub fn close_port(port: u16) {
    println!("ポートの戸締り");

    let delete_rule = || {
        let mut cmd = firewall_process_new();
        cmd.args(["delete", "rule"]).arg(rule_name_arg(port));
        cmd
    };

    delete_rule().status().ok();
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;

    use super::command_new;

    #[test]
    fn invoked_cmd_on_windows() {
        let cmd = command_new("java");

        assert!(cmd.get_program() == "cmd");
        assert_eq!(cmd.get_args().collect::<Vec<&OsStr>>(), &["/C", "java"]);
    }
}
