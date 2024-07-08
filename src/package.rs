use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use serde_json::Value;
use std::env;
use std::path::Path;

// pub fn write_into_dotfile() {
//     let home_dir = env::var("HOME").expect("Failed to get HOME environment variable");
//     let config_dir = Path::new(&home_dir).join(".config/yu");
//     let config_file = Path::new(&config_dir).join("packages.json");
//     if Path::new(&config_file).exists() {
//         return;
//     }

//     std::fs::create_dir_all(&config_dir).expect("Failed to create directory $HOME/.config/yu");
//     // copy the content of packages.json to ~/.config/yu/packages.json
//     std::fs::copy("package.json", &config_file).expect("Failed to copy file");
// }

pub fn read_packages_from_file() -> HashMap<String, HashMap<String, String>> {
    let config_file = Path::new(&env::var("HOME").expect("Failed to get HOME environment variable")).join(".config/yu/packages.json");
    let file = File::open(&config_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let json_value: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    let mut packages: HashMap<String, HashMap<String, String>> = HashMap::new();

    if let Value::Object(package_map) = json_value {
        for (package_name, managers) in package_map {
            if let Value::Object(manager_map) = managers {
                let mut manager_hashmap = HashMap::new();
                for (manager_name, package_value) in manager_map {
                    if let Value::String(package_str) = package_value {
                        manager_hashmap.insert(manager_name, package_str);
                    }
                }
                packages.insert(package_name, manager_hashmap);
            }
        }
    }

    packages
}

pub fn find_package(manager: String, name: String) -> String {
    // use the same name for the same package in different package managers
    let package_names = read_packages_from_file();

    match package_names.get(&name) {
        Some(package_map) => match package_map.get(manager.as_str()) {
            Some(manager_package) => {
                println!("Installing {} using {}: {}", &name, manager, manager_package);
                return manager_package.to_string();
            },
            None => println!("Package {} is not available for package manager {}", &name, manager),
        },
        None => println!("Package {} not found", &name),
    }

    "".to_string()
}