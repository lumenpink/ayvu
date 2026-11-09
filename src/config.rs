//! Configuração do Ayvu em conformidade com a especificação OpenDesktop (XDG)

use std::env;
use std::fs;
use std::path::PathBuf;

/// Retorna o caminho do arquivo de configuração do usuário ($XDG_CONFIG_HOME/ayvu/config.toml).
pub fn get_config_path() -> PathBuf {
    if let Ok(dir) = env::var("XDG_CONFIG_HOME") {
        if !dir.trim().is_empty() {
            return PathBuf::from(dir).join("ayvu").join("config.toml");
        }
    }
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join(".config")
        .join("ayvu")
        .join("config.toml")
}

/// Retorna o caminho do arquivo de estado/cache ($XDG_STATE_HOME/ayvu/update_check.json).
pub fn get_state_path() -> PathBuf {
    if let Ok(dir) = env::var("XDG_STATE_HOME") {
        if !dir.trim().is_empty() {
            return PathBuf::from(dir).join("ayvu").join("update_check.json");
        }
    }
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join(".local")
        .join("state")
        .join("ayvu")
        .join("update_check.json")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub channel: String,
    pub pinned: bool,
    pub pin: String,
    pub auto_check: bool,
    pub check_interval_hours: u64,
    pub auto_update: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            channel: "stable".to_string(),
            pinned: false,
            pin: String::new(),
            auto_check: true,
            check_interval_hours: 24,
            auto_update: false,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let path = get_config_path();
        if !path.exists() {
            return Self::default();
        }

        match fs::read_to_string(&path) {
            Ok(content) => Self::parse_toml(&content),
            Err(_) => Self::default(),
        }
    }

    pub fn parse_toml(content: &str) -> Self {
        let mut cfg = Self::default();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            if let Some((key, val)) = trimmed.split_once('=') {
                let key = key.trim();
                let val = val.trim();

                match key {
                    "channel" => {
                        cfg.channel = unquote(val);
                    }
                    "pinned" => {
                        cfg.pinned = val.parse::<bool>().unwrap_or(false);
                    }
                    "pin" => {
                        cfg.pin = unquote(val);
                    }
                    "auto_check" => {
                        cfg.auto_check = val.parse::<bool>().unwrap_or(true);
                    }
                    "check_interval_hours" => {
                        cfg.check_interval_hours = val.parse::<u64>().unwrap_or(24);
                    }
                    "auto_update" => {
                        cfg.auto_update = val.parse::<bool>().unwrap_or(false);
                    }
                    _ => {}
                }
            }
        }

        cfg
    }

    pub fn to_toml(&self) -> String {
        format!(
            "channel = \"{}\"\npinned = {}\npin = \"{}\"\nauto_check = {}\ncheck_interval_hours = {}\nauto_update = {}\n",
            self.channel,
            self.pinned,
            self.pin,
            self.auto_check,
            self.check_interval_hours,
            self.auto_update
        )
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = get_config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, self.to_toml())
    }

    pub fn pin_version(&mut self, ver: Option<String>) {
        self.pinned = true;
        self.pin = ver.unwrap_or_else(|| crate::version().to_string());
    }

    pub fn unpin_version(&mut self) {
        self.pinned = false;
        self.pin.clear();
    }

    pub fn set_channel(&mut self, channel: &str) {
        self.channel = channel.trim().to_string();
    }

    pub fn set_auto_check(&mut self, enabled: bool) {
        self.auto_check = enabled;
    }

    pub fn set_auto_update(&mut self, enabled: bool) {
        self.auto_update = enabled;
    }
}

fn unquote(val: &str) -> String {
    let s = val.trim();
    if ((s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')))
        && s.len() >= 2
    {
        return s[1..s.len() - 1].to_string();
    }
    s.to_string()
}

/// Resolve o canal efetivo respeitando a ordem de precedência:
/// flag CLI > env AYVU_CHANNEL > config.toml > default "stable"
pub fn resolve_channel(cli_flag: Option<&str>) -> String {
    if let Some(ch) = cli_flag {
        if !ch.trim().is_empty() {
            return ch.trim().to_string();
        }
    }

    if let Ok(env_ch) = env::var("AYVU_CHANNEL") {
        if !env_ch.trim().is_empty() {
            return env_ch.trim().to_string();
        }
    }

    let cfg = Config::load();
    if !cfg.channel.trim().is_empty() {
        return cfg.channel;
    }

    "stable".to_string()
}
