use std::process::Command;

pub fn gen_upgrade_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = Command::new(manager.clone());
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    fn cmd_to_string(cmd: &Command) -> Vec<String> {
        let mut output = vec![cmd.get_program().to_string_lossy().to_string()];
        output.extend(cmd.get_args().map(|s| s.to_string_lossy().to_string()));
        output
    }

    #[test]
    fn test_gen_upgrade_syntax_apt() {
        let cmd = gen_upgrade_syntax("apt".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["apt", "upgrade", "-y"]);
    }

    #[test]
    fn test_gen_upgrade_syntax_dnf() {
        let cmd = gen_upgrade_syntax("dnf".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["dnf", "upgrade", "-y"]);
    }

    #[test]
    fn test_gen_upgrade_syntax_pacman() {
        let cmd = gen_upgrade_syntax("pacman".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["pacman", "-Syu"]);
    }

    #[test]
    fn test_gen_upgrade_syntax_zypper() {
        let cmd = gen_upgrade_syntax("zypper".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["zypper", "update"]);
    }

    #[test]
    fn test_gen_upgrade_syntax_apk() {
        let cmd = gen_upgrade_syntax("apk".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["apk", "upgrade"]);
    }

    #[test]
    fn test_gen_upgrade_syntax_portage() {
        let cmd = gen_upgrade_syntax("portage".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(
            args,
            vec!["portage", "world", "--update", "--deep", "--newuse"]
        );
    }

    #[test]
    fn test_gen_upgrade_syntax_brew() {
        let cmd = gen_upgrade_syntax("brew".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["brew", "upgrade"]); // brew 不需要 sudo
    }

    #[test]
    fn test_gen_upgrade_syntax_unknown() {
        let cmd = gen_upgrade_syntax("unknown".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["unknown"]); // 不應該有任何參數
    }
}
