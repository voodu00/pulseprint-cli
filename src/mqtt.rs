use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS, TlsConfiguration, Transport};
use std::error::Error;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct PrinterConfig {
    pub ip: String,
    pub device_id: String,
    pub access_code: String,
}

pub struct MqttClient {
    client: AsyncClient,
    eventloop: EventLoop,
    config: PrinterConfig,
}

impl MqttClient {
    pub async fn new(config: PrinterConfig) -> Result<Self, Box<dyn Error>> {
        let mut mqtt_options = MqttOptions::new("pulseprint-cli", &config.ip, 8883);

        // Set authentication
        mqtt_options.set_credentials("bblp", &config.access_code);

        // Configure TLS
        let tls_config = TlsConfiguration::Simple {
            ca: vec![],
            alpn: None,
            client_auth: None,
        };
        mqtt_options.set_transport(Transport::Tls(tls_config));

        // Set connection parameters
        mqtt_options.set_keep_alive(Duration::from_secs(30));

        let (client, eventloop) = AsyncClient::new(mqtt_options, 10);

        Ok(MqttClient {
            client,
            eventloop,
            config,
        })
    }

    pub async fn connect(&self) -> Result<(), Box<dyn Error>> {
        // Subscribe to the device report topic
        let report_topic = format!("device/{}/report", self.config.device_id);
        self.client
            .subscribe(&report_topic, QoS::AtMostOnce)
            .await?;

        println!(
            "Connected to printer at {} and subscribed to {}",
            self.config.ip, report_topic
        );

        Ok(())
    }

    pub fn get_eventloop(self) -> EventLoop {
        self.eventloop
    }

}
