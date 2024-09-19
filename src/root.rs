pub fn get_sudo(manager: String) -> std::process::Command {
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
