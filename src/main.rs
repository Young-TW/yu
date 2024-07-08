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
        _ => println!("yu: Unknown command: {}", args[1]),
    }
    0;
}

fn install(manager: String, package: String) {
    if package.is_empty() {
        println!("Usage: install <package>");
        return;
    }

    println!("yu: Installing package: {}", package);
    // install package using package manager
    let mut cmd: std::process::Command = syntax::gen_install_syntax(manager.clone());
    cmd.arg(package);
    // cmd.arg(package::find_package(manager, package));
    let out = cmd.output().expect("yu: Failed to execute command");
    println!("{}", String::from_utf8_lossy(&out.stdout));
}

fn uninstall(manager: String, package: String) {
    if package.is_empty() {
        println!("Usage: uninstall <package>");
        return;
    }

    println!("yu: Uninstalling package: {}", package);
    // uninstall package using package manager
    let mut cmd: std::process::Command = syntax::gen_uninstall_syntax(manager.clone());
    cmd.arg(package);
    // cmd.arg(package::find_package(manager, package));
    let out = cmd.output().expect("yu: Failed to execute command");
    println!("{}", String::from_utf8_lossy(&out.stdout));
}

fn upgrade(manager: String) {
    // update
    println!("yu: Updating system");
    let mut update_cmd: std::process::Command = syntax::gen_update_syntax(manager.clone());
    let out = update_cmd.output().expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&out.stdout));
    // upgrade
    println!("yu: Upgrading system");
    let mut upgrade_cmd: std::process::Command = syntax::gen_upgrade_syntax(manager.clone());
    let out = upgrade_cmd.output().expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&out.stdout));
    println!("yu: System upgraded");
}
