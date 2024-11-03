use crate::root::get_sudo;

pub fn autoremove(manager: String, slient: bool, verbose: bool) {
    if !slient {
        println!("yu: Auto removing unused packages")
    }
}
