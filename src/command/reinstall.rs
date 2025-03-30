use std::process::Command;

pub fn gen_reinstall_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = Command::new(manager.clone());
    match manager.as_str() {
        "apt" | "dnf" | "yum" => {
            command.arg("reinstall");
            command.arg("-y");
        }
        "pacman" => {
            command.arg("-S");
            command.arg("--overwrite");
            command.arg("*");
        }
        "zypper" => {
            command.arg("install");
            command.arg("-y");
        }
        "apk" => {
            command.arg("add");
            command.arg("-f");
        }
        "portage" => {
            command = Command::new("emerge");
            command.arg("--oneshot");
        }
        "brew" => {
            command.arg("reinstall");
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
    fn test_gen_reinstall_syntax_apt() {
        let cmd = gen_reinstall_syntax("apt".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["apt", "reinstall", "-y"]);
    }

    #[test]
    fn test_gen_reinstall_syntax_dnf() {
        let cmd = gen_reinstall_syntax("dnf".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["dnf", "reinstall", "-y"]);
    }

    #[test]
    fn test_gen_reinstall_syntax_pacman() {
        let cmd = gen_reinstall_syntax("pacman".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["pacman", "-S", "--overwrite", "*"]);
    }

    #[test]
    fn test_gen_reinstall_syntax_zypper() {
        let cmd = gen_reinstall_syntax("zypper".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["zypper", "install", "-y"]);
    }

    #[test]
    fn test_gen_reinstall_syntax_apk() {
        let cmd = gen_reinstall_syntax("apk".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["apk", "add", "-f"]);
    }

    #[test]
    fn test_gen_reinstall_syntax_portage() {
        let cmd = gen_reinstall_syntax("portage".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["emerge", "--oneshot"]);
    }

    #[test]
    fn test_gen_reinstall_syntax_brew() {
        let cmd = gen_reinstall_syntax("brew".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["brew", "reinstall"]);
    }

    #[test]
    fn test_gen_reinstall_syntax_unknown() {
        let cmd = gen_reinstall_syntax("unknown".to_string());
        let args = cmd_to_string(&cmd);
        assert!(args.is_empty());
    }
}
