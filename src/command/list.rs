use crate::syntax;

pub fn list(manager: String) {
    let mut list_cmd = syntax::gen_list_syntax(manager.clone())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("yu: Failed to execute command");

    list_cmd.wait().expect("Command wasn't running");
}
