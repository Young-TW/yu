pub fn gen_install_syntax(manager: String) -> std::process::Command {
    let command: std::process::Command = std::process::Command::new(manager);
    // add arguments
    command
}

pub fn gen_uninstall_syntax(manager: String) -> std::process::Command {
    let command: std::process::Command = std::process::Command::new(manager);
    // add arguments
    command
}

pub fn gen_update_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = std::process::Command::new(manager.clone());
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
        "paru" => {
            // pass
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
    let mut command: std::process::Command = std::process::Command::new(manager.clone());
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
        "paru" => {
            // pass
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