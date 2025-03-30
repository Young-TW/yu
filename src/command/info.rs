use std::process::Command;

pub fn gen_info_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = Command::new(manager.clone());
    match manager.as_str() {
        "apt" => {
            command.arg("show");
        }
        "dnf" => {
            command.arg("info");
        }
        "yum" => {
            command.arg("info");
        }
        "pacman" => {
            command.arg("-Qi");
        }
        "zypper" => {
            command.arg("info");
        }
        "apk" => {
            command.arg("info");
        }
        "portage" => {
            command.arg("info");
        }
        "brew" => {
            command.arg("info");
        }
        _ => {
            println!("Unknown package manager: {}", manager);
            command = std::process::Command::new("");
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
    fn test_gen_info_syntax_apt() {
        let cmd = gen_info_syntax("apt".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["apt", "show"]);
    }

    #[test]
    fn test_gen_info_syntax_dnf() {
        let cmd = gen_info_syntax("dnf".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["dnf", "info"]);
    }

    #[test]
    fn test_gen_info_syntax_pacman() {
        let cmd = gen_info_syntax("pacman".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["pacman", "-Qi"]);
    }

    #[test]
    fn test_gen_info_syntax_zypper() {
        let cmd = gen_info_syntax("zypper".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["zypper", "info"]);
    }

    #[test]
    fn test_gen_info_syntax_apk() {
        let cmd = gen_info_syntax("apk".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["apk", "info"]);
    }

    #[test]
    fn test_gen_info_syntax_portage() {
        let cmd = gen_info_syntax("portage".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["portage", "info"]);
    }

    #[test]
    fn test_gen_info_syntax_brew() {
        let cmd = gen_info_syntax("brew".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["brew", "info"]);
    }

    #[test]
    fn test_gen_info_syntax_unknown() {
        let cmd = gen_info_syntax("unknown".to_string());
        let args = cmd_to_string(&cmd);
        assert!(args.is_empty());
    }
}
