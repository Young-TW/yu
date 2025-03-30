use std::process::Command;

pub fn gen_list_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = Command::new(manager.clone());
    match manager.as_str() {
        "apt" => {
            command.arg("list");
            command.arg("--installed");
        }
        "dnf" => {
            command.arg("list");
            command.arg("installed");
        }
        "yum" => {
            command.arg("list");
            command.arg("installed");
        }
        "pacman" => {
            command.arg("-Q");
        }
        "zypper" => {
            command.arg("search");
            command.arg("--installed-only");
        }
        "apk" => {
            command.arg("info");
            command.arg("--installed");
        }
        "portage" => {
            command.arg("--list");
            command.arg("world");
        }
        "brew" => {
            command.arg("list");
        }
        _ => {
            println!("Unknown package manager: {}", manager);
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
    fn test_gen_list_syntax_apt() {
        let cmd = gen_list_syntax("apt".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["apt", "list", "--installed"]);
    }

    #[test]
    fn test_gen_list_syntax_dnf() {
        let cmd = gen_list_syntax("dnf".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["dnf", "list", "installed"]);
    }

    #[test]
    fn test_gen_list_syntax_yum() {
        let cmd = gen_list_syntax("yum".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["yum", "list", "installed"]);
    }

    #[test]
    fn test_gen_list_syntax_pacman() {
        let cmd = gen_list_syntax("pacman".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["pacman", "-Q"]);
    }

    #[test]
    fn test_gen_list_syntax_zypper() {
        let cmd = gen_list_syntax("zypper".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["zypper", "search", "--installed-only"]);
    }

    #[test]
    fn test_gen_list_syntax_apk() {
        let cmd = gen_list_syntax("apk".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["apk", "info", "--installed"]);
    }

    #[test]
    fn test_gen_list_syntax_portage() {
        let cmd = gen_list_syntax("portage".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["portage", "--list", "world"]);
    }

    #[test]
    fn test_gen_list_syntax_brew() {
        let cmd = gen_list_syntax("brew".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["brew", "list"]);
    }
}
