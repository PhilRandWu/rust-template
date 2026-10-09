use std::process::Command;

#[test]
fn cli_prints_default_greeting() {
    let bin = std::env::var("CARGO_BIN_EXE_{{crate_name | replace: "_", "-"}}")
        .expect("CLI binary path should be provided by Cargo");
    let output = Command::new(bin).output().expect("CLI binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    assert!(stdout.contains("Hello, world!"));
}

#[test]
fn cli_rejects_empty_name() {
    let bin = std::env::var("CARGO_BIN_EXE_{{crate_name | replace: "_", "-"}}")
        .expect("CLI binary path should be provided by Cargo");
    let output = Command::new(bin)
        .args(["--name", ""])
        .output()
        .expect("CLI binary should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("stderr should be UTF-8");
    assert!(stderr.contains("input must not be empty"));
}

#[test]
fn cli_accepts_name() {
    let bin = std::env::var("CARGO_BIN_EXE_{{crate_name | replace: "_", "-"}}")
        .expect("CLI binary path should be provided by Cargo");
    let output = Command::new(bin)
        .args(["--name", "Rust"])
        .output()
        .expect("CLI binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    assert!(stdout.contains("Hello, Rust!"));
}
