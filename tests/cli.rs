//! End-to-end tests that spawn the real `yu` binary.
//!
//! `CARGO_BIN_EXE_yu` is injected by Cargo and points at the freshly built
//! binary, so these exercise argument parsing and dispatch for real without
//! ever invoking a system package manager.

use std::process::{Command, Stdio};

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
        stderr.to_lowercase().contains("unknown"),
        "expected an 'unknown' diagnostic, got stderr: {stderr}"
    );
    // An unrecognised request must surface as a non-zero exit code.
    assert!(!output.status.success());
}

#[test]
fn silent_failing_command_emits_no_yu_output() {
    // Regression test for issue #7: `--silent` must suppress *all* yu-originated
    // output, including the failure diagnostic. Installing a package that cannot
    // exist drives `run_package_command` down its non-success branch, which used
    // to `eprintln!("yu: install failed")` unconditionally.
    //
    // Concrete example from the issue: `yu --silent install <nonexistent-pkg>`
    // must exit non-zero and produce empty stdout + stderr.
    let output = yu()
        .arg("--silent")
        .arg("install")
        .arg("yu-nonexistent-pkg-zzz-7")
        // Closed stdin so any `sudo` wrapper fails fast instead of prompting.
        .stdin(Stdio::null())
        .output()
        .expect("failed to spawn yu");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // On hosts with no recognised package manager, yu bails out before ever
    // running the command, so the bug's code path is never exercised. Skip
    // there rather than assert against an unrelated diagnostic.
    if stderr.contains("unknown package manager") {
        eprintln!("skipping: no package manager detected on this host");
        return;
    }

    // A failing command under --silent must still report failure via the exit
    // code...
    assert!(
        !output.status.success(),
        "a failing install must exit non-zero"
    );
    // ...but must not print anything from yu itself.
    assert!(stdout.is_empty(), "--silent leaked stdout: {stdout:?}");
    assert!(
        stderr.is_empty(),
        "--silent leaked stderr (issue #7): {stderr:?}"
    );
}

#[test]
fn multiple_packages_are_accepted_by_argument_parser() {
    // Regression test for issue #10: `yu install foo bar` was rejected by clap
    // with "unexpected argument 'bar' found" because `package` was declared as a
    // single positional argument (index 2). After the fix, multiple package names
    // must be accepted by the parser and forwarded to the underlying manager.
    //
    // This test only exercises the *parsing* layer: even if the install itself
    // fails (no package manager detected, sudo required, packages not found),
    // clap must not reject the second package name outright.
    let output = yu()
        .args(["install", "nonexistent-pkg-a", "nonexistent-pkg-b"])
        .stdin(Stdio::null())
        .output()
        .expect("failed to spawn yu");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !stderr.contains("unexpected argument"),
        "yu rejected the second package name at the CLI level (issue #10): {stderr}"
    );
}

#[test]
fn setup_sudo_makes_no_changes_without_confirmation() {
    // Closed stdin means the confirmation prompt reads EOF, so `setup-sudo`
    // must abort without ever installing a sudoers rule. This pins down the
    // security contract: nothing is granted unless the user explicitly says
    // yes. (On hosts whose manager needs no sudo it reports that instead.)
    let output = yu()
        .arg("setup-sudo")
        .stdin(Stdio::null())
        .output()
        .expect("failed to spawn yu");

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        combined.contains("aborted") || combined.contains("does not run under sudo"),
        "expected a safe no-op, got: {combined}"
    );
}
