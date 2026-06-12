use std::process::Command;

/// Absolute path of the sudoers fragment installed by `yu setup-sudo`.
const SUDOERS_PATH: &str = "/etc/sudoers.d/yu";

/// Returns whether running `program` requires elevated privileges.
///
/// Note: this matches the *actual* executable name produced by the
/// `gen_*_syntax` helpers, not the package-manager identifier. Gentoo's
/// `portage` is driven through `emerge`/`qlist`, so `emerge` is the name we
/// match here. `brew` intentionally runs without `sudo`.
pub fn needs_sudo(program: &str) -> bool {
    matches!(
        program,
        "apt" | "dnf" | "yum" | "pacman" | "zypper" | "apk" | "emerge"
    )
}

/// Pure policy decision: should the command for `program` be prefixed with
/// `sudo`? Only when it needs elevation *and* a `sudo` binary is available
/// (e.g. inside a minimal container running as root, there is none, so we run
/// the command directly).
fn should_use_sudo(program: &str, sudo_available: bool) -> bool {
    needs_sudo(program) && sudo_available
}

/// Wraps `cmd` with `sudo` when elevation is required, otherwise returns it
/// unchanged. `sudo` is left to enforce the system's own authentication
/// policy (prompting for a password, honouring cached credentials, etc.) —
/// `yu` never probes for or alters that policy here.
pub fn get_sudo(cmd: Command) -> Command {
    use which::which;

    let program = cmd.get_program().to_string_lossy().to_string();

    if !should_use_sudo(&program, which("sudo").is_ok()) {
        return cmd;
    }

    // Resolve to an absolute path so the call does not depend on sudo's
    // secure_path, falling back to the bare name if resolution fails.
    let program_path = which(&program)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or(program);

    let mut sudo_cmd = Command::new("sudo");
    sudo_cmd.arg(&program_path);
    sudo_cmd.args(cmd.get_args());
    sudo_cmd
}

/// Maps a detected package manager to the executable that actually runs under
/// `sudo`. Returns `None` for managers that never need elevation (`brew`) or
/// are unknown.
pub fn privileged_executable(manager: &str) -> Option<&'static str> {
    match manager {
        "apt" => Some("apt"),
        "dnf" => Some("dnf"),
        "yum" => Some("yum"),
        "pacman" => Some("pacman"),
        "zypper" => Some("zypper"),
        "apk" => Some("apk"),
        "portage" => Some("emerge"),
        _ => None,
    }
}

/// Interactive `yu setup-sudo` entry point. Explicitly opt-in: it grants a
/// passwordless-`sudo` rule for the detected package manager, which is a
/// security-sensitive action, so it always warns and asks for confirmation
/// and is never triggered automatically by other commands.
///
/// Returns `false` only when a requested setup actually failed, so the caller
/// can exit non-zero. A clean user abort, or a manager that needs no sudo,
/// counts as success (nothing went wrong).
pub fn setup_sudo(manager: &str) -> bool {
    use which::which;

    let exe = match privileged_executable(manager) {
        Some(exe) => exe,
        None => {
            eprintln!("yu: {manager} does not run under sudo; nothing to set up");
            return true;
        }
    };

    let exe_path = match which(exe) {
        Ok(path) => path.to_string_lossy().to_string(),
        Err(_) => {
            eprintln!("yu: cannot find `{exe}` on PATH");
            return false;
        }
    };

    eprintln!("yu: WARNING — this installs a passwordless sudo rule:");
    eprintln!("yu:     {}", sudoers_rule(&exe_path));
    eprintln!("yu: Because `{exe}` can run arbitrary code as root, this is");
    eprintln!("yu: effectively passwordless root for your account. It will be");
    eprintln!("yu: written to {SUDOERS_PATH}.");
    eprint!("yu: Continue? [y/N] ");
    use std::io::Write;
    let _ = std::io::stderr().flush();

    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_err() {
        eprintln!("yu: could not read confirmation; aborting");
        return false;
    }
    let confirmed = matches!(input.trim().to_ascii_lowercase().as_str(), "y" | "yes");
    if !confirmed {
        println!("yu: aborted; no changes made");
        return true;
    }

    match setup_sudoers_rule(&exe_path) {
        Ok(()) => {
            println!("yu: installed sudoers rule at {SUDOERS_PATH}");
            true
        }
        Err(e) => {
            eprintln!("yu: failed to set up sudoers rule: {e}");
            false
        }
    }
}

/// Builds the sudoers line granting the current user passwordless access to
/// `exe_path`.
fn sudoers_rule(exe_path: &str) -> String {
    let user = whoami::username();
    format!("{user} ALL=(ALL) NOPASSWD: {exe_path}")
}

/// Installs the sudoers fragment safely:
/// - the rule content is piped to `sudo tee` over stdin, never interpolated
///   into a shell string (no injection), and never staged through a
///   predictable `/tmp` path (no symlink/TOCTOU race);
/// - `visudo -cf` validates the installed file, and an invalid file is
///   removed again so a broken fragment can never linger in `/etc/sudoers.d`.
pub fn setup_sudoers_rule(exe_path: &str) -> std::io::Result<()> {
    use std::io::Write;
    use std::process::Stdio;

    let rule = format!("{}\n", sudoers_rule(exe_path));

    let mut tee = Command::new("sudo")
        .arg("tee")
        .arg(SUDOERS_PATH)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()?;
    let mut stdin = tee.stdin.take().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::BrokenPipe, "could not open sudo stdin")
    })?;
    stdin.write_all(rule.as_bytes())?;
    drop(stdin); // close the pipe so `tee` sees EOF and exits
    if !tee.wait()?.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "failed to write sudoers file",
        ));
    }

    let _ = Command::new("sudo")
        .args(["chmod", "440", SUDOERS_PATH])
        .status();

    let valid = Command::new("sudo")
        .args(["visudo", "-cf", SUDOERS_PATH])
        .stdout(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if !valid {
        // Never leave an unvalidated fragment behind.
        let _ = Command::new("sudo")
            .args(["rm", "-f", SUDOERS_PATH])
            .status();
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "generated sudoers rule failed validation",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_managers_need_sudo() {
        for program in ["apt", "dnf", "yum", "pacman", "zypper", "apk", "emerge"] {
            assert!(needs_sudo(program), "{program} should require sudo");
        }
    }

    #[test]
    fn brew_does_not_need_sudo() {
        assert!(!needs_sudo("brew"));
    }

    #[test]
    fn portage_helpers_match_actual_executables() {
        // `gen_*_syntax` emits `emerge`/`qlist`, never a literal `portage`
        // binary, so the identifier itself must not be treated as privileged.
        assert!(!needs_sudo("portage"));
        // qlist (package listing) is read-only and needs no elevation.
        assert!(!needs_sudo("qlist"));
    }

    #[test]
    fn unknown_program_does_not_need_sudo() {
        assert!(!needs_sudo("unknown"));
        assert!(!needs_sudo(""));
    }

    #[test]
    fn should_use_sudo_requires_both_elevation_and_a_sudo_binary() {
        assert!(should_use_sudo("apt", true));
        // No sudo binary present (e.g. running as root in a container).
        assert!(!should_use_sudo("apt", false));
        // brew never needs elevation, even when sudo exists.
        assert!(!should_use_sudo("brew", true));
    }

    #[test]
    fn privileged_executable_maps_managers_to_real_binaries() {
        assert_eq!(privileged_executable("apt"), Some("apt"));
        assert_eq!(privileged_executable("pacman"), Some("pacman"));
        // Gentoo is driven through emerge, not a `portage` binary.
        assert_eq!(privileged_executable("portage"), Some("emerge"));
        // brew and unknown managers do not use sudo.
        assert_eq!(privileged_executable("brew"), None);
        assert_eq!(privileged_executable("unknown"), None);
    }

    #[test]
    fn sudoers_rule_is_well_formed() {
        let rule = sudoers_rule("/usr/bin/apt");
        assert!(rule.contains(" ALL=(ALL) NOPASSWD: /usr/bin/apt"));
        assert!(!rule.contains('\n'));
    }
}
