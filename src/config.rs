
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub interval_secs: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub apps: HashMap<String, AppConfig>,
}

impl Default for Config {
    fn default() -> Self {
        let mut apps = HashMap::new();
        apps.insert(
            "Things".to_string(),
            AppConfig { interval_secs: 2 },
        );
        Self { apps }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = Self::config_path();
        
        if !config_path.exists() {
            let default_config = Config::default();
            default_config.save();
            return default_config;
        }

        let config_str = fs::read_to_string(&config_path)
            .expect("Failed to read config file");
        
        toml::from_str(&config_str)
            .expect("Failed to parse config file")
    }

    pub fn save(&self) {
        let config_path = Self::config_path();
        
        // Create directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .expect("Failed to create config directory");
        }

        let toml_str = toml::to_string_pretty(self)
            .expect("Failed to serialize config");
        
        fs::write(&config_path, toml_str)
            .expect("Failed to write config file");
    }

    fn config_path() -> PathBuf {
        let home = dirs::home_dir().expect("Could not find home directory");
        home.join(".config/mini-badger/mini-badger.toml")
    }
}
