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
#[command(about = "A fast, offline-first Ramadan CLI tool for prayer times", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show the next prayer time
    Next,
    /// Show today's prayer times
    Today,
    /// Show the prayer times for the current month
    Month,
    /// Manage configuration (themes, bold headers, etc.)
    Config {
        #[command(subcommand)]
        cmd: Option<ConfigCmd>,
    },
}

#[derive(Subcommand)]
enum ConfigCmd {
    /// Show current configuration
    Show,
    /// Set theme or style options
    Set {
        #[arg(long, value_enum)]
        theme: Option<config::settings::Theme>,
        #[arg(long)]
        bold_headers: Option<bool>,
        #[arg(long)]
        dim_separators: Option<bool>,
    },
}

fn main() {
    let config = AppConfig::load();
    println!("{}", style::welcome(&config));

    let cli = Cli::parse();

    match &cli.command {
        Commands::Next => commands::next::run(),
        Commands::Today => {
            let now = chrono::Local::now();
            let location = models::types::Location {
                latitude: 2.0469,
                longitude: 45.3182,
                timezone: 3.0,
            };

            let times = calc::calculate_prayer_times(
                now.year(),
                now.month(),
                now.day(),
                &location,
                models::types::CalculationMethod::MuslimWorldLeague,
                models::types::Madhab::Shafi,
            );

            let cfg = AppConfig::load();
            println!(
                "\n{}",
                style::title(&cfg, "Salaada Maanta (Mogadishu):")
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
            println!("Calculating this month's prayer times...");
        }
        Commands::Config { cmd } => match cmd.as_ref().unwrap_or(&ConfigCmd::Show) {
            ConfigCmd::Show => commands::config::run_show(),
            ConfigCmd::Set {
                theme,
                bold_headers,
                dim_separators,
            } => commands::config::run_set(*theme, *bold_headers, *dim_separators),
        },
    }
}
