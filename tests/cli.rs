//! End-to-end tests that spawn the real `yu` binary.
//!
//! `CARGO_BIN_EXE_yu` is injected by Cargo and points at the freshly built
//! binary, so these exercise argument parsing and dispatch for real without
//! ever invoking a system package manager.

use std::process::Command;

fn yu() -> Command {
    Command::new(env!("CARGO_BIN_EXE_yu"))
}

#[test]
fn version_flag_prints_crate_version() {
    let output = yu().arg("--version").output().expect("failed to spawn yu");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(env!("CARGO_PKG_VERSION")),
        "expected version in output, got: {stdout}"
    );
}

#[test]
fn help_flag_succeeds_and_describes_the_tool() {
    let output = yu().arg("--help").output().expect("failed to spawn yu");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("package manager"),
        "expected help text, got: {stdout}"
    );
}

#[test]
fn unknown_subcommand_reports_an_error_without_crashing() {
    let output = yu()
        .arg("definitely-not-a-real-subcommand")
        .output()
        .expect("failed to spawn yu");

    // Depending on whether the host has a recognised package manager, this is
    // either "Unknown command: ..." or "Unknown package manager"; both report
    // the problem on stderr and neither should panic.
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Unknown"),
        "expected an 'Unknown' diagnostic, got stderr: {stderr}"
    );
}
