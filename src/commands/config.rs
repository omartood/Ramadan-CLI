use crate::config::settings::{AppConfig, LocationConfig, Theme};
use colored::Colorize;

pub fn run_show() {
    let config = AppConfig::load();
    let path = AppConfig::config_path().unwrap_or_else(|| std::path::PathBuf::from("(unknown)"));
    let (loc, name) = config.get_location();

    println!("{}", "Qaabeynta (Config)".bold().cyan());
    println!("{}", "-----------------------------".dimmed());
    println!("  {} {}", "Faylka config:".dimmed(), path.display());
    println!(
        "  {} {}",
        "Theme:".dimmed(),
        theme_display_name(config.theme)
    );
    println!(
        "  {} {}",
        "Cinwaan bold:".dimmed(),
        if config.bold_headers { "haa" } else { "maya" }
    );
    println!(
        "  {} {}",
        "Xariiq dim:".dimmed(),
        if config.dim_separators { "haa" } else { "maya" }
    );
    println!("  {} {} ({}, {}, UTC{:+})", "Goobta:".dimmed(), name, loc.latitude, loc.longitude, loc.timezone);
    println!();
    println!("{}", "Theme-yada: dark, light, minimal, colorful".dimmed());
    println!("{}", "Goobta: ramadan config set --latitude <n> --longitude <n> --timezone <n> [--location-name <magac>]".dimmed());
}

pub fn run_set(
    theme: Option<Theme>,
    bold_headers: Option<bool>,
    dim_separators: Option<bool>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    timezone: Option<f64>,
    location_name: Option<String>,
) {
    let mut config = AppConfig::load();

    if let Some(t) = theme {
        config.theme = t;
        println!("{} {}", "Theme waa la beddelay:".green(), theme_display_name(t));
    }
    if let Some(b) = bold_headers {
        config.bold_headers = b;
        println!(
            "{} {}",
            "Cinwaan bold:".green(),
            if b { "furan" } else { "xiran" }
        );
    }
    if let Some(d) = dim_separators {
        config.dim_separators = d;
        println!(
            "{} {}",
            "Xariiq dim:".green(),
            if d { "furan" } else { "xiran" }
        );
    }

    if latitude.is_some() || longitude.is_some() || timezone.is_some() || location_name.is_some() {
        let (current, current_name) = config.get_location();
        let loc = LocationConfig {
            latitude: latitude.unwrap_or(current.latitude),
            longitude: longitude.unwrap_or(current.longitude),
            timezone: timezone.unwrap_or(current.timezone),
            name: location_name.clone().or(Some(current_name)),
        };
        config.location = Some(loc);
        let (l, n) = config.get_location();
        println!(
            "{} {} (lat: {}, long: {}, UTC{:+})",
            "Goobta waa la dejiyay:".green(),
            n,
            l.latitude,
            l.longitude,
            l.timezone
        );
    }

    let any_set = theme.is_some()
        || bold_headers.is_some()
        || dim_separators.is_some()
        || latitude.is_some()
        || longitude.is_some()
        || timezone.is_some()
        || location_name.is_some();

    if !any_set {
        println!("{}", "Lama bixin options. Isticmaal: --theme, --bold-headers, --dim-separators, --latitude, --longitude, --timezone, --location-name".yellow());
        return;
    }

    match config.save() {
        Ok(()) => {
            println!("{}", "Qaabeynta waa la keydiyay.".green());
        }
        Err(e) => {
            eprintln!("{} {}", "Qalad keydinta config:".red(), e);
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
