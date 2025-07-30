use clap::{Parser, Subcommand};

mod config;
mod messages;
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
        /// Printer name from config (or use default if not specified)
        #[arg(short, long)]
        name: Option<String>,

        /// Printer IP address (overrides config)
        #[arg(short = 'i', long)]
        ip: Option<String>,

        /// Device ID of the printer (overrides config)
        #[arg(short, long)]
        device_id: Option<String>,

        /// LAN access code for the printer (overrides config)
        #[arg(short, long)]
        access_code: Option<String>,
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
            name,
            ip,
            device_id,
            access_code,
        }) => {
            let printer_config = match load_printer_config(name, ip, device_id, access_code) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Error loading printer configuration: {e}");
                    std::process::exit(1);
                }
            };

            match monitor_printer(printer_config).await {
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

fn load_printer_config(
    name: &Option<String>,
    ip: &Option<String>,
    device_id: &Option<String>,
    access_code: &Option<String>,
) -> Result<config::PrinterConfig, Box<dyn std::error::Error>> {
    // If all manual parameters are provided, use them directly
    if let (Some(ip), Some(device_id), Some(access_code)) = (ip, device_id, access_code) {
        validate_ip_address(ip)?;
        validate_device_id(device_id)?;
        validate_access_code(access_code)?;

        return Ok(config::PrinterConfig::new(
            name.clone().unwrap_or_else(|| "manual".to_string()),
            ip.clone(),
            device_id.clone(),
            access_code.clone(),
        ));
    }

    // Otherwise, load from config
    let config_path = config::AppConfig::get_config_path();
    let app_config = config::AppConfig::load_from_file(&config_path)?;

    // Determine which printer to use
    let printer_config = match name {
        Some(printer_name) => {
            // Use specified printer
            app_config.get_printer(printer_name)?.clone()
        }
        None => {
            // Use default printer
            if app_config.printers.is_empty() {
                return Err("No printers configured. Use 'add' command to add a printer.".into());
            }
            app_config.get_default_printer()?.clone()
        }
    };

    // Apply any overrides
    let mut final_config = printer_config;
    if let Some(ip) = ip {
        validate_ip_address(ip)?;
        final_config.ip = ip.clone();
    }
    if let Some(device_id) = device_id {
        validate_device_id(device_id)?;
        final_config.device_id = device_id.clone();
    }
    if let Some(access_code) = access_code {
        validate_access_code(access_code)?;
        final_config.access_code = access_code.clone();
    }

    Ok(final_config)
}

async fn monitor_printer(
    config: config::PrinterConfig,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use mqtt::subscription::{MessageProcessor, SubscriptionEvent, SubscriptionManager};
    use tokio::sync::mpsc;

    // Create message channel with buffer for backpressure handling
    let (event_sender, event_receiver) = mpsc::channel::<SubscriptionEvent>(100);

    // Create MQTT client
    let mqtt_client = mqtt::MqttClient::new(config.clone()).await?;
    let (client, eventloop) = mqtt_client.into_parts();

    // Create subscription manager
    let mut subscription_manager =
        SubscriptionManager::new(client, eventloop, config, event_sender);

    // Start subscription
    subscription_manager.start_subscription().await?;

    // Create message processor
    let message_processor = MessageProcessor::new(event_receiver);

    // Spawn subscription task
    let subscription_handle = tokio::spawn(async move {
        subscription_manager.run().await;
    });

    // Process messages in main task
    let processor_handle = tokio::spawn(async move {
        message_processor
            .process_messages(handle_subscription_event)
            .await;
    });

    // Wait for either task to complete
    tokio::select! {
        _ = subscription_handle => {
            eprintln!("Subscription task ended");
        }
        _ = processor_handle => {
            eprintln!("Message processor ended");
        }
        _ = tokio::signal::ctrl_c() => {
            println!("\n🛑 Received interrupt signal, shutting down...");
        }
    }

    Ok(())
}

fn handle_subscription_event(
    event: mqtt::subscription::SubscriptionEvent,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use mqtt::subscription::SubscriptionEvent;

    match event {
        SubscriptionEvent::Message(message) => {
            process_device_message(*message);
        }
        SubscriptionEvent::Connected => {
            println!("✅ Connected to printer");
        }
        SubscriptionEvent::Disconnected(reason) => {
            eprintln!("❌ Disconnected: {reason}");
        }
    }

    Ok(())
}

fn process_device_message(message: messages::DeviceMessage) {
    let message_type = message.get_message_type();
    let sequence_id = message.get_sequence_id().unwrap_or("none");

    match message_type {
        messages::MessageType::PrintPushStatus => {
            if let Some(status) = messages::PrinterStatus::from_device_message(&message) {
                handle_print_status(status);
            }
            handle_bambu_print_status(&message);
        }
        messages::MessageType::PushingPushAll => {
            println!("📊 Received complete printer status (pushall)");
            handle_pushall_message(&message);
        }
        messages::MessageType::SystemPushAll => {
            println!("🔧 Received system information");
            handle_system_message(&message);
        }
        messages::MessageType::Unknown(cmd) => {
            println!("❓ Unknown message type: {cmd} (seq: {sequence_id})");
        }
    }
}

fn handle_print_status(status: messages::PrinterStatus) {
    use messages::PrintState;

    let state_icon = match &status.state {
        PrintState::Idle => "💤",
        PrintState::Printing => "🖨️",
        PrintState::Paused => "⏸️",
        PrintState::Failed => "❌",
        PrintState::Finished => "✅",
        PrintState::Unknown(_) => "❓",
    };

    print!("{state_icon} Print Status: {:?}", status.state);

    if let Some(progress) = status.progress {
        print!(" - Progress: {progress}%");
    }

    if let Some(eta) = &status.eta {
        print!(" - ETA: {eta}");
    }

    if let Some(remaining) = status.remaining_time {
        let hours = remaining / 3600;
        let minutes = (remaining % 3600) / 60;
        let seconds = remaining % 60;
        if hours > 0 {
            print!(" - Remaining: {hours}h {minutes}m {seconds}s");
        } else {
            print!(" - Remaining: {minutes}m {seconds}s");
        }
    }

    if let Some(reason) = &status.fail_reason {
        print!(" - Failure: {reason}");
    }

    println!();
}

// Enhanced function to show actual printer data from messages
fn handle_bambu_print_status(message: &messages::DeviceMessage) {
    if let Some(print_info) = &message.print {
        let mut info_parts = Vec::new();

        // Temperature info
        if let Some(nozzle_temp) = print_info.nozzle_temper {
            info_parts.push(format!("🌡️ Nozzle: {nozzle_temp:.1}°C"));
        }

        if let Some(bed_temp) = print_info.bed_temper {
            info_parts.push(format!("🛏️ Bed: {bed_temp:.1}°C"));
        }

        // Print progress info
        if let Some(layer) = print_info.layer_num {
            info_parts.push(format!("📄 Layer: {layer}"));
        }

        if let Some(remaining) = print_info.mc_remaining_time {
            let hours = remaining / 3600;
            let minutes = (remaining % 3600) / 60;
            if hours > 0 {
                info_parts.push(format!("⏱️ Remaining: {hours}h {minutes}m"));
            } else {
                info_parts.push(format!("⏱️ Remaining: {minutes}m"));
            }
        }

        if let Some(wifi) = &print_info.wifi_signal {
            info_parts.push(format!("📶 WiFi: {wifi}"));
        }

        if !info_parts.is_empty() {
            println!("🖨️ Printer Status: {}", info_parts.join(" | "));
        }
    }
}

fn handle_pushall_message(message: &messages::DeviceMessage) {
    // Extract and display comprehensive printer information
    if let Some(print_info) = &message.print {
        if let Some(state) = &print_info.state {
            println!("  Print State: {state}");
        }
        if let Some(percent) = print_info.percent {
            println!("  Progress: {percent}%");
        }
    }

    // Display printer model and other device info from extra fields
    display_device_info(&message.extra);
}

fn handle_system_message(message: &messages::DeviceMessage) {
    // Display system information and device details
    if let Some(system_info) = &message.system {
        println!("  System Command: {:?}", system_info.command);
    }

    // Display device information from extra fields
    display_device_info(&message.extra);
}

fn display_device_info(extra_fields: &std::collections::HashMap<String, serde_json::Value>) {
    // Look for common device information fields
    if let Some(model) = extra_fields.get("model") {
        if let Some(model_str) = model.as_str() {
            println!("  🖨️  Model: {model_str}");
        }
    }

    if let Some(sn) = extra_fields.get("sn") {
        if let Some(sn_str) = sn.as_str() {
            println!("  🏷️  Serial Number: {sn_str}");
        }
    }

    if let Some(firmware) = extra_fields.get("ota") {
        if let Some(firmware_obj) = firmware.as_object() {
            if let Some(version) = firmware_obj.get("version") {
                if let Some(version_str) = version.as_str() {
                    println!("  📦 Firmware: {version_str}");
                }
            }
        }
    }

    if let Some(wifi) = extra_fields.get("wifi") {
        if let Some(wifi_obj) = wifi.as_object() {
            if let Some(ssid) = wifi_obj.get("ssid") {
                if let Some(ssid_str) = ssid.as_str() {
                    println!("  📶 WiFi: {ssid_str}");
                }
            }
        }
    }

    // Display temperature information if available
    if let Some(temp) = extra_fields.get("temp") {
        if let Some(temp_obj) = temp.as_object() {
            if let Some(bed_temp) = temp_obj.get("bed_temp") {
                if let Some(bed_current) = bed_temp.as_f64() {
                    println!("  🌡️  Bed Temperature: {bed_current}°C");
                }
            }
            if let Some(nozzle_temp) = temp_obj.get("nozzle_temp") {
                if let Some(nozzle_current) = nozzle_temp.as_f64() {
                    println!("  🌡️  Nozzle Temperature: {nozzle_current}°C");
                }
            }
        }
    }
}
