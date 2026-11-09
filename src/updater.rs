//! Mecanismo de auto-atualização atômica e verificação de lançamentos da Banca

use crate::config::{get_state_path, Config};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateCheckState {
    pub last_checked_timestamp: u64,
    pub latest_version: String,
    pub channel: String,
}

impl Default for UpdateCheckState {
    fn default() -> Self {
        Self {
            last_checked_timestamp: 0,
            latest_version: String::new(),
            channel: "stable".to_string(),
        }
    }
}

impl UpdateCheckState {
    pub fn load() -> Self {
        let path = get_state_path();
        if !path.exists() {
            return Self::default();
        }

        match fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = get_state_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)
    }
}

/// Backup path para um binário especificado
pub fn get_backup_path(bin_path: &Path) -> PathBuf {
    let file_name = bin_path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "ayvu".to_string());
    bin_path.with_file_name(format!("{}.bak", file_name))
}

/// Substituição atômica de binário com backup e preservação de permissões executáveis
pub fn atomic_replace(bin_path: &Path, new_binary_data: &[u8]) -> Result<(), String> {
    let parent = bin_path
        .parent()
        .ok_or_else(|| "Caminho inválido do binário".to_string())?;

    if !parent.exists() {
        fs::create_dir_all(parent).map_err(|e| format!("Falha ao criar diretório pai: {}", e))?;
    }

    // Gerar nome de arquivo temporário único no mesmo diretório/sistema de arquivos
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let temp_path = parent.join(format!(".ayvu-tmp-{}.tmp", now));

    fs::write(&temp_path, new_binary_data)
        .map_err(|e| format!("Falha ao escrever binário temporário: {}", e))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Err(e) = fs::set_permissions(&temp_path, fs::Permissions::from_mode(0o755)) {
            let _ = fs::remove_file(&temp_path);
            return Err(format!("Falha ao atribuir permissões de execução: {}", e));
        }
    }

    // Criar backup se o binário original existir
    let backup_path = get_backup_path(bin_path);
    if bin_path.exists() {
        if let Err(e) = fs::copy(bin_path, &backup_path) {
            let _ = fs::remove_file(&temp_path);
            return Err(format!("Falha ao criar backup do binário anterior: {}", e));
        }
    }

    // Substituição atômica
    if let Err(e) = fs::rename(&temp_path, bin_path) {
        let _ = fs::remove_file(&temp_path);
        // Tentar restaurar backup se necessário
        if backup_path.exists() && !bin_path.exists() {
            let _ = fs::copy(&backup_path, bin_path);
        }
        return Err(format!("Falha na substituição atômica: {}", e));
    }

    Ok(())
}

/// Restaura o binário a partir do arquivo de backup
pub fn rollback(bin_path: &Path) -> Result<(), String> {
    let backup_path = get_backup_path(bin_path);
    if !backup_path.exists() {
        return Err("Arquivo de backup não encontrado para restauração".to_string());
    }

    fs::copy(&backup_path, bin_path).map_err(|e| format!("Falha ao restaurar backup: {}", e))?;

    Ok(())
}

/// Obtém timestamp atual em segundos
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Consulta o manifesto de distribuição da Banca para detectar nova versão
pub fn check_banca_manifest(channel: &str) -> Result<Option<String>, String> {
    let manifest_url = "https://banca.2lp.in/downloads/ayvu/index.json".to_string();

    let agent = ureq::AgentBuilder::new()
        .timeout(std::time::Duration::from_secs(5))
        .build();

    let response = match agent.get(&manifest_url).call() {
        Ok(res) => res,
        Err(e) => return Err(format!("Erro ao acessar a Banca: {}", e)),
    };

    let body = response
        .into_string()
        .map_err(|e| format!("Falha ao ler resposta da Banca: {}", e))?;

    let val: serde_json::Value =
        serde_json::from_str(&body).map_err(|e| format!("Manifesto JSON inválido: {}", e))?;

    // Estrutura esperada: channels.<channel>.version ou releases[...].version
    let latest_ver = if let Some(ver) = val.pointer(&format!("/channels/{}/version", channel)) {
        ver.as_str().map(|s| s.to_string())
    } else if let Some(items) = val.get("items").and_then(|v| v.as_array()) {
        items
            .iter()
            .find(|item| item.get("channel").and_then(|c| c.as_str()) == Some(channel))
            .and_then(|item| item.get("version").and_then(|v| v.as_str()))
            .map(|s| s.to_string())
    } else {
        None
    };

    Ok(latest_ver)
}

/// Verificação discreta no startup da CLI com saída exclusivamente para stderr
pub fn check_startup_notice(current_version: &str) {
    let cfg = Config::load();
    if !cfg.auto_check || cfg.pinned {
        return;
    }

    let mut state = UpdateCheckState::load();
    let now = current_timestamp();
    let interval_secs = cfg.check_interval_hours * 3600;

    if now < state.last_checked_timestamp + interval_secs && !state.latest_version.is_empty() {
        if state.latest_version != current_version {
            eprintln!(
                "aviso: versao mais recente do ayvu ({}) disponivel no canal {}. Execute 'ayvu update'.",
                state.latest_version, state.channel
            );
        }
        return;
    }

    // Tentar atualizar o estado
    if let Ok(Some(latest)) = check_banca_manifest(&cfg.channel) {
        state.last_checked_timestamp = now;
        state.latest_version = latest.clone();
        state.channel = cfg.channel.clone();
        let _ = state.save();

        if latest != current_version {
            eprintln!(
                "aviso: versao mais recente do ayvu ({}) disponivel no canal {}. Execute 'ayvu update'.",
                latest, cfg.channel
            );
        }
    }
}
