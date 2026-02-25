use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub monitors: HashMap<String, MonitorConfig>,
    #[serde(default)]
    pub favorites: Vec<Favorite>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitorConfig {
    #[serde(default)]
    pub input_aliases: HashMap<u16, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Favorite {
    pub monitor_id: String,
    pub input_value: u16,
}

impl Config {
    pub fn config_path() -> Option<PathBuf> {
        dirs::home_dir().map(|p| p.join(".config").join("monitor-switch").join("config.json"))
    }

    pub fn load() -> Self {
        Self::config_path()
            .and_then(|path| fs::read_to_string(&path).ok())
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path().ok_or("Could not determine config directory")?;
        
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        
        let content = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(&path, content).map_err(|e| e.to_string())
    }

    pub fn get_alias(&self, monitor_id: &str, input_value: u16) -> Option<&str> {
        self.monitors
            .get(monitor_id)
            .and_then(|m| m.input_aliases.get(&input_value))
            .map(|s| s.as_str())
    }

    pub fn set_alias(&mut self, monitor_id: &str, input_value: u16, alias: String) {
        self.monitors
            .entry(monitor_id.to_string())
            .or_default()
            .input_aliases
            .insert(input_value, alias);
    }

    pub fn remove_alias(&mut self, monitor_id: &str, input_value: u16) {
        if let Some(monitor) = self.monitors.get_mut(monitor_id) {
            monitor.input_aliases.remove(&input_value);
        }
    }

    pub fn is_favorite(&self, monitor_id: &str, input_value: u16) -> bool {
        self.favorites.iter().any(|f| f.monitor_id == monitor_id && f.input_value == input_value)
    }

    pub fn add_favorite(&mut self, monitor_id: &str, input_value: u16) {
        let fav = Favorite {
            monitor_id: monitor_id.to_string(),
            input_value,
        };
        if !self.favorites.contains(&fav) {
            self.favorites.push(fav);
        }
    }

    pub fn remove_favorite(&mut self, monitor_id: &str, input_value: u16) {
        self.favorites.retain(|f| !(f.monitor_id == monitor_id && f.input_value == input_value));
    }

    pub fn get_favorites(&self) -> &[Favorite] {
        &self.favorites
    }
}

