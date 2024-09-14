use crate::syntax;

pub fn install(manager: String, package: String, silent: bool, verbose: bool) {
    if package.is_empty() {
        eprintln!("Usage: install <package>");
        return;
    }

    if !silent {
        println!("yu: Installing package: {}", package);
    }

    let mut install_cmd = syntax::gen_install_syntax(manager.clone())
        .arg(package)
        .stdout(if verbose { std::process::Stdio::inherit() } else { std::process::Stdio::null() })
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("yu: Failed to execute command");

    install_cmd.wait().expect("Command wasn't running");
}
