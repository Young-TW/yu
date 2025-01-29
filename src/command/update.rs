use crate::root::get_sudo;

pub fn update(manager: String, silent: bool, verbose: bool) {
    if !silent {
        println!("yu: Updating system");
    }
    let mut update_cmd = gen_update_syntax(manager.clone())
        .stdout(if verbose {
            std::process::Stdio::inherit()
        } else {
            std::process::Stdio::null()
        })
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("Failed to execute update command");

    update_cmd.wait().expect("Update command wasn't running");
}

pub fn gen_update_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = get_sudo(manager.clone());
    match manager.as_str() {
        "apt" => {
            command.arg("update");
        }
        "dnf" | "yum" => {
            command.arg("check-update");
        }
        "pacman" => {
            command.arg("-Sy");
        }
        "zypper" => {
            command.arg("refresh");
        }
        "apk" => {
            command.arg("update");
        }
        "portage" => {
            command.arg("sync");
        }
        "brew" => {
            command.arg("update");
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
    fn test_gen_update_syntax_apt() {
        let cmd = gen_update_syntax("apt".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["sudo", "apt", "update"]);
    }

    #[test]
    fn test_gen_update_syntax_dnf() {
        let cmd = gen_update_syntax("dnf".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["sudo", "dnf", "check-update"]);
    }

    #[test]
    fn test_gen_update_syntax_pacman() {
        let cmd = gen_update_syntax("pacman".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["sudo", "pacman", "-Sy"]);
    }

    #[test]
    fn test_gen_update_syntax_zypper() {
        let cmd = gen_update_syntax("zypper".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["sudo", "zypper", "refresh"]);
    }

    #[test]
    fn test_gen_update_syntax_apk() {
        let cmd = gen_update_syntax("apk".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["sudo", "apk", "update"]);
    }

    #[test]
    fn test_gen_update_syntax_portage() {
        let cmd = gen_update_syntax("portage".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["sudo", "portage", "sync"]);
    }

    #[test]
    fn test_gen_update_syntax_brew() {
        let cmd = gen_update_syntax("brew".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["brew", "update"]); // brew 不需要 sudo
    }

    #[test]
    fn test_gen_update_syntax_unknown() {
        let cmd = gen_update_syntax("unknown".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["unknown"]); // 什麼都不應該加
    }
}
