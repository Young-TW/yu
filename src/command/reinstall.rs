use crate::root::get_sudo;

pub fn reinstall(manager: String, package: String, silent: bool, verbose: bool) {
    if package.is_empty() {
        eprintln!("Usage: reinstall <package>");
        return;
    }

    if !silent {
        println!("yu: Reinstalling package: {}", package);
    }

    let mut reinstall_cmd = gen_reinstall_syntax(manager.clone())
        .arg(package)
        .stdout(if verbose { std::process::Stdio::inherit() } else { std::process::Stdio::null() })
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("yu: Failed to execute command");

    reinstall_cmd.wait().expect("Command wasn't running");
}

pub fn gen_reinstall_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = get_sudo(manager.clone());
    match manager.as_str() {
        "apt" | "dnf" | "yum" | "pacman" | "zypper" => {
            command.arg("reinstall");
            command.arg("-y");
        }
        "apk" => {
            command.arg("add");
            command.arg("-f");
        }
        "portage" => {
            command.arg("emerge");
            command.arg("--oneshot");
        }
        "brew" => {
            command.arg("reinstall");
        }
        _ => {
            println!("Unknown package manager: {}", manager);
        }
    }
    command
}
