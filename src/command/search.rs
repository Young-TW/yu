use std::process::Command;

pub fn gen_search_syntax(manager: String) -> std::process::Command {
    let mut command = Command::new(manager.clone());
    match manager.as_str() {
        "apt" => {
            command.arg("search");
        }
        "dnf" => {
            command.arg("search");
        }
        "yum" => {
            command.arg("search");
        }
        "pacman" => {
            command.arg("-Ss");
        }
        "zypper" => {
            command.arg("search");
        }
        "apk" => {
            command.arg("search");
        }
        "portage" => {
            command.arg("search");
        }
        "brew" => {
            command.arg("search");
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
    fn test_gen_search_syntax_apt() {
        let cmd = gen_search_syntax("apt".to_string());
        let expected = vec!["apt", "search"];
        assert_eq!(cmd_to_string(&cmd), expected);
    }

    #[test]
    fn test_gen_search_syntax_dnf() {
        let cmd = gen_search_syntax("dnf".to_string());
        let expected = vec!["dnf", "search"];
        assert_eq!(cmd_to_string(&cmd), expected);
    }

    #[test]
    fn test_gen_search_syntax_yum() {
        let cmd = gen_search_syntax("yum".to_string());
        let expected = vec!["yum", "search"];
        assert_eq!(cmd_to_string(&cmd), expected);
    }

    #[test]
    fn test_gen_search_syntax_pacman() {
        let cmd = gen_search_syntax("pacman".to_string());
        let expected = vec!["pacman", "-Ss"];
        assert_eq!(cmd_to_string(&cmd), expected);
    }

    #[test]
    fn test_gen_search_syntax_zypper() {
        let cmd = gen_search_syntax("zypper".to_string());
        let expected = vec!["zypper", "search"];
        assert_eq!(cmd_to_string(&cmd), expected);
    }

    #[test]
    fn test_gen_search_syntax_apk() {
        let cmd = gen_search_syntax("apk".to_string());
        let expected = vec!["apk", "search"];
        assert_eq!(cmd_to_string(&cmd), expected);
    }

    #[test]
    fn test_gen_search_syntax_portage() {
        let cmd = gen_search_syntax("portage".to_string());
        let expected = vec!["portage", "search"];
        assert_eq!(cmd_to_string(&cmd), expected);
    }

    #[test]
    fn test_gen_search_syntax_brew() {
        let cmd = gen_search_syntax("brew".to_string());
        let expected = vec!["brew", "search"];
        assert_eq!(cmd_to_string(&cmd), expected);
    }
}