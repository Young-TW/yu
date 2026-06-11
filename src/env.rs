use std::path::Path;

/// Candidate package managers, in priority order, paired with the absolute
/// path that signals their presence on the system.
const MANAGERS: &[(&str, &str)] = &[
    ("apt", "/usr/bin/apt"),
    ("dnf", "/usr/bin/dnf"),
    ("yum", "/usr/bin/yum"),
    ("brew", "/opt/homebrew/bin/brew"),
    ("brew", "/usr/local/bin/brew"),
    ("brew", "/home/linuxbrew/.linuxbrew/bin/brew"),
    ("zypper", "/usr/bin/zypper"),
    ("pacman", "/usr/bin/pacman"),
    ("apk", "/usr/bin/apk"),
    ("portage", "/usr/bin/emerge"),
];

pub fn detect_package_manager() -> String {
    detect_with(|path| Path::new(path).exists())
}

/// Detection core, parameterised over an existence check so it can be unit
/// tested without touching the real filesystem.
fn detect_with(exists: impl Fn(&str) -> bool) -> String {
    for (manager, path) in MANAGERS {
        if exists(path) {
            return manager.to_string();
        }
    }
    "unknown".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_apt() {
        let result = detect_with(|p| p == "/usr/bin/apt");
        assert_eq!(result, "apt");
    }

    #[test]
    fn detects_pacman() {
        let result = detect_with(|p| p == "/usr/bin/pacman");
        assert_eq!(result, "pacman");
    }

    #[test]
    fn detects_portage_from_emerge() {
        let result = detect_with(|p| p == "/usr/bin/emerge");
        assert_eq!(result, "portage");
    }

    #[test]
    fn detects_brew_from_any_known_path() {
        for path in [
            "/opt/homebrew/bin/brew",
            "/usr/local/bin/brew",
            "/home/linuxbrew/.linuxbrew/bin/brew",
        ] {
            let result = detect_with(|p| p == path);
            assert_eq!(result, "brew", "expected brew for {path}");
        }
    }

    #[test]
    fn returns_unknown_when_nothing_present() {
        let result = detect_with(|_| false);
        assert_eq!(result, "unknown");
    }

    #[test]
    fn apt_takes_priority_over_dnf() {
        // When several managers are present, the first in priority order wins.
        let result = detect_with(|p| p == "/usr/bin/apt" || p == "/usr/bin/dnf");
        assert_eq!(result, "apt");
    }

    #[test]
    fn dnf_takes_priority_over_yum() {
        let result = detect_with(|p| p == "/usr/bin/dnf" || p == "/usr/bin/yum");
        assert_eq!(result, "dnf");
    }
}
