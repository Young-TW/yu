use clap::{Arg, Command};

mod env;
mod syntax;

mod command {
    pub mod install;
    pub mod uninstall;
    pub mod upgrade;
    pub mod update;
}

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
        "install" => command::install::install(package_manager, package, silent, verbose),
        "uninstall" => command::uninstall::uninstall(package_manager, package, silent, verbose),
        "upgrade" => command::upgrade::upgrade(package_manager, silent, verbose),
        "update" => command::update::update(package_manager, silent, verbose),
        _ => eprintln!("Unknown command: {}", command),
    }
}
