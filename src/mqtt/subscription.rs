use crate::config::PrinterConfig;
use crate::messages::DeviceMessage;
use rumqttc::{AsyncClient, Event, EventLoop, Packet, QoS};
use std::error::Error;
use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};

#[derive(Debug)]
pub enum SubscriptionEvent {
    Message(Box<DeviceMessage>),
    Connected,
    Disconnected(String),
}

pub struct SubscriptionManager {
    client: AsyncClient,
    eventloop: EventLoop,
    config: PrinterConfig,
    event_sender: mpsc::Sender<SubscriptionEvent>,
}

impl SubscriptionManager {
    pub fn new(
        client: AsyncClient,
        eventloop: EventLoop,
        config: PrinterConfig,
        event_sender: mpsc::Sender<SubscriptionEvent>,
    ) -> Self {
        Self {
            client,
            eventloop,
            config,
            event_sender,
        }
    }

    pub async fn start_subscription(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let report_topic = self.config.report_topic();

        println!(
            "Connecting to printer '{}' at {} with device ID {}",
            self.config.name, self.config.ip, self.config.device_id
        );

        self.client
            .subscribe(&report_topic, QoS::AtMostOnce)
            .await?;

        println!(
            "Connected to printer '{}' at {} and subscribed to {}",
            self.config.name, self.config.ip, report_topic
        );
        println!("📡 Monitoring printer status - Press Ctrl+C to stop...");

        Ok(())
    }

    pub async fn run(mut self) {
        let reconnect_delay = Duration::from_secs(5);
        let max_reconnect_delay = Duration::from_secs(60);
        let mut current_delay = reconnect_delay;

        loop {
            match self.process_events().await {
                Ok(_) => {
                    // Connection closed normally
                    break;
                }
                Err(e) => {
                    let error_msg = format!("Connection error: {e}");
                    eprintln!("{error_msg}");

                    // Send disconnection event
                    let _ = self
                        .event_sender
                        .send(SubscriptionEvent::Disconnected(error_msg))
                        .await;

                    // Exponential backoff for reconnection
                    eprintln!("Reconnecting in {current_delay:?}...");
                    sleep(current_delay).await;

                    // Increase delay for next attempt, cap at max
                    current_delay = std::cmp::min(current_delay * 2, max_reconnect_delay);

                    // Try to resubscribe
                    match self.start_subscription().await {
                        Ok(_) => {
                            // Reset delay on successful reconnection
                            current_delay = reconnect_delay;
                            let _ = self.event_sender.send(SubscriptionEvent::Connected).await;
                        }
                        Err(e) => {
                            eprintln!("Failed to resubscribe: {e}");
                        }
                    }
                }
            }
        }
    }

    async fn process_events(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        loop {
            match self.eventloop.poll().await {
                Ok(notification) => {
                    if let Event::Incoming(Packet::Publish(publish)) = notification {
                        Self::handle_mqtt_message(&self.event_sender, publish).await?;
                    }
                }
                Err(e) => {
                    return Err(format!("MQTT event loop error: {e}").into());
                }
            }
        }
    }

    async fn handle_mqtt_message(
        event_sender: &mpsc::Sender<SubscriptionEvent>,
        publish: rumqttc::Publish,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let payload_str = std::str::from_utf8(&publish.payload)?;

        match DeviceMessage::parse(payload_str) {
            Ok(message) => {
                // Send parsed message through the channel
                event_sender
                    .send(SubscriptionEvent::Message(Box::new(message)))
                    .await
                    .map_err(|_| "Failed to send message to handler")?;
            }
            Err(e) => {
                eprintln!("Failed to parse MQTT message: {e}");
                if payload_str.len() < 1000 {
                    eprintln!("Raw message: {payload_str}");
                } else {
                    eprintln!("Raw message (truncated): {}...", &payload_str[..500]);
                }
            }
        }

        Ok(())
    }
}

pub struct MessageProcessor {
    event_receiver: mpsc::Receiver<SubscriptionEvent>,
}

impl MessageProcessor {
    pub fn new(event_receiver: mpsc::Receiver<SubscriptionEvent>) -> Self {
        Self { event_receiver }
    }

    pub async fn process_messages<F>(mut self, mut handler: F)
    where
        F: FnMut(SubscriptionEvent) -> Result<(), Box<dyn Error + Send + Sync>> + Send + 'static,
    {
        while let Some(event) = self.event_receiver.recv().await {
            if let Err(e) = handler(event) {
                eprintln!("Error processing event: {e}");
            }
        }
    }
}

#[cfg(test)]
#[path = "subscription_tests.rs"]
mod tests;
