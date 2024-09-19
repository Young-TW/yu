use crate::root::get_sudo;

pub fn list(manager: String) {
    let mut list_cmd = gen_list_syntax(manager.clone())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("yu: Failed to execute command");

    list_cmd.wait().expect("Command wasn't running");
}

pub fn gen_list_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = get_sudo(manager.clone());
    match manager.as_str() {
        "apt" => {
            command.arg("list");
            command.arg("--installed");
        }
        "dnf" => {
            command.arg("list");
            command.arg("installed");
        }
        "yum" => {
            command.arg("list");
            command.arg("installed");
        }
        "pacman" => {
            command.arg("-Q");
        }
        "zypper" => {
            command.arg("search");
            command.arg("--installed-only");
        }
        "apk" => {
            command.arg("info");
            command.arg("--installed");
        }
        "portage" => {
            command.arg("--list");
            command.arg("world");
        }
        "brew" => {
            command.arg("list");
        }
        _ => {
            println!("Unknown package manager: {}", manager);
        }
    }
    command
}
