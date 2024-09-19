use crate::root::get_sudo;

pub fn uninstall(manager: String, package: String, silent: bool, verbose: bool) {
    if package.is_empty() {
        eprintln!("Usage: uninstall <package>");
        return;
    }

    if !silent {
        println!("yu: Uninstalling package: {}", package);
    }

    let mut uninstall_cmd = gen_uninstall_syntax(manager.clone())
        .arg(package)
        .stdout(if verbose { std::process::Stdio::inherit() } else { std::process::Stdio::null() })
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("yu: Failed to execute command");

    uninstall_cmd.wait().expect("Command wasn't running");
}

pub fn gen_uninstall_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = get_sudo(manager.clone());
    // add arguments
    match manager.as_str() {
        "apt" | "dnf" | "yum" | "pacman" | "zypper" => {
            command.arg("remove");
            command.arg("-y");
        }
        "apk" => {
            command.arg("del");
        }
        "portage" => {
            command.arg("emerge");
            command.arg("--unmerge");
        }
        "brew" => {
            command.arg("uninstall");
        }
        _ => {
            println!("Unknown package manager: {}", manager);
        }
    }

    command
}
