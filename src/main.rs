use clap::{Parser, Subcommand};
use chrono::Datelike;

mod commands;
mod calc;
mod models;
mod config;
mod tui;

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
    /// Manage configuration
    Config,
}

fn main() {
    println!("Ku soo dhawaaw Ramadan CLI 🌙");
    
    let cli = Cli::parse();

    match &cli.command {
        Commands::Next => {
            println!("Calculating next prayer time...");
        }
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
            
            println!("\nSalaada Maanta (Mogadishu):");
            println!("Taariikhda: {}", times.date);
            println!("-----------------------------");
            println!("Fajr:     {}", times.fajr);
            println!("Qorraxda: {}", times.sunrise);
            println!("Dhuhr:    {}", times.dhuhr);
            println!("Asr:      {}", times.asr);
            println!("Maghrib:  {}", times.maghrib);
            println!("Cishaha:  {}", times.isha);
            println!("-----------------------------");
        }
        Commands::Month => {
            println!("Calculating this month's prayer times...");
        }
        Commands::Config => {
            println!("Managing configuration...");
        }
    }
}
