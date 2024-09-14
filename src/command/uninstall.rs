use crate::syntax;

pub fn uninstall(manager: String, package: String, silent: bool, verbose: bool) {
    if package.is_empty() {
        eprintln!("Usage: uninstall <package>");
        return;
    }

    if !silent {
        println!("yu: Uninstalling package: {}", package);
    }

    let mut uninstall_cmd = syntax::gen_uninstall_syntax(manager.clone())
        .arg(package)
        .stdout(if verbose { std::process::Stdio::inherit() } else { std::process::Stdio::null() })
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("yu: Failed to execute command");

    uninstall_cmd.wait().expect("Command wasn't running");
}
