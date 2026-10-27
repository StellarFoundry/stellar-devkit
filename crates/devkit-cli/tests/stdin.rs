use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn scval_reads_one_value_from_stdin() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_stellar-foundry"))
        .args(["--format", "json", "scval", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("start CLI");

    child
        .stdin
        .take()
        .expect("stdin pipe")
        .write_all(b"AAAAAwAAACo=")
        .expect("write XDR input");

    let output = child.wait_with_output().expect("wait for CLI");
    assert!(
        output.status.success(),
        "CLI failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("valid JSON output");
    assert_eq!(value["kind"], "scval");
}

#[test]
fn stdin_input_is_bounded() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_stellar-foundry"))
        .args(["--format", "json", "scval", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("start CLI");

    let mut stdin = child.stdin.take().expect("stdin pipe");
    let oversized = vec![b'A'; 16 * 1024 * 1024 + 1];
    stdin.write_all(&oversized).expect("write oversized input");
    drop(stdin);

    let output = child.wait_with_output().expect("wait for CLI");
    assert_eq!(output.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("16 MiB")
            || String::from_utf8_lossy(&output.stderr).contains("16777216-byte limit")
    );
}
