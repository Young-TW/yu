use crate::root::get_sudo;

pub fn install(manager: String, package: String, silent: bool, verbose: bool) {
    if package.is_empty() {
        eprintln!("Usage: install <package>");
        return;
    }

    if !silent {
        println!("yu: Installing package: {}", package);
    }

    let mut install_cmd = gen_install_syntax(manager.clone())
        .arg(package)
        .stdout(if verbose {
            std::process::Stdio::inherit()
        } else {
            std::process::Stdio::null()
        })
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("yu: Failed to execute command");

    install_cmd.wait().expect("Command wasn't running");
}

pub fn gen_install_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = get_sudo(manager.clone());
    // add arguments
    match manager.as_str() {
        "apt" | "dnf" | "yum" | "pacman" | "zypper" => {
            command.arg("install");
            command.arg("-y");
        }
        "apk" => {
            command.arg("add");
        }
        "portage" => {
            command.arg("emerge");
        }
        "brew" => {
            command.arg("install");
        }
        _ => {
            println!("Unknown package manager: {}", manager);
        }
    }
    command
}
