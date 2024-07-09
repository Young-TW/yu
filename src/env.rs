pub fn detect_package_manager() -> String {
    let package_manager: String;
    if (std::path::Path::new("/usr/bin/apt")).exists() {
        package_manager = "apt".to_string();
    } else if (std::path::Path::new("/usr/bin/dnf")).exists() {
        package_manager = "dnf".to_string();
    } else if (std::path::Path::new("/usr/bin/yum")).exists() {
        package_manager = "yum".to_string();
    } else if (std::path::Path::new("/opt/homebrew/bin/brew")).exists() {
        package_manager = "brew".to_string();
    } else if (std::path::Path::new("/usr/bin/zypper")).exists() {
        package_manager = "zypper".to_string();
    } else if (std::path::Path::new("/usr/bin/pacman")).exists() {
        package_manager = "pacman".to_string();
    } else {
        package_manager = "unknown".to_string();
    }

    package_manager
}
