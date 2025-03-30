use std::fs::{self, File};
use std::path::Path;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

use which::which;

pub fn get_sudo(manager: String) -> std::process::Command {
    let needs_sudo = matches!(
        manager.as_str(),
        "apt" | "dnf" | "yum" | "pacman" | "zypper" | "apk" | "portage"
    );

    if needs_sudo {
        let sudoers_path = Path::new("/etc/sudoers.d/uni-pkg");
        if !sudoers_path.exists() {
            println!("yu: setting up a sudoers rule for {}", manager);
            println!("yu: this will allow you to run {} without a password", manager);

            if let Ok(path) = which(&manager) {
                let full_path = path.to_string_lossy().to_string();
                if let Err(e) = setup_sudoers_rule(full_path) {
                    eprintln!("yu: failed to set up sudoers rule: {}", e);
                    eprintln!("please ensure you have permission to write to /etc/sudoers.d/uni-pkg");
                }
            } else {
                eprintln!("yu: could not find the path of {}. Skipping sudoers setup", manager);
            }
        }

        let mut cmd = Command::new("sudo");
        cmd.arg(manager);
        cmd
    } else {
        Command::new(manager)
    }
}

pub fn setup_sudoers_rule(command: String) -> std::io::Result<()> {
    let user = whoami::username();
    let rule = format!(
        "{user} ALL=(ALL) NOPASSWD: {command}\n",
        user = user,
        command = command
    );

    let tmp_path = "/tmp/uni-pkg-sudoers.tmp";
    let final_path = "/etc/sudoers.d/uni-pkg";

    let mut file = File::create(tmp_path)?;
    file.write_all(rule.as_bytes())?;
    fs::set_permissions(tmp_path, fs::Permissions::from_mode(0o440))?;

    let check_status = Command::new("visudo")
        .args(["-cf", tmp_path])
        .status()
        .expect("failed to execute visudo");

    if !check_status.success() {
        eprintln!("yu: visudo check failed. Not writing to sudoers");
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "sudoers syntax error",
        ));
    }

    let copy_status = Command::new("sudo")
        .args(["cp", tmp_path, final_path])
        .status()
        .expect("failed to execute 'sudo cp'");

    if !copy_status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "sudo cp failed",
        ));
    }

    println!("yu: sudoers setup successful: {final_path}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sudo() {
        let manager = "apt".to_string();
        let cmd = get_sudo(manager.clone());
        assert_eq!(cmd.get_program(), "sudo");

        let manager = "dnf".to_string();
        let cmd = get_sudo(manager.clone());
        assert_eq!(cmd.get_program(), "sudo");

        let manager = "yum".to_string();
        let cmd = get_sudo(manager.clone());
        assert_eq!(cmd.get_program(), "sudo");

        let manager = "pacman".to_string();
        let cmd = get_sudo(manager.clone());
        assert_eq!(cmd.get_program(), "sudo");

        let manager = "zypper".to_string();
        let cmd = get_sudo(manager.clone());
        assert_eq!(cmd.get_program(), "sudo");

        let manager = "apk".to_string();
        let cmd = get_sudo(manager.clone());
        assert_eq!(cmd.get_program(), "sudo");

        let manager = "portage".to_string();
        let cmd = get_sudo(manager.clone());
        assert_eq!(cmd.get_program(), "sudo");

        let manager = "brew".to_string();
        let cmd = get_sudo(manager.clone());
        assert_eq!(cmd.get_program(), "brew");
    }
}
