use crate::root::get_sudo;

pub fn install(manager: String, package: String, silent: bool, verbose: bool) {
    if package.is_empty() {
        eprintln!("Usage: install <package>");
        return;
    }

    if !silent {
        println!("yu: Installing package: {}", package);
    }

    let mut install_cmd = gen_install_syntax(manager.clone())
        .arg(package)
        .stdout(if verbose {
            std::process::Stdio::inherit()
        } else {
            std::process::Stdio::null()
        })
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("yu: Failed to execute command");

    install_cmd.wait().expect("Command wasn't running");
}

pub fn gen_install_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = get_sudo(manager.clone());
    // add arguments
    match manager.as_str() {
        "apt" | "dnf" | "yum" | "pacman" | "zypper" => {
            command.arg("install");
            command.arg("-y");
        }
        "apk" => {
            command.arg("add");
        }
        "portage" => {
            command.arg("emerge");
        }
        "brew" => {
            command.arg("install");
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
    fn test_gen_install_syntax_apt() {
        let cmd = gen_install_syntax("apt".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["sudo", "apt", "install", "-y"]);
    }

    #[test]
    fn test_gen_install_syntax_dnf() {
        let cmd = gen_install_syntax("dnf".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["sudo", "dnf", "install", "-y"]);
    }

    #[test]
    fn test_gen_install_syntax_yum() {
        let cmd = gen_install_syntax("yum".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["sudo", "yum", "install", "-y"]);
    }

    #[test]
    fn test_gen_install_syntax_pacman() {
        let cmd = gen_install_syntax("pacman".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["sudo", "pacman", "install", "-y"]);
    }

    #[test]
    fn test_gen_install_syntax_zypper() {
        let cmd = gen_install_syntax("zypper".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["sudo", "zypper", "install", "-y"]);
    }

    #[test]
    fn test_gen_install_syntax_apk() {
        let cmd = gen_install_syntax("apk".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["sudo", "apk", "add"]);
    }

    #[test]
    fn test_gen_install_syntax_portage() {
        let cmd = gen_install_syntax("portage".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["sudo", "portage", "emerge"]);
    }

    #[test]
    fn test_gen_install_syntax_brew() {
        let cmd = gen_install_syntax("brew".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["brew", "install"]);
    }
}
