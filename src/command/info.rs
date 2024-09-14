use crate::syntax;

pub fn info(manager: String, package: String, silent: bool) {
    if package.is_empty() {
        eprintln!("Usage: info <package>");
        return;
    }

    if !silent {
        println!("yu: Getting package information");
    }

    let mut info_cmd = syntax::gen_info_syntax(manager.clone())
        .arg(package)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("Failed to execute info command");

    info_cmd.wait().expect("Info command wasn't running");
}