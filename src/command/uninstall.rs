use std::process::Command;

pub fn gen_uninstall_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = Command::new(manager.clone());
    // add arguments
    match manager.as_str() {
        "apt" | "dnf" | "yum" | "zypper" => {
            command.arg("remove");
            command.arg("-y");
        }
        "apk" => {
            command.arg("del");
        }
        "portage" => {
            command = Command::new("emerge");
            command.arg("--unmerge");
        }
        "pacman" => {
            command.arg("-R");
            command.arg("-y");
        }
        "brew" => {
            command.arg("uninstall");
        }
        _ => {
            println!("Unknown package manager: {}", manager);
            command = Command::new("");
        }
    }

    command
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    fn cmd_to_string(cmd: &Command) -> Vec<String> {
        let program = cmd.get_program().to_string_lossy().to_string();
        if program.is_empty() {
            return vec![];
        }
        let mut output = vec![program];
        output.extend(cmd.get_args().map(|s| s.to_string_lossy().to_string()));
        output
    }

    #[test]
    fn test_gen_uninstall_syntax_apt() {
        let cmd = gen_uninstall_syntax("apt".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["apt", "remove", "-y"]);
    }

    #[test]
    fn test_gen_uninstall_syntax_dnf() {
        let cmd = gen_uninstall_syntax("dnf".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["dnf", "remove", "-y"]);
    }

    #[test]
    fn test_gen_uninstall_syntax_pacman() {
        let cmd = gen_uninstall_syntax("pacman".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["pacman", "-R", "-y"]);
    }

    #[test]
    fn test_gen_uninstall_syntax_zypper() {
        let cmd = gen_uninstall_syntax("zypper".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["zypper", "remove", "-y"]);
    }

    #[test]
    fn test_gen_uninstall_syntax_apk() {
        let cmd = gen_uninstall_syntax("apk".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["apk", "del"]);
    }

    #[test]
    fn test_gen_uninstall_syntax_portage() {
        let cmd = gen_uninstall_syntax("portage".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["emerge", "--unmerge"]);
    }

    #[test]
    fn test_gen_uninstall_syntax_brew() {
        let cmd = gen_uninstall_syntax("brew".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["brew", "uninstall"]);
    }

    #[test]
    fn test_gen_uninstall_syntax_unknown() {
        let cmd = gen_uninstall_syntax("unknown".to_string());
        let args = cmd_to_string(&cmd);
        assert!(args.is_empty());
    }
}
