use std::collections::HashMap;

pub fn make_hashmap() -> HashMap<String, HashMap<String, String>> {
    let mut packages: HashMap<String, HashMap<String, String>> = HashMap::new();

    let mut cmake_map = HashMap::new();
    cmake_map.insert("apt".to_string(), "cmake".to_string());
    cmake_map.insert("dnf".to_string(), "cmake".to_string());
    cmake_map.insert("yum".to_string(), "cmake".to_string());
    cmake_map.insert("paru".to_string(), "cmake".to_string());
    cmake_map.insert("pacman".to_string(), "cmake".to_string());
    cmake_map.insert("brew".to_string(), "cmake".to_string());
    packages.insert("cmake".to_string(), cmake_map);

    let mut gcc_map = HashMap::new();
    gcc_map.insert("apt".to_string(), "gcc".to_string());
    gcc_map.insert("dnf".to_string(), "gcc".to_string());
    gcc_map.insert("yum".to_string(), "gcc".to_string());
    gcc_map.insert("paru".to_string(), "gcc".to_string());
    gcc_map.insert("pacman".to_string(), "gcc".to_string());
    gcc_map.insert("brew".to_string(), "gcc".to_string());
    packages.insert("gcc".to_string(), gcc_map);

    let mut gpp_map = HashMap::new();
    gpp_map.insert("apt".to_string(), "g++".to_string());
    gpp_map.insert("dnf".to_string(), "gcc-c++".to_string());
    gpp_map.insert("yum".to_string(), "gcc-c++".to_string());
    gpp_map.insert("paru".to_string(), "gcc".to_string());
    gpp_map.insert("pacman".to_string(), "gcc".to_string());
    gpp_map.insert("brew".to_string(), "gcc".to_string());
    packages.insert("g++".to_string(), gpp_map);

    let mut python_map = HashMap::new();
    python_map.insert("apt".to_string(), "python3".to_string());
    python_map.insert("dnf".to_string(), "python3".to_string());
    python_map.insert("yum".to_string(), "python3".to_string());
    python_map.insert("paru".to_string(), "python".to_string());
    python_map.insert("pacman".to_string(), "python".to_string());
    python_map.insert("brew".to_string(), "python".to_string());
    packages.insert("python".to_string(), python_map);

    packages
}

pub fn find_package(manager: String, name: String) -> String {
    // use the same name for the same package in different package managers
    let package_names = make_hashmap();

    match package_names.get(&name) {
        Some(package_map) => match package_map.get(manager.as_str()) {
            Some(manager_package) => {
                println!("Installing {} using {}: {}", &name, manager, manager_package);
                // 這裡可以加入實際安裝套件的命令，例如調用 system() 函數
            },
            None => println!("Package {} is not available for package manager {}", &name, manager),
        },
        None => println!("Package {} not found", &name),
    }

    "".to_string()
}