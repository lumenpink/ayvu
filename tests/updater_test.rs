use std::env;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_xdg_config_paths_and_defaults() {
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
