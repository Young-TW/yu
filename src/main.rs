use clap::{Arg, Command};

mod env;
mod root;

mod command {
    pub mod autoremove;
    pub mod info;
    pub mod install;
    pub mod list;
    pub mod reinstall;
    pub mod uninstall;
    pub mod update;
    pub mod upgrade;
}

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
        "autoremove" => command::autoremove::autoremove(package_manager, silent),
        "info" => command::info::info(package_manager, package, silent),
        "install" => command::install::install(package_manager, package, silent, verbose),
        "uninstall" => command::uninstall::uninstall(package_manager, package, silent, verbose),
        "reinstall" => command::reinstall::reinstall(package_manager, package, silent, verbose),
        "upgrade" => command::upgrade::upgrade(package_manager, silent, verbose),
        "update" => command::update::update(package_manager, silent, verbose),
        "list" => command::list::list(package_manager),
        _ => eprintln!("Unknown command: {}", command),
    }
}
