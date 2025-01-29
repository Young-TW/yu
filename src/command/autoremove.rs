use crate::root::get_sudo;

pub fn autoremove(manager: String, slient: bool, verbose: bool) {
    if !slient {
        println!("yu: Auto removing unused packages")
    }

    let mut autoremove_cmd = gen_autoremove_syntax(manager.clone())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("Failed to execute autoremove command");

    autoremove_cmd
        .wait()
        .expect("Autoremove command wasn't running");
}

pub fn gen_autoremove_syntax(manager: String) -> std::process::Command {
    let mut command: std::process::Command = get_sudo(manager.clone());
    match manager.as_str() {
        "apt" => {
            command.arg("autoremove");
        }
        "dnf" => {
            command.arg("autoremove");
        }
        "yum" => {
            command.arg("autoremove");
        }
        "pacman" => {
            command.arg("-Rns");
        }
        "zypper" => {
            command.arg("remove");
        }
        "apk" => {
            command.arg("autoremove");
        }
        "portage" => {
            command.arg("--depclean");
        }
        "brew" => {
            command.arg("cleanup");
        }
        _ => {
            println!("Unknown package manager: {}", manager);
        }
    }
    command
}
