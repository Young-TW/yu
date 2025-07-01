use std::process::Command;

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
