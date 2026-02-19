use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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
        }
    }
}

impl AppConfig {
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

/// Apply theme/style to terminal output (ANSI via `colored` crate)
pub mod style {
    use colored::Colorize;

    use super::{AppConfig, Theme};

    fn bold_if(s: &str, bold: bool) -> String {
        if bold {
            s.bold().to_string()
        } else {
            s.to_string()
        }
    }

    fn dim_if(s: &str, dim: bool) -> String {
        if dim {
            s.dimmed().to_string()
        } else {
            s.to_string()
        }
    }

    pub fn welcome(config: &AppConfig) -> String {
        let s = "Ku soo dhawaaw Ramadan CLI 🌙";
        match config.theme {
            Theme::Dark => s.cyan().to_string(),
            Theme::Light => s.blue().bold().to_string(),
            Theme::Minimal => s.to_string(),
            Theme::Colorful => s.truecolor(255, 179, 71).bold().to_string(), // warm orange
        }
    }

    pub fn title(config: &AppConfig, text: &str) -> String {
        let s = bold_if(text, config.bold_headers);
        match config.theme {
            Theme::Dark => s.cyan().to_string(),
            Theme::Light => s.blue().to_string(),
            Theme::Minimal => s,
            Theme::Colorful => s.truecolor(255, 179, 71).to_string(),
        }
    }

    pub fn label(config: &AppConfig, text: &str) -> String {
        let s = bold_if(text, config.bold_headers);
        match config.theme {
            Theme::Dark => s.green().to_string(),
            Theme::Light => s.blue().dimmed().to_string(),
            Theme::Minimal => s,
            Theme::Colorful => s.truecolor(135, 206, 250).to_string(), // light blue
        }
    }

    pub fn value(config: &AppConfig, text: &str) -> String {
        match config.theme {
            Theme::Dark => text.white().to_string(),
            Theme::Light => text.truecolor(30, 30, 30).to_string(),
            Theme::Minimal => text.to_string(),
            Theme::Colorful => text.truecolor(230, 230, 230).to_string(),
        }
    }

    pub fn separator(config: &AppConfig) -> String {
        let s = "-----------------------------";
        let s = dim_if(s, config.dim_separators);
        match config.theme {
            Theme::Dark => s.dimmed().to_string(),
            Theme::Light => s.dimmed().to_string(),
            Theme::Minimal => s,
            Theme::Colorful => s.truecolor(100, 100, 100).to_string(),
        }
    }

    /// Format a line like "Fajr:     05:23"
    pub fn prayer_line(config: &AppConfig, name: &str, time: &str) -> String {
        format!(
            "{} {}",
            label(config, &format!("{name}:")),
            value(config, time)
        )
    }
}
