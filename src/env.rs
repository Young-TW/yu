pub fn detect_package_manager() -> String {
    if std::path::Path::new("/usr/bin/apt").exists() {
        "apt".to_string()
    } else if std::path::Path::new("/usr/bin/dnf").exists() {
        "dnf".to_string()
    } else if std::path::Path::new("/usr/bin/yum").exists() {
        "yum".to_string()
    } else if std::path::Path::new("/opt/homebrew/bin/brew").exists() {
        "brew".to_string()
    } else if std::path::Path::new("/usr/bin/zypper").exists() {
        "zypper".to_string()
    } else if std::path::Path::new("/usr/bin/pacman").exists() {
        "pacman".to_string()
    } else if std::path::Path::new("/usr/bin/apk").exists() {
        "apk".to_string()
    } else if std::path::Path::new("/usr/bin/portage").exists() {
        "portage".to_string()
    } else {
        "unknown".to_string()
    }
}
