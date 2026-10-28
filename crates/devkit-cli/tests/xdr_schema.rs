use std::process::{Command, Output};

fn run_cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_stellar-foundry"))
        .args(args)
        .output()
        .expect("run CLI")
}

#[test]
fn scval_json_output_is_versioned() {
    let output = run_cli(&["--format", "json", "scval", "AAAAAwAAACo="]);
    assert!(
        output.status.success(),
        "CLI failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("valid JSON output");
    assert_eq!(value["schema_version"], "1.0");
    assert_eq!(value["kind"], "scval");
    assert_eq!(value["value"]["u32"], 42);
    assert!(value.get("raw_xdr").is_none());
}

#[test]
fn raw_flag_echoes_the_base64_input() {
    let output = run_cli(&["--format", "json", "--raw", "scval", "AAAAAwAAACo="]);
    assert!(
        output.status.success(),
        "CLI failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("valid JSON output");
    assert_eq!(value["schema_version"], "1.0");
    assert_eq!(value["kind"], "scval");
    assert_eq!(value["raw_xdr"], "AAAAAwAAACo=");
}
