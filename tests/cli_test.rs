use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_cli_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_ayvu"))
        .arg("--help")
        .output()
        .expect("failed to execute ayvu --help");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("render"));
    assert!(stdout.contains("serve"));
}

#[test]
fn test_cli_render_command() {
    let dir = tempdir().expect("failed to create tempdir");
    let input_path = dir.path().join("dialogue.txt");
    let output_path = dir.path().join("comic.svg");

    let script = "Alice (happy): Test from CLI!\nBob (thinking): Works great!";
    std::fs::write(&input_path, script).expect("failed to write input");

    let status = Command::new(env!("CARGO_BIN_EXE_ayvu"))
        .arg("render")
        .arg(&input_path)
        .arg("-o")
        .arg(&output_path)
        .arg("--title")
        .arg("CLI Test Strip")
        .status()
        .expect("failed to run ayvu render");

    assert!(status.success());
    assert!(output_path.exists());

    let svg_content = std::fs::read_to_string(&output_path).expect("failed to read rendered svg");
    assert!(svg_content.contains("<svg"));
    assert!(svg_content.contains("CLI Test Strip"));
    assert!(svg_content.contains("Test from CLI!"));
    assert!(svg_content.contains("Works great!"));
}

#[test]
fn test_cli_version_flag() {
    let expected_version = include_str!("../VERSION").trim();
    let output = Command::new(env!("CARGO_BIN_EXE_ayvu"))
        .arg("--version")
        .output()
        .expect("failed to execute ayvu --version");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(expected_version),
        "Expected version '{}' in stdout, got '{}'",
        expected_version,
        stdout
    );
}

#[test]
fn test_lib_version_function() {
    let expected_version = include_str!("../VERSION").trim();
    assert_eq!(ayvu::version(), expected_version);
    assert_eq!(ayvu::VERSION, expected_version);
}

