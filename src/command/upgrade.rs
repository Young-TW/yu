use crate::syntax;

pub fn upgrade(manager: String, silent: bool, verbose: bool) {
    if !silent {
        println!("yu: Upgrading system");
    }
    let mut upgrade_cmd = syntax::gen_upgrade_syntax(manager.clone())
        .stdout(if verbose { std::process::Stdio::inherit() } else { std::process::Stdio::null() })
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("Failed to execute upgrade command");

    upgrade_cmd.wait().expect("Upgrade command wasn't running");

    if !silent {
        println!("yu: System upgraded");
    }
}
