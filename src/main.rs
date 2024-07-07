mod env;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [install|uninstall|upgrade]", args[0]);
        return;
    }

    let package_manager = env::detect_package_manager();

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
    // // install package using package manager
    // std::process::Command::new(manager)
    //     .arg("install")
    //     .arg(package)
    //     .status()
    //     .expect("Failed to execute command");
    // 0;
}

fn uninstall(manager: String, package: String) {
    if package.is_empty() {
        println!("Usage: uninstall <package>");
        return;
    }

    println!("Uninstalling package: {}", package);
    0;
}

fn upgrade(manager: String) {
    // update

    // upgrade

    0;
}
