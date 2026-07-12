use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("clash2linux").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("clash2linux"));
}

#[test]
fn test_subcommands_listed_in_help() {
    let mut cmd = Command::cargo_bin("clash2linux").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("update-core"))
        .stdout(predicate::str::contains("subscribe"))
        .stdout(predicate::str::contains("start"))
        .stdout(predicate::str::contains("stop"))
        .stdout(predicate::str::contains("status"));
}

#[test]
fn test_tui_placeholder() {
    let mut cmd = Command::cargo_bin("clash2linux").unwrap();
    cmd.arg("tui");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("TUI 界面将在后续版本实现"));
}
