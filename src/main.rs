use clap::{Arg, Command};

mod env;
mod syntax;

fn main() {
    let matches = Command::new("yu")
        .version("0.1.0")
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
                .short('S')
                .help("Run the command silently")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('V')
                .help("Run the command with verbose output")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .help("Show the version information")
                .action(clap::ArgAction::Version),
        )
        .get_matches();

    let package_manager = env::detect_package_manager();

    let command = matches.get_one::<String>("command").map(|s| s.as_str()).unwrap_or("upgrade");
    let package = matches.get_one::<String>("package").unwrap_or(&"".to_string()).to_string();
    let silent = *matches.get_one::<bool>("silent").unwrap_or(&false);
    let verbose = *matches.get_one::<bool>("verbose").unwrap_or(&false);

    match command {
        "install" => install(package_manager.clone(), package, silent, verbose),
        "uninstall" => uninstall(package_manager.clone(), package, silent, verbose),
        "upgrade" => upgrade(package_manager.clone(), silent, verbose),
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

fn install(manager: String, package: String, silent: bool, verbose: bool) {
    if package.is_empty() {
        eprintln!("Usage: install <package>");
        return;
    }

    if !silent {
        println!("yu: Installing package: {}", package);
    }

    let mut cmd = syntax::gen_install_syntax(manager.clone())
        .arg(package)
        .stdout(if verbose { std::process::Stdio::inherit() } else { std::process::Stdio::null() })
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("yu: Failed to execute command");

    cmd.wait().expect("Command wasn't running");
}

fn uninstall(manager: String, package: String, silent: bool, verbose: bool) {
    if package.is_empty() {
        eprintln!("Usage: uninstall <package>");
        return;
    }

    if !silent {
        println!("yu: Uninstalling package: {}", package);
    }

    let mut cmd = syntax::gen_uninstall_syntax(manager.clone())
        .arg(package)
        .stdout(if verbose { std::process::Stdio::inherit() } else { std::process::Stdio::null() })
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("yu: Failed to execute command");

    cmd.wait().expect("Command wasn't running");
}

fn upgrade(manager: String, silent: bool, verbose: bool) {
    if !silent {
        println!("yu: Updating system");
    }
    let mut update_cmd = syntax::gen_update_syntax(manager.clone())
        .stdout(if verbose { std::process::Stdio::inherit() } else { std::process::Stdio::null() })
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("Failed to execute update command");

    update_cmd.wait().expect("Update command wasn't running");

    if !silent {
        println!("yu: Upgrading system");
    }
    let mut upgrade_cmd = syntax::gen_upgrade_syntax(manager.clone())
        .stdout(if verbose { std::process::Stdio::inherit() } else { std::process::Stdio::null() })
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("Failed to execute upgrade command");

    upgrade_cmd.wait().expect("Upgrade command wasn't running");

    if !silent {
        println!("yu: System upgraded");
    }
}
