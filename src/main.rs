use clap::{Parser, Subcommand};

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
            let config = mqtt::PrinterConfig {
                ip: printer.clone(),
                device_id: device_id.clone(),
                access_code: access_code.clone(),
            };

            match monitor_printer(config).await {
                Ok(_) => println!("Monitoring completed successfully"),
                Err(e) => eprintln!("Error monitoring printer: {}", e),
            }
        }
        None => {
            println!("Welcome to PulsePrint-CLI! Use --help for usage.");
        }
    }
}

async fn monitor_printer(config: mqtt::PrinterConfig) -> Result<(), Box<dyn std::error::Error>> {
    const MAX_RETRIES: u32 = 5;
    const RETRY_DELAY_SECS: u64 = 5;

    let mut retry_count = 0;

    loop {
        println!(
            "Connecting to printer at {} with device ID {} (attempt {}/{})",
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
                eprintln!("Connection attempt failed: {}", e);

                retry_count += 1;
                if retry_count > MAX_RETRIES {
                    return Err(
                        format!("Failed to connect after {} attempts", MAX_RETRIES + 1).into(),
                    );
                }

                println!("Retrying in {} seconds...", RETRY_DELAY_SECS);
                tokio::time::sleep(tokio::time::Duration::from_secs(RETRY_DELAY_SECS)).await;
            }
        }
    }
}

async fn attempt_connection(
    config: &mqtt::PrinterConfig,
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
                        println!("Received: {:?}", packet);
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
                return Err(format!("MQTT connection error: {}", e).into());
            }
        }
    }
}
