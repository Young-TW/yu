use crate::syntax;

pub fn reinstall(manager: String, package: String, silent: bool, verbose: bool) {
    if package.is_empty() {
        eprintln!("Usage: reinstall <package>");
        return;
    }

    if !silent {
        println!("yu: Reinstalling package: {}", package);
    }

    let mut reinstall_cmd = syntax::gen_reinstall_syntax(manager.clone())
        .arg(package)
        .stdout(if verbose { std::process::Stdio::inherit() } else { std::process::Stdio::null() })
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("yu: Failed to execute command");

    reinstall_cmd.wait().expect("Command wasn't running");
}