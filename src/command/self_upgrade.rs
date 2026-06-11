use std::process::Command;

// Not yet wired into the CLI dispatcher in `main.rs` (language-specific
// managers such as npm/pip/cargo are not detected by `env::detect_package_manager`
// yet), but kept compiled and tested so the syntax stays correct.
#[allow(dead_code)]
pub fn self_upgrade(manager: &str) -> Result<Command, String> {
    let mut cmd = Command::new(manager);

    match manager {
        "npm" => {
            cmd.args(&["install", "-g", "npm"]);
        }
        "pnpm" => {
            cmd.arg("self-update");
        }
        "pip" => {
            cmd.args(&["install", "--upgrade", "pip"]);
        }
        "pipx" => {
            cmd.args(&["upgrade", "pipx"]);
        }
        "uv" => {
            cmd.arg("self-upgrade");
        }
        "cargo" => {
            // 改用 rustup 更新 cargo 本身
            cmd = Command::new("rustup");
            cmd.args(&["self", "update"]);
        }
        _ => {
            return Err(format!("不支援的套件管理器：{}", manager));
        }
    }

    Ok(cmd)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    fn cmd_to_string(cmd: &Command) -> Vec<String> {
        let program = cmd.get_program().to_string_lossy().to_string();
        if program.is_empty() {
            return vec![];
        }
        let mut output = vec![program];
        output.extend(cmd.get_args().map(|s| s.to_string_lossy().to_string()));
        output
    }

    #[test]
    fn test_self_upgrade_npm() {
        let cmd = self_upgrade("npm").unwrap();
        assert_eq!(cmd_to_string(&cmd), vec!["npm", "install", "-g", "npm"]);
    }

    #[test]
    fn test_self_upgrade_pnpm() {
        let cmd = self_upgrade("pnpm").unwrap();
        assert_eq!(cmd_to_string(&cmd), vec!["pnpm", "self-update"]);
    }

    #[test]
    fn test_self_upgrade_pip() {
        let cmd = self_upgrade("pip").unwrap();
        assert_eq!(
            cmd_to_string(&cmd),
            vec!["pip", "install", "--upgrade", "pip"]
        );
    }

    #[test]
    fn test_self_upgrade_pipx() {
        let cmd = self_upgrade("pipx").unwrap();
        assert_eq!(cmd_to_string(&cmd), vec!["pipx", "upgrade", "pipx"]);
    }

    #[test]
    fn test_self_upgrade_uv() {
        let cmd = self_upgrade("uv").unwrap();
        assert_eq!(cmd_to_string(&cmd), vec!["uv", "self-upgrade"]);
    }

    #[test]
    fn test_self_upgrade_cargo_uses_rustup() {
        let cmd = self_upgrade("cargo").unwrap();
        assert_eq!(cmd_to_string(&cmd), vec!["rustup", "self", "update"]);
    }

    #[test]
    fn test_self_upgrade_unknown_is_err() {
        assert!(self_upgrade("unknown").is_err());
    }
}
