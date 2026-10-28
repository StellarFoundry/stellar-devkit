//! Cross-platform behaviour: path separators and line endings.
//!
//! CI runs these tests on Linux, macOS, and Windows. They assert host-independent
//! values rather than comparing against a platform-specific string.

use std::path::{Component, Path, PathBuf};

use devkit_core::{Network, OutputFormat};

#[test]
fn native_and_forward_slash_paths_have_identical_components() {
    let forward = Path::new("fixtures/malformed/scval-empty.txt");
    let native: PathBuf = ["fixtures", "malformed", "scval-empty.txt"]
        .iter()
        .collect();

    let forward_components: Vec<Component<'_>> = forward.components().collect();
    let native_components: Vec<Component<'_>> = native.components().collect();

    assert_eq!(forward_components, native_components);
    assert_eq!(
        forward_components
            .iter()
            .filter_map(|component| match component {
                Component::Normal(part) => Some(*part),
                _ => None,
            })
            .collect::<Vec<_>>(),
        vec![
            std::ffi::OsStr::new("fixtures"),
            std::ffi::OsStr::new("malformed"),
            std::ffi::OsStr::new("scval-empty.txt"),
        ]
    );
}

#[test]
fn current_directory_components_are_removed_from_both_separator_styles() {
    let forward = Path::new("fixtures/./malformed/scval-empty.txt");
    let native = Path::new("fixtures")
        .join(".")
        .join("malformed")
        .join("scval-empty.txt");

    let has_current_dir = |path: &Path| {
        path.components()
            .any(|component| component == Component::CurDir)
    };
    assert!(!has_current_dir(forward));
    assert!(!has_current_dir(&native));
    assert_eq!(
        forward.components().collect::<Vec<_>>(),
        native.components().collect::<Vec<_>>()
    );
}

#[test]
fn parsers_ignore_platform_line_endings_and_surrounding_whitespace() {
    assert_eq!(
        "json\r\n".parse::<OutputFormat>().unwrap(),
        OutputFormat::Json
    );
    assert_eq!("testnet\r\n".parse::<Network>().unwrap(), Network::Testnet);
    assert_eq!(
        " JSON ".parse::<OutputFormat>().unwrap(),
        OutputFormat::Json
    );
}
