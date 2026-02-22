use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Location stored in config (latitude, longitude, timezone, optional display name)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationConfig {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: f64,
    #[serde(default)]
    pub name: Option<String>,
}

/// Theme options for terminal output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, clap::ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum Theme {
    /// Dark background (cyan/green accents)
    #[default]
    Dark,
    /// Light background (blue/bold)
    Light,
    /// No colors, plain text
    Minimal,
    /// Colorful (multiple accent colors)
    Colorful,
}

/// User preferences for terminal customization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub theme: Theme,
    /// Use bold for section headers
    #[serde(default = "default_bold_headers")]
    pub bold_headers: bool,
    /// Use dim style for separators
    #[serde(default = "default_dim_separators")]
    pub dim_separators: bool,
    /// Location for prayer times (lat, long, timezone, optional display name)
    #[serde(default)]
    pub location: Option<LocationConfig>,
}

fn default_bold_headers() -> bool {
    true
}
fn default_dim_separators() -> bool {
    true
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            bold_headers: true,
            dim_separators: true,
            location: None,
        }
    }
}

/// Default Mogadishu coordinates
fn default_location() -> crate::models::types::Location {
    crate::models::types::Location {
        latitude: 2.0469,
        longitude: 45.3182,
        timezone: 3.0,
    }
}

impl AppConfig {
    /// Get location for calculation and display name (e.g. "Mogadishu" or custom name).
    pub fn get_location(&self) -> (crate::models::types::Location, String) {
        match &self.location {
            Some(loc) => (
                crate::models::types::Location {
                    latitude: loc.latitude,
                    longitude: loc.longitude,
                    timezone: loc.timezone,
                },
                loc.name
                    .clone()
                    .unwrap_or_else(|| "Custom".to_string()),
            ),
            None => (default_location(), "Mogadishu".to_string()),
        }
    }

    /// Config directory: ~/.config/ramadan (Linux/Mac) or %APPDATA%\ramadan (Windows)
    pub fn config_dir() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("ramadan"))
    }

    /// Full path to config file
    pub fn config_path() -> Option<PathBuf> {
        Self::config_dir().map(|d| d.join("config.toml"))
    }

    /// Load config from disk; returns default if missing or on error
    pub fn load() -> Self {
        let path = match Self::config_path() {
            Some(p) => p,
            None => return Self::default(),
        };
        let s = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(_) => return Self::default(),
        };
        toml::from_str(&s).unwrap_or_default()
    }

    /// Save config to disk
    pub fn save(&self) -> anyhow::Result<()> {
        let dir = Self::config_dir().ok_or_else(|| anyhow::anyhow!("No config directory"))?;
        fs::create_dir_all(&dir)?;
        let path = dir.join("config.toml");
        let s = toml::to_string_pretty(self)?;
        fs::write(path, s)?;
        Ok(())
    }
}

/// Old style module kept minimal – the new rendering lives in tui::render
pub mod style {
    // Retained for potential future use but all rendering now uses tui::render
}

