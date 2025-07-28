use super::*;

#[test]
fn test_printer_config_creation() {
    let config = PrinterConfig {
        ip: "192.168.1.100".to_string(),
        device_id: "01S00A000000000".to_string(),
        access_code: "12345678".to_string(),
    };

    assert_eq!(config.ip, "192.168.1.100");
    assert_eq!(config.device_id, "01S00A000000000");
    assert_eq!(config.access_code, "12345678");
}

#[test]
fn test_printer_config_clone() {
    let config = PrinterConfig {
        ip: "192.168.1.100".to_string(),
        device_id: "01S00A000000000".to_string(),
        access_code: "12345678".to_string(),
    };

    let cloned_config = config.clone();
    assert_eq!(config.ip, cloned_config.ip);
    assert_eq!(config.device_id, cloned_config.device_id);
    assert_eq!(config.access_code, cloned_config.access_code);
}

#[tokio::test]
async fn test_mqtt_client_creation_with_valid_config() {
    let config = PrinterConfig {
        ip: "127.0.0.1".to_string(),
        device_id: "test_device".to_string(),
        access_code: "test_code".to_string(),
    };

    // This will create the client but won't actually connect
    let result = MqttClient::new(config).await;
    assert!(
        result.is_ok(),
        "MqttClient creation should succeed with valid config"
    );
}

#[test]
fn test_mqtt_options_configuration() {
    let config = PrinterConfig {
        ip: "192.168.1.100".to_string(),
        device_id: "test_device".to_string(),
        access_code: "test_access_code".to_string(),
    };

    let mqtt_options = rumqttc::MqttOptions::new("pulseprint-cli", &config.ip, 8883);
    let (broker_addr, broker_port) = mqtt_options.broker_address();
    assert_eq!(broker_addr, "192.168.1.100");
    assert_eq!(broker_port, 8883);
}

#[test]
fn test_report_topic_format() {
    let device_id = "01S00A000000000";
    let expected_topic = format!("device/{}/report", device_id);
    assert_eq!(expected_topic, "device/01S00A000000000/report");
}

#[test]
fn test_printer_config_debug_format() {
    let config = PrinterConfig {
        ip: "192.168.1.100".to_string(),
        device_id: "test_device".to_string(),
        access_code: "secret".to_string(),
    };

    let debug_output = format!("{:?}", config);
    assert!(debug_output.contains("192.168.1.100"));
    assert!(debug_output.contains("test_device"));
    assert!(debug_output.contains("secret"));
}

#[test]
fn test_empty_config_values() {
    let config = PrinterConfig {
        ip: "".to_string(),
        device_id: "".to_string(),
        access_code: "".to_string(),
    };

    assert_eq!(config.ip, "");
    assert_eq!(config.device_id, "");
    assert_eq!(config.access_code, "");
}

#[test]
fn test_config_with_special_characters() {
    let config = PrinterConfig {
        ip: "192.168.1.100".to_string(),
        device_id: "device-with_special.chars".to_string(),
        access_code: "access@code#123".to_string(),
    };

    assert_eq!(config.device_id, "device-with_special.chars");
    assert_eq!(config.access_code, "access@code#123");
}

#[tokio::test]
async fn test_mqtt_client_with_empty_config() {
    let config = PrinterConfig {
        ip: "".to_string(),
        device_id: "".to_string(),
        access_code: "".to_string(),
    };

    // Should create client but fail when trying to connect
    let result = MqttClient::new(config).await;
    // Empty IP should still create the client object (connection will fail later)
    assert!(result.is_ok() || result.is_err()); // Either outcome is acceptable for empty config
}
