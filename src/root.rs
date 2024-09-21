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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sudo() {
        let manager = "apt".to_string();
        let cmd = get_sudo(manager.clone());
        assert_eq!(cmd.get_program(), "sudo");

        let manager = "dnf".to_string();
        let cmd = get_sudo(manager.clone());
        assert_eq!(cmd.get_program(), "sudo");

        let manager = "yum".to_string();
        let cmd = get_sudo(manager.clone());
        assert_eq!(cmd.get_program(), "sudo");

        let manager = "pacman".to_string();
        let cmd = get_sudo(manager.clone());
        assert_eq!(cmd.get_program(), "sudo");

        let manager = "zypper".to_string();
        let cmd = get_sudo(manager.clone());
        assert_eq!(cmd.get_program(), "sudo");

        let manager = "apk".to_string();
        let cmd = get_sudo(manager.clone());
        assert_eq!(cmd.get_program(), "sudo");

        let manager = "portage".to_string();
        let cmd = get_sudo(manager.clone());
        assert_eq!(cmd.get_program(), "sudo");

        let manager = "brew".to_string();
        let cmd = get_sudo(manager.clone());
        assert_eq!(cmd.get_program(), "brew");
    }
}
