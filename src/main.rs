use clap::{Arg, Command};

mod root;
mod command;
mod env;

use root::get_sudo;
use std::process::Command as SysCommand;

fn main() {
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
    if package_manager == "Unknown" {
        eprintln!("Unknown package manager");
        return;
    }

    let command = matches
        .get_one::<String>("command")
        .map(|s| s.as_str())
        .unwrap_or("upgrade");
    let package = matches
        .get_one::<String>("package")
        .unwrap_or(&"".to_string())
        .to_string();
    let silent = *matches.get_one::<bool>("silent").unwrap_or(&false);
    let verbose = *matches.get_one::<bool>("verbose").unwrap_or(&false);

    match command {
        "autoremove" => {
            let raw = command::autoremove::gen_autoremove_syntax(package_manager.clone());
            run_package_command(raw, "autoremove", silent, verbose, package);
        }
        "upgrade" => {
            let raw = command::upgrade::gen_upgrade_syntax(package_manager.clone());
            run_package_command(raw, "upgrade", silent, verbose, package);
        }
        "update" => {
            let raw = command::update::gen_update_syntax(package_manager.clone());
            run_package_command(raw, "update", silent, verbose, package);
        }
        "install" => {
            let raw = command::install::gen_install_syntax(package_manager.clone());
            run_package_command(raw, "install", silent, verbose, package);
        }
        "uninstall" => {
            let raw = command::uninstall::gen_uninstall_syntax(package_manager.clone());
            run_package_command(raw, "uninstall", silent, verbose, package);
        }
        "reinstall" => {
            let raw = command::reinstall::gen_reinstall_syntax(package_manager.clone());
            run_package_command(raw, "reinstall", silent, verbose, package);
        }
        "info" => {
            let raw = command::info::gen_info_syntax(package_manager.clone());
            run_package_command(raw, "info", silent, verbose, package);
        }
        "list" => {
            let raw = command::list::gen_list_syntax(package_manager.clone());
            run_package_command(raw, "list", silent, verbose, package);
        }
        _ => eprintln!("Unknown command: {}", command),
    }
}

fn run_package_command(mut cmd: SysCommand, action: &str, silent: bool, verbose: bool, package: String) {
    if !package.is_empty() {
        cmd.arg(&package);
    }
    use std::process::Stdio;

    // 控制輸出
    if silent {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
    } else if !verbose {
        cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    }

    let mut cmd = get_sudo(cmd);

    let status = cmd.status().expect(&format!("failed to {}", action));

    if status.success() && !silent {
        println!("yu: {} succeeded", action);
    } else if !status.success() {
        eprintln!("yu: {} failed", action);
    }
}
