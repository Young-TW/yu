use std::process::{Stdio};

mod env;
mod syntax;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let package_manager = env::detect_package_manager();
    if args.len() < 2 {
        upgrade(package_manager.clone());
        return;
    }

    match args[1].as_str() {
        "install" => install(package_manager.clone(), args[2].clone()),
        "uninstall" => uninstall(package_manager.clone(), args[2].clone()),
        "upgrade" => upgrade(package_manager.clone()),
        _ => {
            println!("Unknown command: {}", args[1]);
        }
    }
}

fn install(manager: String, package: String) {
    if package.is_empty() {
        println!("Usage: install <package>");
        return;
    }

    println!("yu: Installing package: {}", package);
    let mut cmd = syntax::gen_install_syntax(manager.clone())
        .arg(package)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("yu: Failed to execute command");

    cmd.wait().expect("Command wasn't running");
}

fn uninstall(manager: String, package: String) {
    if package.is_empty() {
        println!("Usage: uninstall <package>");
        return;
    }

    println!("yu: Uninstalling package: {}", package);
    let mut cmd = syntax::gen_uninstall_syntax(manager.clone())
        .arg(package)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("yu: Failed to execute command");

    cmd.wait().expect("Command wasn't running");
}

fn upgrade(manager: String) {
    println!("yu: Updating system");
    let mut update_cmd = syntax::gen_update_syntax(manager.clone())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute update command");

    update_cmd.wait().expect("Update command wasn't running");

    println!("yu: Upgrading system");
    let mut upgrade_cmd = syntax::gen_upgrade_syntax(manager.clone())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute upgrade command");

    upgrade_cmd.wait().expect("Upgrade command wasn't running");

    println!("yu: System upgraded");
}
