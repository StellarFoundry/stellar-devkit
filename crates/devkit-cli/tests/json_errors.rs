use std::process::{Command, Output};

fn run_cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_stellar-foundry"))
        .args(args)
        .output()
        .expect("run CLI")
}

#[test]
fn decode_error_uses_structured_json_and_keeps_exit_code() {
    let output = run_cli(&["--format", "json", "scval", "not-valid-base64!!!"]);
    assert_eq!(output.status.code(), Some(3));
    let value: serde_json::Value =
        serde_json::from_slice(&output.stderr).expect("JSON error on stderr");
    assert_eq!(value["error"]["category"], "decode");
    assert!(value["error"]["message"]
        .as_str()
        .unwrap()
        .contains("decode error"));
    assert!(output.stdout.is_empty());
}

#[test]
fn usage_error_uses_structured_json_without_echoing_arguments() {
    let output = run_cli(&["--format", "json", "scval"]);
    assert_eq!(output.status.code(), Some(2));
    let value: serde_json::Value =
        serde_json::from_slice(&output.stderr).expect("JSON usage error on stderr");
    assert_eq!(value["error"]["category"], "usage");
    assert!(value["error"]["message"]
        .as_str()
        .unwrap()
        .contains("invalid command-line"));
    assert!(output.stdout.is_empty());
}

#[test]
fn terminal_errors_remain_human_readable() {
    let output = run_cli(&["scval", "not-valid-base64!!!"]);
    assert_eq!(output.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&output.stderr).starts_with("error: decode error:"));
}
