use crate::root::get_sudo;

pub fn autoremove(manager: String, silent: bool) {
    if !silent {
        println!("yu: Auto removing unused packages")
    }

    let mut autoremove_cmd = gen_autoremove_syntax(manager.clone())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("Failed to execute autoremove command");

    autoremove_cmd
        .wait()
        .expect("Autoremove command wasn't running");
}

pub fn gen_autoremove_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = get_sudo(manager.clone());
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
        assert_eq!(args, vec!["sudo", "apt", "autoremove"]);
    }

    #[test]
    fn test_gen_autoremove_syntax_dnf() {
        let cmd = gen_autoremove_syntax("dnf".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["sudo", "dnf", "autoremove"]);
    }

    #[test]
    fn test_gen_autoremove_syntax_yum() {
        let cmd = gen_autoremove_syntax("yum".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["sudo", "yum", "autoremove"]);
    }

    #[test]
    fn test_gen_autoremove_syntax_pacman() {
        let cmd = gen_autoremove_syntax("pacman".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["sudo", "pacman", "-Rns"]);
    }

    #[test]
    fn test_gen_autoremove_syntax_zypper() {
        let cmd = gen_autoremove_syntax("zypper".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["sudo", "zypper", "remove"]);
    }

    #[test]
    fn test_gen_autoremove_syntax_apk() {
        let cmd = gen_autoremove_syntax("apk".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["sudo", "apk", "autoremove"]);
    }

    #[test]
    fn test_gen_autoremove_syntax_portage() {
        let cmd = gen_autoremove_syntax("portage".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["sudo", "portage", "--depclean"]);
    }

    #[test]
    fn test_gen_autoremove_syntax_brew() {
        let cmd = gen_autoremove_syntax("brew".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["brew", "cleanup"]);
    }
}

