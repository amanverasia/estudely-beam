use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use estudely_core::config::{DEFAULT_CODE_LENGTH, DEFAULT_RELAY_URL};
use estudely_core::EstudelyConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub relay_url: String,
    pub code_length: usize,
    pub download_dir: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        let download_dir = dirs::download_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .to_string_lossy()
            .to_string();

        Self {
            relay_url: DEFAULT_RELAY_URL.to_string(),
            code_length: DEFAULT_CODE_LENGTH,
            download_dir,
        }
    }
}

impl AppSettings {
    pub fn to_core_config(&self) -> EstudelyConfig {
        EstudelyConfig {
            relay_url: if self.relay_url == DEFAULT_RELAY_URL {
                None
            } else {
                Some(self.relay_url.clone())
            },
            code_length: self.code_length,
            ..Default::default()
        }
    }

    pub fn config_path(app_data_dir: &std::path::Path) -> PathBuf {
        app_data_dir.join("settings.json")
    }

    pub fn load(app_data_dir: &std::path::Path) -> Self {
        let path = Self::config_path(app_data_dir);
        match std::fs::read_to_string(&path) {
            Ok(json) => serde_json::from_str(&json).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self, app_data_dir: &std::path::Path) -> Result<(), String> {
        let path = Self::config_path(app_data_dir);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(&path, json).map_err(|e| e.to_string())?;
        Ok(())
    }
}
