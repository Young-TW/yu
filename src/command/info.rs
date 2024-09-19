use crate::root::get_sudo;

pub fn info(manager: String, package: String, silent: bool) {
    if package.is_empty() {
        eprintln!("Usage: info <package>");
        return;
    }

    if !silent {
        println!("yu: Getting package information");
    }

    let mut info_cmd = gen_info_syntax(manager.clone())
        .arg(package)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("Failed to execute info command");

    info_cmd.wait().expect("Info command wasn't running");
}

pub fn gen_info_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = get_sudo(manager.clone());
    match manager.as_str() {
        "apt" => {
            command.arg("show");
        }
        "dnf" => {
            command.arg("info");
        }
        "yum" => {
            command.arg("info");
        }
        "pacman" => {
            command.arg("-Qi");
        }
        "zypper" => {
            command.arg("info");
        }
        "apk" => {
            command.arg("info");
        }
        "portage" => {
            command.arg("info");
        }
        "brew" => {
            command.arg("info");
        }
        _ => {
            println!("Unknown package manager: {}", manager);
        }
    }
    command
}
