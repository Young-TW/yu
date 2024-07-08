fn get_sudo(manager: String) -> std::process::Command {
    let command = match manager.as_str() {
        "apt" | "dnf" | "yum" | "pacman" => {
            let mut cmd = std::process::Command::new("sudo");
            cmd.arg(manager.clone());
            cmd
        }
        _ => std::process::Command::new(manager.clone())
    };

    command
}

pub fn gen_install_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = get_sudo(manager.clone());
    // add arguments
    command.arg("install");
    command.arg("-y");
    command
}

pub fn gen_uninstall_syntax(manager: String) -> std::process::Command {
    let command: std::process::Command = get_sudo(manager.clone());
    // add arguments
    command
}

pub fn gen_update_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = get_sudo(manager.clone());
    match manager.as_str() {
        "apt" => {
            command.arg("update");
        },
        "dnf" => {
            command.arg("check-update");
        },
        "yum" => {
            command.arg("check-update");
        },
        "pacman" => {
            command.arg("-Sy");
        },
        "brew" => {
            command.arg("update");
        },
        _ => {
            println!("Unknown package manager: {}", manager);
        }
    }
    command
}

pub fn gen_upgrade_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = get_sudo(manager.clone());
    match manager.as_str() {
        "apt" => {
            command.arg("upgrade");
            command.arg("-y");
        },
        "dnf" => {
            command.arg("upgrade");
            command.arg("-y");
        },
        "yum" => {
            command.arg("upgrade");
            command.arg("-y");
        },
        "pacman" => {
            command.arg("-Syu");
        },
        "brew" => {
            command.arg("upgrade");
        },
        _ => {
            println!("Unknown package manager: {}", manager);
        }
    }

    command
}