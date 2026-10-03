use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn test_makefile_structure_and_dry_run() {
    let makefile = Path::new("Makefile");
    assert!(makefile.is_file(), "Makefile should exist at project root");

    let content = fs::read_to_string(makefile).expect("Makefile readable");
    assert!(content.contains("release:"));
    assert!(content.contains("install:"));
    assert!(content.contains("uninstall:"));
    assert!(content.contains("clean:"));

    // Dry run targets
    let status = Command::new("make")
        .arg("-n")
        .arg("release")
        .status()
        .expect("make -n release should run");
    assert!(status.success(), "make -n release should succeed");

    let test_status = Command::new("make")
        .arg("-n")
        .arg("test")
        .status()
        .expect("make -n test should run");
    assert!(test_status.success(), "make -n test should succeed");
}

#[test]
fn test_install_script() {
    let script = Path::new("install.sh");
    assert!(script.is_file(), "install.sh should exist at project root");

    let output = Command::new("sh")
        .arg("install.sh")
        .arg("--help")
        .output()
        .expect("sh install.sh --help should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("--prefix"));
}

#[test]
fn test_dockerfile_and_compose_specs() {
    let dockerfile = Path::new("Dockerfile");
    assert!(dockerfile.is_file(), "Dockerfile should exist");

    let docker_content = fs::read_to_string(dockerfile).expect("Dockerfile readable");
    assert!(docker_content.contains("FROM"));
    assert!(docker_content.contains("EXPOSE 3100"));
    assert!(docker_content.contains("HEALTHCHECK"));
    assert!(docker_content.contains("ayvu"));

    let compose = Path::new("compose.yaml");
    assert!(compose.is_file(), "compose.yaml should exist");

    let compose_content = fs::read_to_string(compose).expect("compose.yaml readable");
    assert!(compose_content.contains("services:"));
    assert!(compose_content.contains("3100:3100"));
}
