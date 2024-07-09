use std::process::{Stdio};

use fluent::FluentBundle;
use unic_langid::langid;

mod env;
mod syntax;
mod language;

fn main() {
    let langid_zh_tw = langid!("zh-TW");
    let resource = language::load_resource(&langid_zh_tw, "text.ftl");
    let mut bundle = FluentBundle::new(vec![langid_zh_tw]);
    bundle.add_resource(resource).expect("Failed to add resource");

    let args: Vec<String> = std::env::args().collect();
    let package_manager = env::detect_package_manager();
    if args.len() < 2 {
        upgrade(package_manager.clone());
        return;
    }

    // check second parameter
    match args[1].as_str() {
        "install" => install(package_manager.clone(), args[2].clone()),
        "uninstall" => uninstall(package_manager.clone(), args[2].clone()),
        "upgrade" => upgrade(package_manager.clone()),
        _ => {
            language::print_message(bundle, "unknown-command");
            print!(": {}", args[1]);
        }
    }
    0;
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
