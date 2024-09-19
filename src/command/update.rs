use crate::root::get_sudo;

pub fn update(manager: String, silent: bool, verbose: bool) {
    if !silent {
        println!("yu: Updating system");
    }
    let mut update_cmd = gen_update_syntax(manager.clone())
        .stdout(if verbose { std::process::Stdio::inherit() } else { std::process::Stdio::null() })
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("Failed to execute update command");

    update_cmd.wait().expect("Update command wasn't running");
}

pub fn gen_update_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = get_sudo(manager.clone());
    match manager.as_str() {
        "apt" => {
            command.arg("update");
        }
        "dnf" | "yum" => {
            command.arg("check-update");
        }
        "pacman" => {
            command.arg("-Sy");
        }
        "zypper" => {
            command.arg("refresh");
        }
        "apk" => {
            command.arg("update");
        }
        "portage" => {
            command.arg("sync");
        }
        "brew" => {
            command.arg("update");
        }
        _ => {
            println!("Unknown package manager: {}", manager);
        }
    }
    command
}
