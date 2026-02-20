use clap::{Parser, Subcommand};
use chrono::Datelike;

mod commands;
mod calc;
mod models;
mod config;
mod tui;

use config::settings::{AppConfig, style};

#[derive(Parser)]
#[command(name = "ramadan")]
#[command(about = "Qalab CLI Ramadan ah – wakhtiyada salaadda (offline)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Tus salaadda soo socota iyo wakhtiga ka hartay
    Next,
    /// Tus salaaddyada maanta
    Today,
    /// Tus salaaddyada bisha kan
    Month,
    /// Maar qaabeynta (theme, goobta, iwm.)
    Config {
        #[command(subcommand)]
        cmd: Option<ConfigCmd>,
    },
}

#[derive(Subcommand)]
enum ConfigCmd {
    /// Tus qaabeynta hadda
    Show,
    /// Deji theme, style, ama goobta
    Set {
        #[arg(long, value_enum)]
        theme: Option<config::settings::Theme>,
        #[arg(long)]
        bold_headers: Option<bool>,
        #[arg(long)]
        dim_separators: Option<bool>,
        #[arg(long)]
        latitude: Option<f64>,
        #[arg(long)]
        longitude: Option<f64>,
        #[arg(long)]
        timezone: Option<f64>,
        #[arg(long)]
        location_name: Option<String>,
    },
}

fn main() {
    let config = AppConfig::load();
    println!("{}", style::welcome(&config));

    let cli = Cli::parse();

    match &cli.command {
        Commands::Next => commands::next::run(),
        Commands::Today => {
            let cfg = AppConfig::load();
            let (location, location_name) = cfg.get_location();
            let now = chrono::Local::now();

            let times = calc::calculate_prayer_times(
                now.year(),
                now.month(),
                now.day(),
                &location,
                models::types::CalculationMethod::MuslimWorldLeague,
                models::types::Madhab::Shafi,
            );

            println!(
                "\n{}",
                style::title(&cfg, &format!("Salaada Maanta ({location_name}):"))
            );
            println!(
                "{} {}",
                style::label(&cfg, "Taariikhda:"),
                style::value(&cfg, &times.date)
            );
            println!("{}", style::separator(&cfg));
            println!("{}", style::prayer_line(&cfg, "Fajr    ", &times.fajr));
            println!("{}", style::prayer_line(&cfg, "Qorraxda", &times.sunrise));
            println!("{}", style::prayer_line(&cfg, "Dhuhr   ", &times.dhuhr));
            println!("{}", style::prayer_line(&cfg, "Asr     ", &times.asr));
            println!("{}", style::prayer_line(&cfg, "Maghrib ", &times.maghrib));
            println!("{}", style::prayer_line(&cfg, "Cishaha ", &times.isha));
            println!("{}", style::separator(&cfg));
        }
        Commands::Month => {
            println!("Bisha kan: salaaddyada waa la hisaabaynayaa...");
        }
        Commands::Config { cmd } => match cmd.as_ref().unwrap_or(&ConfigCmd::Show) {
            ConfigCmd::Show => commands::config::run_show(),
            ConfigCmd::Set {
                theme,
                bold_headers,
                dim_separators,
                latitude,
                longitude,
                timezone,
                location_name,
            } => commands::config::run_set(
                *theme,
                *bold_headers,
                *dim_separators,
                *latitude,
                *longitude,
                *timezone,
                location_name.clone(),
            ),
        },
    }
}
