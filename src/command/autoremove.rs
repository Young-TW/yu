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
        "zypper" => {
            command.arg("remove");
        }
        "apk" => {
            command.arg("autoremove");
        }
        "portage" => {
            command = Command::new("emerge");
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

/// Returns the query command that lists orphan packages: `pacman -Qdtq`.
pub fn pacman_query_orphans_cmd() -> Command {
    let mut cmd = Command::new("pacman");
    cmd.arg("-Qdtq");
    cmd
}

/// Returns the removal command pre-loaded with `packages`: `pacman -Rns --noconfirm <pkgs>`.
pub fn pacman_remove_orphans_cmd(packages: &[&str]) -> Command {
    let mut cmd = Command::new("pacman");
    cmd.args(["-Rns", "--noconfirm"]);
    for pkg in packages {
        cmd.arg(pkg);
    }
    cmd
}

/// Two-step pacman autoremove: queries orphans then removes them.
/// Exits 0 with no error when there are no orphans to remove.
pub fn run_pacman_autoremove(silent: bool, verbose: bool) -> std::process::ExitCode {
    use std::process::Stdio;

    let mut query = pacman_query_orphans_cmd();
    query.stdout(Stdio::piped());
    if silent {
        query.stderr(Stdio::null());
    } else {
        query.stderr(Stdio::inherit());
    }

    if verbose && !silent {
        eprintln!("yu: running {:?}", query);
    }

    let query_output = match query.output() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("yu: failed to query orphans: {e}");
            return std::process::ExitCode::FAILURE;
        }
    };

    let stdout = String::from_utf8_lossy(&query_output.stdout);
    let packages: Vec<&str> = stdout.split_whitespace().collect();

    if packages.is_empty() {
        if !silent {
            println!("yu: no orphan packages to remove");
        }
        return std::process::ExitCode::SUCCESS;
    }

    let mut remove = pacman_remove_orphans_cmd(&packages);

    if silent {
        remove.stdout(Stdio::null()).stderr(Stdio::null());
    } else {
        remove.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    }

    let mut remove = crate::root::get_sudo(remove);

    if verbose && !silent {
        eprintln!("yu: running {:?}", remove);
    }

    match remove.status() {
        Ok(status) => {
            if status.success() {
                if !silent {
                    println!("yu: autoremove succeeded");
                }
                std::process::ExitCode::SUCCESS
            } else {
                eprintln!("yu: autoremove failed");
                std::process::ExitCode::from(status.code().map(|c| c as u8).unwrap_or(1))
            }
        }
        Err(e) => {
            eprintln!("yu: failed to autoremove: {e}");
            std::process::ExitCode::FAILURE
        }
    }
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
        assert_eq!(args, vec!["emerge", "--depclean"]);
    }

    #[test]
    fn test_gen_autoremove_syntax_brew() {
        let cmd = gen_autoremove_syntax("brew".to_string());
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["brew", "cleanup"]);
    }

    #[test]
    fn test_pacman_query_orphans_cmd() {
        let cmd = pacman_query_orphans_cmd();
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["pacman", "-Qdtq"]);
    }

    #[test]
    fn test_pacman_remove_orphans_cmd() {
        let cmd = pacman_remove_orphans_cmd(&["orphan1", "orphan2"]);
        let args = cmd_to_string(&cmd);
        assert_eq!(
            args,
            vec!["pacman", "-Rns", "--noconfirm", "orphan1", "orphan2"]
        );
    }

    #[test]
    fn test_pacman_remove_orphans_cmd_single_package() {
        let cmd = pacman_remove_orphans_cmd(&["only-orphan"]);
        let args = cmd_to_string(&cmd);
        assert_eq!(args, vec!["pacman", "-Rns", "--noconfirm", "only-orphan"]);
    }
}
