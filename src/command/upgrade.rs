use crate::root::get_sudo;

pub fn upgrade(manager: String, silent: bool, verbose: bool) {
    if !silent {
        println!("yu: Upgrading system");
    }
    let mut upgrade_cmd = gen_upgrade_syntax(manager.clone())
        .stdout(if verbose { std::process::Stdio::inherit() } else { std::process::Stdio::null() })
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("Failed to execute upgrade command");

    let upgrade_result = upgrade_cmd.wait().expect("Upgrade command wasn't running");

    if !upgrade_result.success() {
        println!("yu: Failed to upgrade system");
    } else if !silent {
        println!("yu: System upgraded");
    }
}

pub fn gen_upgrade_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = get_sudo(manager.clone());
    match manager.as_str() {
        "apt" | "dnf" | "yum" => {
            command.arg("upgrade");
            command.arg("-y");
        }
        "pacman" => {
            command.arg("-Syu");
        }
        "zypper" => {
            command.arg("update");
        }
        "apk" => {
            command.arg("upgrade");
        }
        "portage" => {
            command.arg("world");
            command.arg("--update");
            command.arg("--deep");
            command.arg("--newuse");
        }
        "brew" => {
            command.arg("upgrade");
        }
        _ => {
            println!("Unknown package manager: {}", manager);
        }
    }

    command
}
