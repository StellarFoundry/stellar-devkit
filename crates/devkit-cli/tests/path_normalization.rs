//! Cross-platform path handling for file-based commands.

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_dir() -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "stellar-foundry-paths-{}-{suffix}",
        std::process::id()
    ))
}

fn inspect(path: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_stellar-foundry"))
        .args(["--format", "json", "inspect-wasm", path])
        .output()
        .expect("run inspect-wasm")
}

#[test]
fn native_and_forward_slash_paths_produce_identical_output() {
    let dir = unique_dir();
    let nested = dir.join("nested");
    fs::create_dir_all(&nested).expect("create fixture directory");
    let wasm = nested.join("minimal.wasm");
    fs::write(&wasm, [0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]).expect("write minimal wasm");

    let native = wasm.to_string_lossy().to_string();
    let forward = native.replace('\\', "/");
    let with_parent = nested
        .join("..")
        .join("nested")
        .join("minimal.wasm")
        .to_string_lossy()
        .to_string();

    let native_output = inspect(&native);
    let forward_output = inspect(&forward);
    let parent_output = inspect(&with_parent);

    assert!(
        native_output.status.success(),
        "native path failed: {}",
        String::from_utf8_lossy(&native_output.stderr)
    );
    assert_eq!(native_output.stdout, forward_output.stdout);
    assert_eq!(native_output.stdout, parent_output.stdout);

    fs::remove_dir_all(&dir).expect("remove fixture directory");
}
