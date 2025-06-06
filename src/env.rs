use std::path::Path;

pub fn detect_package_manager() -> String {
    if Path::new("/usr/bin/apt").exists() {
        "apt".to_string()
    } else if Path::new("/usr/bin/dnf").exists() {
        "dnf".to_string()
    } else if Path::new("/usr/bin/yum").exists() {
        "yum".to_string()
    } else if Path::new("/opt/homebrew/bin/brew").exists()
        || Path::new("/usr/local/bin/brew").exists()
        || Path::new("/home/linuxbrew/.linuxbrew/bin/brew").exists()
    {
        "brew".to_string()
    } else if Path::new("/usr/bin/zypper").exists() {
        "zypper".to_string()
    } else if Path::new("/usr/bin/pacman").exists() {
        "pacman".to_string()
    } else if Path::new("/usr/bin/apk").exists() {
        "apk".to_string()
    } else if Path::new("/usr/bin/emerge").exists() {
        "portage".to_string()
    } else {
        "unknown".to_string()
    }
}
