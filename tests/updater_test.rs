use std::env;
use std::fs;
use std::process::Command;
use std::sync::Mutex;
use tempfile::tempdir;

static ENV_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_xdg_config_paths_and_defaults() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let temp = tempdir().expect("tempdir");
    let config_dir = temp.path().join("config");
    let state_dir = temp.path().join("state");

    env::set_var("XDG_CONFIG_HOME", &config_dir);
    env::set_var("XDG_STATE_HOME", &state_dir);
    env::remove_var("AYVU_CHANNEL");

    let cfg_path = ayvu::config::get_config_path();
    assert_eq!(cfg_path, config_dir.join("ayvu").join("config.toml"));

    let state_path = ayvu::config::get_state_path();
    assert_eq!(state_path, state_dir.join("ayvu").join("update_check.json"));

    let cfg = ayvu::config::Config::load();
    assert_eq!(cfg.channel, "stable");
    assert!(!cfg.pinned);
    assert_eq!(cfg.pin, "");
    assert!(cfg.auto_check);
    assert_eq!(cfg.check_interval_hours, 24);
    assert!(!cfg.auto_update);
}

#[test]
fn test_xdg_config_precedence() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let temp = tempdir().expect("tempdir");
    let config_dir = temp.path().join("config");
    let ayvu_dir = config_dir.join("ayvu");
    fs::create_dir_all(&ayvu_dir).expect("create dir");

    let toml_content = r#"
channel = "nightly"
pinned = true
pin = "0.1.0-dev.10"
auto_check = false
check_interval_hours = 12
auto_update = true
"#;
    fs::write(ayvu_dir.join("config.toml"), toml_content).expect("write toml");

    env::set_var("XDG_CONFIG_HOME", &config_dir);
    env::remove_var("AYVU_CHANNEL");

    let cfg = ayvu::config::Config::load();
    assert_eq!(cfg.channel, "nightly");
    assert!(cfg.pinned);
    assert_eq!(cfg.pin, "0.1.0-dev.10");
    assert!(!cfg.auto_check);
    assert_eq!(cfg.check_interval_hours, 12);
    assert!(cfg.auto_update);

    // Env var overrides config file channel
    env::set_var("AYVU_CHANNEL", "stable");
    let resolved_channel = ayvu::config::resolve_channel(None);
    assert_eq!(resolved_channel, "stable");

    // Flag overrides env var
    let flag_override = ayvu::config::resolve_channel(Some("custom-channel"));
    assert_eq!(flag_override, "custom-channel");

    env::remove_var("AYVU_CHANNEL");
}

#[test]
fn test_config_mutators_and_persistence() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let temp = tempdir().expect("tempdir");
    let config_dir = temp.path().join("config");
    env::set_var("XDG_CONFIG_HOME", &config_dir);

    let mut cfg = ayvu::config::Config::default();
    cfg.set_channel("nightly");
    cfg.pin_version(Some("0.1.0-test.1".to_string()));
    cfg.set_auto_check(false);
    cfg.set_auto_update(true);

    assert_eq!(cfg.channel, "nightly");
    assert!(cfg.pinned);
    assert_eq!(cfg.pin, "0.1.0-test.1");
    assert!(!cfg.auto_check);
    assert!(cfg.auto_update);

    assert!(cfg.save().is_ok());

    let loaded = ayvu::config::Config::load();
    assert_eq!(loaded.channel, "nightly");
    assert!(loaded.pinned);
    assert_eq!(loaded.pin, "0.1.0-test.1");
    assert!(!loaded.auto_check);
    assert!(loaded.auto_update);

    let mut unpinned = loaded;
    unpinned.unpin_version();
    assert!(!unpinned.pinned);
    assert!(unpinned.pin.is_empty());
}

#[test]
fn test_atomic_binary_replacement_and_rollback() {
    let temp = tempdir().expect("tempdir");
    let bin_path = temp.path().join("ayvu");
    let backup_path = temp.path().join("ayvu.bak");

    fs::write(&bin_path, b"original-v1-binary").expect("write original");

    // Replace with new valid content
    let new_content = b"updated-v2-binary";
    let replace_res = ayvu::updater::atomic_replace(&bin_path, new_content);
    assert!(replace_res.is_ok());
    assert_eq!(fs::read(&bin_path).unwrap(), new_content);
    assert_eq!(fs::read(&backup_path).unwrap(), b"original-v1-binary");

    // Test rollback capability
    let rollback_res = ayvu::updater::rollback(&bin_path);
    assert!(rollback_res.is_ok());
    assert_eq!(fs::read(&bin_path).unwrap(), b"original-v1-binary");
}

#[test]
fn test_cli_config_commands() {
    let temp = tempdir().expect("tempdir");
    let config_dir = temp.path().join("cfg");
    let state_dir = temp.path().join("st");

    let bin = env!("CARGO_BIN_EXE_ayvu");

    // ayvu config set-channel nightly
    let out = Command::new(bin)
        .env("XDG_CONFIG_HOME", &config_dir)
        .env("XDG_STATE_HOME", &state_dir)
        .args(["config", "set-channel", "nightly"])
        .output()
        .expect("set-channel failed");
    assert!(out.status.success());

    // ayvu config get
    let out = Command::new(bin)
        .env("XDG_CONFIG_HOME", &config_dir)
        .env("XDG_STATE_HOME", &state_dir)
        .args(["config", "get"])
        .output()
        .expect("config get failed");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("channel = \"nightly\""));

    // ayvu config pin
    let out = Command::new(bin)
        .env("XDG_CONFIG_HOME", &config_dir)
        .env("XDG_STATE_HOME", &state_dir)
        .args(["config", "pin", "0.2.0-pinned"])
        .output()
        .expect("config pin failed");
    assert!(out.status.success());

    // ayvu config show
    let out = Command::new(bin)
        .env("XDG_CONFIG_HOME", &config_dir)
        .env("XDG_STATE_HOME", &state_dir)
        .args(["config", "show"])
        .output()
        .expect("config show failed");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Pinned:      true"));
    assert!(stdout.contains("Pin:         0.2.0-pinned"));
}
