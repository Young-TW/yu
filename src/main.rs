mod env;
mod syntax;
mod package;

use std::io::{BufReader, BufRead};
use std::process::{Stdio};

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
    let mut update_cmd = syntax::gen_update_syntax(manager.clone())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute update command");

    let update_stdout = update_cmd.stdout.take().expect("Failed to capture update stdout");
    let update_stderr = update_cmd.stderr.take().expect("Failed to capture update stderr");

    let stdout_reader = BufReader::new(update_stdout);
    let stderr_reader = BufReader::new(update_stderr);

    for line in stdout_reader.lines() {
        println!("{}", line.unwrap());
    }

    for line in stderr_reader.lines() {
        eprintln!("{}", line.unwrap());
    }

    update_cmd.wait().expect("Update command wasn't running");

    // upgrade
    println!("yu: Upgrading system");
    let mut upgrade_cmd = syntax::gen_upgrade_syntax(manager.clone())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute upgrade command");

    let upgrade_stdout = upgrade_cmd.stdout.take().expect("Failed to capture upgrade stdout");
    let upgrade_stderr = upgrade_cmd.stderr.take().expect("Failed to capture upgrade stderr");

    let stdout_reader = BufReader::new(upgrade_stdout);
    let stderr_reader = BufReader::new(upgrade_stderr);

    for line in stdout_reader.lines() {
        println!("{}", line.unwrap());
    }

    for line in stderr_reader.lines() {
        eprintln!("{}", line.unwrap());
    }

    upgrade_cmd.wait().expect("Upgrade command wasn't running");

    println!("yu: System upgraded");
}
