use crate::config::settings::{AppConfig, Theme};
use colored::Colorize;

pub fn run_show() {
    let config = AppConfig::load();
    let path = AppConfig::config_path().unwrap_or_else(|| std::path::PathBuf::from("(unknown)"));

    println!("{}", "Configuration (Config)".bold().cyan());
    println!("{}", "-----------------------------".dimmed());
    println!("  {} {}", "Config file:".dimmed(), path.display());
    println!(
        "  {} {}",
        "Theme:".dimmed(),
        theme_display_name(config.theme)
    );
    println!(
        "  {} {}",
        "Bold headers:".dimmed(),
        if config.bold_headers { "yes" } else { "no" }
    );
    println!(
        "  {} {}",
        "Dim separators:".dimmed(),
        if config.dim_separators { "yes" } else { "no" }
    );
    println!();
    println!("{}", "Available themes: dark, light, minimal, colorful".dimmed());
    println!(
        "{}",
        "Change with: ramadan config set --theme <name>".dimmed()
    );
}

pub fn run_set(theme: Option<Theme>, bold_headers: Option<bool>, dim_separators: Option<bool>) {
    let mut config = AppConfig::load();

    if let Some(t) = theme {
        config.theme = t;
        println!("{} {}", "Theme set to:".green(), theme_display_name(t));
    }
    if let Some(b) = bold_headers {
        config.bold_headers = b;
        println!(
            "{} {}",
            "Bold headers:".green(),
            if b { "enabled" } else { "disabled" }
        );
    }
    if let Some(d) = dim_separators {
        config.dim_separators = d;
        println!(
            "{} {}",
            "Dim separators:".green(),
            if d { "enabled" } else { "disabled" }
        );
    }

    if theme.is_none() && bold_headers.is_none() && dim_separators.is_none() {
        println!("{}", "No options given. Use --theme, --bold-headers, --dim-separators".yellow());
        return;
    }

    match config.save() {
        Ok(()) => {
            println!("{}", "Configuration saved.".green());
        }
        Err(e) => {
            eprintln!("{} {}", "Error saving config:".red(), e);
        }
    }
}

fn theme_display_name(t: Theme) -> &'static str {
    match t {
        Theme::Dark => "dark",
        Theme::Light => "light",
        Theme::Minimal => "minimal",
        Theme::Colorful => "colorful",
    }
}
