use crate::syntax;

pub fn list(manager: String, silent: bool, verbose: bool) {
    if !silent {
        
    }

    let mut cmd = syntax::gen_list_syntax(manager.clone())
        .stdout(if verbose { std::process::Stdio::inherit() } else { std::process::Stdio::null() })
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("yu: Failed to execute command");

    cmd.wait().expect("Command wasn't running");
}
