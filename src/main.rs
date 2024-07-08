mod env;
mod syntax;
mod package;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let package_manager = env::detect_package_manager();
    if args.len() < 2 {
        upgrade(package_manager.clone());
        return;
    }

    // package::write_into_dotfile();
    // check second parameter
    match args[1].as_str() {
        "install" => install(package_manager.clone(), args[2].clone()),
        "uninstall" => uninstall(package_manager.clone(), args[2].clone()),
        "upgrade" => upgrade(package_manager.clone()),
        _ => println!("Unknown command: {}", args[1]),
    }
    0;
}

fn install(manager: String, package: String) {
    if package.is_empty() {
        println!("Usage: install <package>");
        return;
    }

    println!("Installing package: {}", package);
    // install package using package manager
    let mut cmd: std::process::Command = syntax::gen_install_syntax(manager.clone());
    cmd.arg(package);
    // cmd.arg(package::find_package(manager, package));
    cmd.output().expect("Failed to execute command");
    0;
}

fn uninstall(manager: String, package: String) {
    if package.is_empty() {
        println!("Usage: uninstall <package>");
        return;
    }

    println!("Uninstalling package: {}", package);
    // uninstall package using package manager
    let mut cmd: std::process::Command = syntax::gen_uninstall_syntax(manager.clone());
    cmd.arg(package);
    // cmd.arg(package::find_package(manager, package));
    cmd.output().expect("Failed to execute command");
    0;
}

fn upgrade(manager: String) {
    // update
    println!("Updating system");
    let mut update_cmd: std::process::Command = syntax::gen_update_syntax(manager.clone());
    update_cmd.output().expect("Failed to execute command");
    // upgrade
    println!("Upgrading system");
    let mut upgrade_cmd: std::process::Command = syntax::gen_upgrade_syntax(manager.clone());
    upgrade_cmd.output().expect("Failed to execute command");
    println!("System upgraded");
    0;
}
