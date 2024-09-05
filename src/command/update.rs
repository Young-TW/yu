use crate::syntax;

pub fn update(manager: String, silent: bool, verbose: bool) {
    if !silent {
        println!("yu: Updating system");
    }
    let mut update_cmd = syntax::gen_update_syntax(manager.clone())
        .stdout(if verbose { std::process::Stdio::inherit() } else { std::process::Stdio::null() })
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("Failed to execute update command");

    update_cmd.wait().expect("Update command wasn't running");
}
