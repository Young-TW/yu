use std::process::Command;

pub fn gen_autoremove_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = Command::new(manager.clone());
    match manager.as_str() {
        "apt" => {
            command.arg("autoremove");
        }
        "dnf" => {
            command.arg("autoremove");
        }
        "yum" => {
            command.arg("autoremove");
        }
        "pacman" => {
            command.arg("-Rns");
            command.arg("--noconfirm");
        }
        "zypper" => {
            command.arg("remove");
        }
        "apk" => {
            command.arg("autoremove");
        }
        "portage" => {
            command.arg("--depclean");
        }
        "brew" => {
            command.arg("cleanup");
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
    fn test_gen_autoremove_syntax_apt() {
        let cmd = gen_autoremove_syntax("apt".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["apt", "autoremove"]);
    }

    #[test]
    fn test_gen_autoremove_syntax_dnf() {
        let cmd = gen_autoremove_syntax("dnf".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["dnf", "autoremove"]);
    }

    #[test]
    fn test_gen_autoremove_syntax_yum() {
        let cmd = gen_autoremove_syntax("yum".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["yum", "autoremove"]);
    }

    #[test]
    fn test_gen_autoremove_syntax_pacman() {
        let cmd = gen_autoremove_syntax("pacman".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["pacman", "-Rns", "--noconfirm"]);
    }

    #[test]
    fn test_gen_autoremove_syntax_zypper() {
        let cmd = gen_autoremove_syntax("zypper".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["zypper", "remove"]);
    }

    #[test]
    fn test_gen_autoremove_syntax_apk() {
        let cmd = gen_autoremove_syntax("apk".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["apk", "autoremove"]);
    }

    #[test]
    fn test_gen_autoremove_syntax_portage() {
        let cmd = gen_autoremove_syntax("portage".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["portage", "--depclean"]);
    }

    #[test]
    fn test_gen_autoremove_syntax_brew() {
        let cmd = gen_autoremove_syntax("brew".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["brew", "cleanup"]);
    }
}

