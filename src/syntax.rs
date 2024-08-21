fn get_sudo(manager: String) -> std::process::Command {
    let command = match manager.as_str() {
        "apt" | "dnf" | "yum" | "pacman" | "zypper" | "apk" | "portage" => {
            let mut cmd = std::process::Command::new("sudo");
            cmd.arg(manager.clone());
            cmd
        }
        _ => std::process::Command::new(manager.clone()),
    };

    command
}

pub fn gen_install_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = get_sudo(manager.clone());
    // add arguments
    match manager.as_str() {
        "apt" | "dnf" | "yum" | "pacman" | "zypper" => {
            command.arg("install");
            command.arg("-y");
        }
        "apk" => {
            command.arg("add");
        }
        "portage" => {
            command.arg("emerge");
        }
        "brew" => {
            command.arg("install");
        }
        _ => {
            println!("Unknown package manager: {}", manager);
        }
    }
    command
}

pub fn gen_uninstall_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = get_sudo(manager.clone());
    // add arguments
    match manager.as_str() {
        "apt" | "dnf" | "yum" | "pacman" | "zypper" => {
            command.arg("remove");
            command.arg("-y");
        }
        "apk" => {
            command.arg("del");
        }
        "portage" => {
            command.arg("emerge");
            command.arg("--unmerge");
        }
        "brew" => {
            command.arg("uninstall");
        }
        _ => {
            println!("Unknown package manager: {}", manager);
        }
    }

    command
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
