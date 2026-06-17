use clap::{Arg, Command};

mod command;
mod env;
mod root;

use root::get_sudo;
use std::process::{Command as SysCommand, ExitCode};

fn main() -> ExitCode {
    let matches = Command::new("yu")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A simple package manager wrapper")
        .arg(
            Arg::new("command")
                .help("The command to execute")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("package")
                .help("The package to install or uninstall")
                .required(false)
                .index(2),
        )
        .arg(
            Arg::new("silent")
                .long("silent")
                .short('s')
                .help("Run the command silently")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .help("Run the command with verbose output")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let package_manager = env::detect_package_manager();
    if package_manager == "unknown" {
        eprintln!("yu: unknown package manager");
        return ExitCode::FAILURE;
    }

    let command = matches
        .get_one::<String>("command")
        .map(|s| s.as_str())
        .unwrap_or("upgrade");
    let package = matches
        .get_one::<String>("package")
        .cloned()
        .unwrap_or_default();
    let silent = *matches.get_one::<bool>("silent").unwrap_or(&false);
    let verbose = *matches.get_one::<bool>("verbose").unwrap_or(&false);

    match command {
        // Security-sensitive, explicitly opt-in: never reached automatically.
        "setup-sudo" => {
            if root::setup_sudo(&package_manager) {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        // pacman -Rns requires at least one package argument; autoremove must
        // first query orphans with `pacman -Qdtq` and skip removal when none exist.
        "autoremove" if package_manager == "pacman" => {
            command::autoremove::run_pacman_autoremove(silent, verbose)
        }
        // zypper has no direct autoremove equivalent; report unsupported rather
        // than invoking `zypper remove` with no package argument (which zypper
        // rejects as invalid).
        "autoremove" if package_manager == "zypper" => command::autoremove::run_zypper_autoremove(),
        other => match build_command(other, &package_manager) {
            Some(raw) => run_package_command(raw, other, silent, verbose, &package),
            None => {
                eprintln!("yu: unknown command: {other}");
                ExitCode::FAILURE
            }
        },
    }
}

/// Maps a user-facing subcommand and package manager to the raw system
/// command to run, or `None` if the subcommand is not recognised.
fn build_command(command: &str, manager: &str) -> Option<SysCommand> {
    let pm = manager.to_string();
    let cmd = match command {
        "autoremove" => command::autoremove::gen_autoremove_syntax(pm),
        "upgrade" => command::upgrade::gen_upgrade_syntax(pm),
        "update" => command::update::gen_update_syntax(pm),
        "install" => command::install::gen_install_syntax(pm),
        "uninstall" => command::uninstall::gen_uninstall_syntax(pm),
        "reinstall" => command::reinstall::gen_reinstall_syntax(pm),
        "search" => command::search::gen_search_syntax(pm),
        "info" => command::info::gen_info_syntax(pm),
        "list" => command::list::gen_list_syntax(pm),
        _ => return None,
    };
    Some(cmd)
}

/// Appends the optional package argument, wraps the command in `sudo` when
/// required, and spawns it, returning the child's exit status.
fn execute(
    mut cmd: SysCommand,
    package: &str,
    silent: bool,
    verbose: bool,
) -> std::io::Result<std::process::ExitStatus> {
    use std::process::Stdio;

    if !package.is_empty() {
        cmd.arg(package);
    }

    // 控制輸出
    if silent {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
    } else {
        cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    }

    let mut cmd = get_sudo(cmd);
    if verbose && !silent {
        eprintln!("yu: running {:?}", cmd);
    }

    cmd.status()
}

fn run_package_command(
    cmd: SysCommand,
    action: &str,
    silent: bool,
    verbose: bool,
    package: &str,
) -> ExitCode {
    match execute(cmd, package, silent, verbose) {
        Ok(status) => {
            if status.success() {
                if !silent {
                    println!("yu: {action} succeeded");
                }
            } else {
                eprintln!("yu: {action} failed");
            }
            // Surface the child's own exit code so scripts can detect failure.
            ExitCode::from(exit_code(status))
        }
        Err(e) => {
            eprintln!("yu: failed to {action}: {e}");
            ExitCode::FAILURE
        }
    }
}

/// Translates a finished child's status into a process exit code, preserving
/// the child's own code where available (0 on success, the reported code on
/// failure, or 1 when it was terminated without one, e.g. by a signal).
fn exit_code(status: std::process::ExitStatus) -> u8 {
    if status.success() {
        0
    } else {
        status.code().map(|c| c as u8).unwrap_or(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exit_code_preserves_child_status() {
        let ok = SysCommand::new("true").status().expect("spawn true");
        assert_eq!(exit_code(ok), 0);

        let fail = SysCommand::new("false").status().expect("spawn false");
        assert_eq!(exit_code(fail), 1);
    }

    #[test]
    fn build_command_dispatches_known_subcommands() {
        let cmd = build_command("install", "apt").expect("install should be known");
        assert_eq!(cmd.get_program().to_string_lossy(), "apt");

        let cmd = build_command("upgrade", "pacman").expect("upgrade should be known");
        assert_eq!(cmd.get_program().to_string_lossy(), "pacman");
    }

    #[test]
    fn build_command_returns_none_for_unknown_subcommand() {
        assert!(build_command("frobnicate", "apt").is_none());
    }

    #[test]
    fn execute_spawns_and_reports_success() {
        // Actually launches a child process and observes its real exit status.
        let status = execute(SysCommand::new("true"), "", true, false)
            .expect("spawning `true` should not fail");
        assert!(status.success());
    }

    #[test]
    fn execute_spawns_and_reports_failure() {
        let status = execute(SysCommand::new("false"), "", true, false)
            .expect("spawning `false` should not fail");
        assert!(!status.success());
    }

    #[test]
    fn execute_appends_package_argument_and_still_spawns() {
        // `true` ignores its arguments, but the command must still spawn and
        // succeed once a package argument has been appended.
        let status = execute(SysCommand::new("true"), "somepkg", true, false)
            .expect("spawning `true somepkg` should not fail");
        assert!(status.success());
    }

    #[test]
    fn execute_surfaces_spawn_errors_instead_of_panicking() {
        // A binary that does not exist must yield an `Err`, not a panic.
        let result = execute(SysCommand::new("yu-no-such-binary-xyz"), "", true, false);
        assert!(result.is_err());
    }
}
