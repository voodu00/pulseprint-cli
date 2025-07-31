#[cfg(test)]
mod tests {
    use crate::messages::DeviceMessage;
    use crate::mqtt::subscription::{MessageProcessor, SubscriptionEvent};
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_subscription_event_creation() {
        // Test Message event
        let message = DeviceMessage {
            print: None,
            system: None,
            info: None,
            pushing: None,
            sequence_id: Some("test123".to_string()),
            extra: std::collections::HashMap::new(),
        };

        let event = SubscriptionEvent::Message(Box::new(message));
        match event {
            SubscriptionEvent::Message(msg) => {
                assert_eq!(msg.sequence_id, Some("test123".to_string()));
            }
            _ => panic!("Expected Message event"),
        }

        // Test Connected event
        let connected = SubscriptionEvent::Connected;
        matches!(connected, SubscriptionEvent::Connected);

        // Test Disconnected event
        let disconnected = SubscriptionEvent::Disconnected("Connection lost".to_string());
        match disconnected {
            SubscriptionEvent::Disconnected(reason) => {
                assert_eq!(reason, "Connection lost");
            }
            _ => panic!("Expected Disconnected event"),
        }
    }

    #[tokio::test]
    async fn test_message_processor() {
        let (sender, receiver) = mpsc::channel::<SubscriptionEvent>(10);
        let (result_sender, mut result_receiver) = mpsc::unbounded_channel::<String>();

        // Send test events
        let _ = sender.send(SubscriptionEvent::Connected).await;
        let _ = sender
            .send(SubscriptionEvent::Disconnected("Test".to_string()))
            .await;
        drop(sender); // Close the channel

        let processor = MessageProcessor::new(receiver);

        tokio::spawn(async move {
            processor
                .process_messages(move |event| {
                    match &event {
                        SubscriptionEvent::Connected => {
                            let _ = result_sender.send("connected".to_string());
                        }
                        SubscriptionEvent::Disconnected(_) => {
                            let _ = result_sender.send("disconnected".to_string());
                        }
                        SubscriptionEvent::Message(_) => {
                            let _ = result_sender.send("message".to_string());
                        }
                    }
                    Ok(())
                })
                .await;
        });

        let mut events_received = Vec::new();
        while let Some(event) = result_receiver.recv().await {
            events_received.push(event);
            if events_received.len() == 2 {
                break;
            }
        }

        assert_eq!(events_received, vec!["connected", "disconnected"]);
    }

    #[tokio::test]
    async fn test_message_processor_error_handling() {
        let (sender, receiver) = mpsc::channel::<SubscriptionEvent>(10);
        let (result_sender, mut result_receiver) = mpsc::unbounded_channel::<i32>();

        // Send test event
        let _ = sender.send(SubscriptionEvent::Connected).await;
        drop(sender);

        let processor = MessageProcessor::new(receiver);

        tokio::spawn(async move {
            processor
                .process_messages(move |_event| {
                    let _ = result_sender.send(1);
                    Err("Test error".into())
                })
                .await;
        });

        let mut error_count = 0;
        while let Some(_) = result_receiver.recv().await {
            error_count += 1;
        }

        assert_eq!(error_count, 1, "Handler should have been called once");
    }

    #[test]
    fn test_subscription_event_size() {
        // Verify that boxing the DeviceMessage reduces enum size
        let message_size = std::mem::size_of::<Box<DeviceMessage>>();
        let event_size = std::mem::size_of::<SubscriptionEvent>();

        // The event enum should be reasonably sized after boxing
        assert!(
            event_size < 100,
            "SubscriptionEvent size should be reasonable after boxing"
        );
        assert!(
            message_size == std::mem::size_of::<*const DeviceMessage>(),
            "Box should be pointer-sized"
        );
    }
}
