use clap::{Parser, Subcommand};

mod config;
mod mqtt;

/// PulsePrint-CLI: A tool for monitoring Bambu Labs printers via MQTT
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Monitor a Bambu Labs printer via MQTT
    Monitor {
        /// Printer IP address
        #[arg(short, long)]
        printer: String,

        /// Device ID of the printer
        #[arg(short, long)]
        device_id: String,

        /// LAN access code for the printer
        #[arg(short, long)]
        access_code: String,
    },
    /// Add a new printer configuration
    Add {
        /// Printer name (unique identifier)
        #[arg(short, long)]
        name: String,

        /// Printer IP address
        #[arg(short = 'i', long)]
        ip: String,

        /// Device ID of the printer
        #[arg(short, long)]
        device_id: String,

        /// LAN access code for the printer
        #[arg(short, long)]
        access_code: String,

        /// Set as default printer
        #[arg(long)]
        set_default: bool,
    },
    /// List all configured printers
    List,
    /// Remove a printer configuration
    Remove {
        /// Name of the printer to remove
        name: String,
    },
    /// Set the default printer
    SetDefault {
        /// Name of the printer to set as default
        name: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Monitor {
            printer,
            device_id,
            access_code,
        }) => {
            let config = config::PrinterConfig::new(
                "temp".to_string(),
                printer.clone(),
                device_id.clone(),
                access_code.clone(),
            );

            match monitor_printer(config).await {
                Ok(_) => println!("Monitoring completed successfully"),
                Err(e) => eprintln!("Error monitoring printer: {e}"),
            }
        }
        Some(Commands::Add {
            name,
            ip,
            device_id,
            access_code,
            set_default,
        }) => {
            if let Err(e) = handle_add_printer(name, ip, device_id, access_code, *set_default) {
                eprintln!("Error adding printer: {e}");
                std::process::exit(1);
            }
        }
        Some(Commands::List) => {
            if let Err(e) = handle_list_printers() {
                eprintln!("Error listing printers: {e}");
                std::process::exit(1);
            }
        }
        Some(Commands::Remove { name }) => {
            if let Err(e) = handle_remove_printer(name) {
                eprintln!("Error removing printer: {e}");
                std::process::exit(1);
            }
        }
        Some(Commands::SetDefault { name }) => {
            if let Err(e) = handle_set_default_printer(name) {
                eprintln!("Error setting default printer: {e}");
                std::process::exit(1);
            }
        }
        None => {
            println!("Welcome to PulsePrint-CLI! Use --help for usage.");
        }
    }
}

fn handle_add_printer(
    name: &str,
    ip: &str,
    device_id: &str,
    access_code: &str,
    set_default: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Validate inputs
    validate_ip_address(ip)?;
    validate_device_id(device_id)?;
    validate_access_code(access_code)?;

    // Load existing config
    let config_path = config::AppConfig::get_config_path();
    let mut app_config = config::AppConfig::load_from_file(&config_path)?;

    // Create printer config
    let printer_config = config::PrinterConfig::new(
        name.to_string(),
        ip.to_string(),
        device_id.to_string(),
        access_code.to_string(),
    );

    // Add printer
    app_config.add_printer(name.to_string(), printer_config)?;

    // Set as default if requested
    if set_default {
        app_config.set_default_printer(name)?;
    }

    // Save config
    app_config.save_to_file(&config_path)?;

    println!("✅ Printer '{name}' added successfully");
    if set_default {
        println!("🎯 Set as default printer");
    }

    Ok(())
}

fn handle_list_printers() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = config::AppConfig::get_config_path();
    let app_config = config::AppConfig::load_from_file(&config_path)?;

    if app_config.printers.is_empty() {
        println!("No printers configured. Use 'add' command to add a printer.");
        return Ok(());
    }

    println!("Configured Printers:");
    println!("==================");

    let default_name = app_config.default_printer.as_ref();
    let mut printers: Vec<_> = app_config.list_printers();
    printers.sort_by_key(|(name, _)| name.as_str());

    for (name, printer) in printers {
        let default_marker = if Some(name) == default_name {
            " (default)"
        } else {
            ""
        };

        println!("📄 {name}{default_marker}");
        println!("   IP: {}", printer.ip);
        println!("   Device ID: {}", printer.device_id);
        println!(
            "   Port: {} (TLS: {})",
            printer.port,
            if printer.use_tls {
                "enabled"
            } else {
                "disabled"
            }
        );
        if let Some(model) = &printer.model {
            println!("   Model: {model}");
        }
        println!();
    }

    Ok(())
}

fn handle_remove_printer(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = config::AppConfig::get_config_path();
    let mut app_config = config::AppConfig::load_from_file(&config_path)?;

    // Remove printer
    let removed_printer = app_config.remove_printer(name)?;

    // Save config
    app_config.save_to_file(&config_path)?;

    let removed_name = &removed_printer.name;
    println!("🗑️  Printer '{removed_name}' removed successfully");

    // Show message if this was the default printer
    if app_config.default_printer.is_none() && !app_config.printers.is_empty() {
        let first_printer = app_config.printers.keys().next().unwrap();
        println!("💡 Consider setting a new default printer with: set-default {first_printer}");
    }

    Ok(())
}

fn handle_set_default_printer(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = config::AppConfig::get_config_path();
    let mut app_config = config::AppConfig::load_from_file(&config_path)?;

    // Set default printer
    app_config.set_default_printer(name)?;

    // Save config
    app_config.save_to_file(&config_path)?;

    println!("🎯 Printer '{name}' set as default");

    Ok(())
}

fn validate_ip_address(ip: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::net::Ipv4Addr;

    ip.parse::<Ipv4Addr>()
        .map_err(|_| format!("Invalid IP address: {ip}"))?;

    Ok(())
}

fn validate_device_id(device_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    if device_id.is_empty() {
        return Err("Device ID cannot be empty".into());
    }

    if device_id.len() < 5 {
        return Err("Device ID seems too short (should be like '01S00A000000000')".into());
    }

    Ok(())
}

fn validate_access_code(access_code: &str) -> Result<(), Box<dyn std::error::Error>> {
    if access_code.is_empty() {
        return Err("Access code cannot be empty".into());
    }

    if access_code.len() != 8 {
        return Err("Access code should be exactly 8 characters".into());
    }

    // Check if it's all digits
    if !access_code.chars().all(|c| c.is_ascii_digit()) {
        return Err("Access code should contain only digits".into());
    }

    Ok(())
}

async fn monitor_printer(config: config::PrinterConfig) -> Result<(), Box<dyn std::error::Error>> {
    const MAX_RETRIES: u32 = 5;
    const RETRY_DELAY_SECS: u64 = 5;

    let mut retry_count = 0;

    loop {
        println!(
            "Connecting to printer '{}' at {} with device ID {} (attempt {}/{})",
            config.name,
            config.ip,
            config.device_id,
            retry_count + 1,
            MAX_RETRIES + 1
        );

        match attempt_connection(&config).await {
            Ok(_) => {
                println!("Connection successful! Monitoring stopped.");
                return Ok(());
            }
            Err(e) => {
                eprintln!("Connection attempt failed: {e}");

                retry_count += 1;
                if retry_count > MAX_RETRIES {
                    return Err(
                        format!("Failed to connect after {} attempts", MAX_RETRIES + 1).into(),
                    );
                }

                println!("Retrying in {RETRY_DELAY_SECS} seconds...");
                tokio::time::sleep(tokio::time::Duration::from_secs(RETRY_DELAY_SECS)).await;
            }
        }
    }
}

async fn attempt_connection(
    config: &config::PrinterConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let mqtt_client = mqtt::MqttClient::new(config.clone()).await?;
    mqtt_client.connect().await?;

    let mut eventloop = mqtt_client.get_eventloop();

    loop {
        match eventloop.poll().await {
            Ok(notification) => {
                use rumqttc::Event;
                match notification {
                    Event::Incoming(packet) => {
                        println!("Received: {packet:?}");
                    }
                    Event::Outgoing(packet) => {
                        // Less verbose for outgoing packets
                        if let rumqttc::Outgoing::Publish(_) = packet {
                            // Only log actual publishes, not pings
                        }
                    }
                }
            }
            Err(e) => {
                return Err(format!("MQTT connection error: {e}").into());
            }
        }
    }
}
