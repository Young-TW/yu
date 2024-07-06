fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [install|uninstall|update|upgrade]", args[0]);
        return;
    }

    // check second parameter
    match args[1].as_str() {
        "install" => install(args[2].clone()),
        "uninstall" => uninstall(args[2].clone()),
        "update" => update(),
        "upgrade" => upgrade(),
        _ => println!("Unknown command: {}", args[1]),
    }
    0;
}

fn install(package: String) {
    if package.is_empty() {
        println!("Usage: install <package>");
        return;
    }

    println!("Installing package: {}", package);
    0;
}

fn uninstall(package: String) {
    if package.is_empty() {
        println!("Usage: uninstall <package>");
        return;
    }

    println!("Uninstalling package: {}", package);
    0;
}

fn update() {
    0;
}

fn upgrade() {
    0;
}
