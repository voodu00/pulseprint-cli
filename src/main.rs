use clap::{Parser, Subcommand};

/// PulsePrint-CLI: A tool for monitoring Bambu Labs printers via MQTT
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Placeholder for monitoring printers (expand with MQTT later)
    Monitor {
        /// Optional: Printer IP or hostname (for future use)
        #[arg(short, long)]
        printer: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Monitor { printer }) => {
            if let Some(p) = printer {
                println!("Monitoring printer at: {}", p);
            } else {
                println!("Monitoring default printer. (MQTT functionality coming soon!)");
            }
        }
        None => {
            println!("Welcome to PulsePrint-CLI! Use --help for usage.");
        }
    }
}