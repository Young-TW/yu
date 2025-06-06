use std::process::Command;

pub fn get_sudo(cmd: std::process::Command) -> std::process::Command {
    use std::process::{Command, Stdio};
    use which::which;

    let program = cmd.get_program().to_string_lossy().to_string();
    let needs_sudo = matches!(
        program.as_str(),
        "apt" | "dnf" | "yum" | "pacman" | "zypper" | "apk" | "portage"
    );

    if !needs_sudo {
        return cmd;
    }

    if let Ok(path) = which(&program) {
        let abs_path = path.to_string_lossy().to_string();

        let version_arg = if program == "pacman" { "-V" } else { "--version" };
        let check_status = Command::new("sudo")
            .args(["-n", &abs_path, version_arg])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();

        let needs_setup = match check_status {
            Ok(status) => !status.success(),
            Err(_) => true,
        };

        if needs_setup {
            println!("yu: setting up a sudoers rule for {}", program);
            println!("yu: this will allow you to run {} without a password", program);

            if let Err(e) = setup_sudoers_rule(abs_path.clone()) {
                eprintln!("yu: failed to set up sudoers rule: {}", e);
                eprintln!("please ensure you have permission to write to /etc/sudoers.d/uni-pkg");
                std::process::exit(1);
            }
        }

        // 重新包 sudo 指令
        let mut sudo_cmd = Command::new("sudo");
        sudo_cmd.arg(&abs_path);
        sudo_cmd.args(cmd.get_args());
        return sudo_cmd;
    }

    cmd
}

pub fn setup_sudoers_rule(command: String) -> std::io::Result<()> {
    let user = whoami::username();
    let rule = format!("{user} ALL=(ALL) NOPASSWD: {command}\n");

    let script = format!(
        "echo '{rule}' > /tmp/uni-pkg-sudoers.tmp && \
         chmod 440 /tmp/uni-pkg-sudoers.tmp && \
         visudo -cf /tmp/uni-pkg-sudoers.tmp && \
         cp /tmp/uni-pkg-sudoers.tmp /etc/sudoers.d/uni-pkg && \
         echo 'yu: sudoers setup successful: /etc/sudoers.d/uni-pkg'"
    );

    let status = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg(&script)
        .status()
        .expect("failed to execute sudo shell script");

    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "sudoers setup failed",
        ));
    }

    Ok(())
}
