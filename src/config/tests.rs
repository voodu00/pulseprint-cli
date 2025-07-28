use super::*;
use tempfile::tempdir;

#[test]
fn test_printer_config_creation() {
    let config = PrinterConfig::new(
        "test_printer".to_string(),
        "192.168.1.100".to_string(),
        "device123".to_string(),
        "access123".to_string(),
    );

    assert_eq!(config.name, "test_printer");
    assert_eq!(config.ip, "192.168.1.100");
    assert_eq!(config.device_id, "device123");
    assert_eq!(config.access_code, "access123");
    assert_eq!(config.port, 8883);
    assert_eq!(config.use_tls, true);
    assert_eq!(config.model, None);
    assert_eq!(config.firmware_version, None);
}

#[test]
fn test_printer_config_topics() {
    let config = PrinterConfig::new(
        "test".to_string(),
        "192.168.1.100".to_string(),
        "device123".to_string(),
        "access123".to_string(),
    );

    assert_eq!(config.report_topic(), "device/device123/report");
    assert_eq!(config.request_topic(), "device/device123/request");
}

#[test]
fn test_printer_config_mqtt_url() {
    let mut config = PrinterConfig::new(
        "test".to_string(),
        "192.168.1.100".to_string(),
        "device123".to_string(),
        "access123".to_string(),
    );

    assert_eq!(config.mqtt_url(), "mqtts://192.168.1.100:8883");

    config.use_tls = false;
    assert_eq!(config.mqtt_url(), "mqtt://192.168.1.100:8883");

    config.port = 1883;
    assert_eq!(config.mqtt_url(), "mqtt://192.168.1.100:1883");
}

#[test]
fn test_app_config_default() {
    let config = AppConfig::default();
    assert!(config.printers.is_empty());
    assert_eq!(config.default_printer, None);
    assert_eq!(config.mqtt_settings.keep_alive_secs, 30);
    assert_eq!(config.mqtt_settings.retry_attempts, 5);
}

#[test]
fn test_app_config_add_remove_printer() {
    let mut app_config = AppConfig::default();
    let printer_config = PrinterConfig::new(
        "test_printer".to_string(),
        "192.168.1.100".to_string(),
        "device123".to_string(),
        "access123".to_string(),
    );

    // Add printer
    assert!(
        app_config
            .add_printer("test_printer".to_string(), printer_config.clone())
            .is_ok()
    );
    assert_eq!(app_config.printers.len(), 1);
    assert_eq!(app_config.default_printer, Some("test_printer".to_string()));

    // Try to add duplicate printer
    assert!(
        app_config
            .add_printer("test_printer".to_string(), printer_config.clone())
            .is_err()
    );

    // Remove printer
    let removed = app_config.remove_printer("test_printer").unwrap();
    assert_eq!(removed.name, "test_printer");
    assert!(app_config.printers.is_empty());
    assert_eq!(app_config.default_printer, None);
}

#[test]
fn test_config_file_operations() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("test_config.json");

    let mut app_config = AppConfig::default();
    let printer_config = PrinterConfig::new(
        "test_printer".to_string(),
        "192.168.1.100".to_string(),
        "device123".to_string(),
        "access123".to_string(),
    );

    app_config
        .add_printer("test_printer".to_string(), printer_config)
        .unwrap();

    // Save config
    assert!(app_config.save_to_file(&config_path).is_ok());
    assert!(config_path.exists());

    // Load config
    let loaded_config = AppConfig::load_from_file(&config_path).unwrap();
    assert_eq!(loaded_config.printers.len(), 1);
    assert!(loaded_config.printers.contains_key("test_printer"));
    assert_eq!(
        loaded_config.default_printer,
        Some("test_printer".to_string())
    );
}

#[test]
fn test_get_printer() {
    let mut app_config = AppConfig::default();
    let printer_config = PrinterConfig::new(
        "test_printer".to_string(),
        "192.168.1.100".to_string(),
        "device123".to_string(),
        "access123".to_string(),
    );

    app_config.add_printer("test_printer".to_string(), printer_config).unwrap();

    // Get existing printer
    let retrieved = app_config.get_printer("test_printer").unwrap();
    assert_eq!(retrieved.name, "test_printer");
    assert_eq!(retrieved.ip, "192.168.1.100");

    // Try to get non-existent printer
    assert!(app_config.get_printer("nonexistent").is_err());
}

#[test]
fn test_default_printer_operations() {
    let mut app_config = AppConfig::default();
    let printer1 = PrinterConfig::new(
        "printer1".to_string(),
        "192.168.1.100".to_string(),
        "device1".to_string(),
        "access1".to_string(),
    );
    let printer2 = PrinterConfig::new(
        "printer2".to_string(),
        "192.168.1.101".to_string(),
        "device2".to_string(),
        "access2".to_string(),
    );

    // No printers, should fail
    assert!(app_config.get_default_printer().is_err());

    // Add first printer
    app_config.add_printer("printer1".to_string(), printer1).unwrap();
    let default = app_config.get_default_printer().unwrap();
    assert_eq!(default.name, "printer1");

    // Add second printer
    app_config.add_printer("printer2".to_string(), printer2).unwrap();
    
    // Set new default
    assert!(app_config.set_default_printer("printer2").is_ok());
    let default = app_config.get_default_printer().unwrap();
    assert_eq!(default.name, "printer2");

    // Try to set non-existent printer as default
    assert!(app_config.set_default_printer("nonexistent").is_err());
}

#[test]
fn test_list_printers() {
    let mut app_config = AppConfig::default();
    
    // Empty list
    let printers = app_config.list_printers();
    assert!(printers.is_empty());

    // Add printers
    let printer1 = PrinterConfig::new(
        "printer1".to_string(),
        "192.168.1.100".to_string(),
        "device1".to_string(),
        "access1".to_string(),
    );
    let printer2 = PrinterConfig::new(
        "printer2".to_string(),
        "192.168.1.101".to_string(),
        "device2".to_string(),
        "access2".to_string(),
    );

    app_config.add_printer("printer1".to_string(), printer1).unwrap();
    app_config.add_printer("printer2".to_string(), printer2).unwrap();

    let printers = app_config.list_printers();
    assert_eq!(printers.len(), 2);
    
    let names: Vec<&String> = printers.iter().map(|(name, _)| *name).collect();
    assert!(names.contains(&&"printer1".to_string()));
    assert!(names.contains(&&"printer2".to_string()));
}

#[test]
fn test_config_path() {
    let path = AppConfig::get_config_path();
    assert!(path.to_string_lossy().contains("pulseprint-cli"));
    assert!(path.to_string_lossy().ends_with("config.json"));
}

#[test]
fn test_load_nonexistent_config() {
    let temp_dir = tempdir().unwrap();
    let nonexistent_path = temp_dir.path().join("nonexistent.json");
    
    // Should return default config for non-existent file
    let config = AppConfig::load_from_file(&nonexistent_path).unwrap();
    assert!(config.printers.is_empty());
}

#[test]
fn test_error_conditions() {
    let mut app_config = AppConfig::default();
    let printer_config = PrinterConfig::new(
        "test_printer".to_string(),
        "192.168.1.100".to_string(),
        "device123".to_string(),
        "access123".to_string(),
    );

    // Test PrinterExists error  
    app_config.add_printer("test_printer".to_string(), printer_config.clone()).unwrap();
    let err = app_config.add_printer("test_printer".to_string(), printer_config).unwrap_err();
    assert!(matches!(err, ConfigError::PrinterExists(_)));

    // Test PrinterNotFound error
    let err = app_config.remove_printer("nonexistent").unwrap_err();
    assert!(matches!(err, ConfigError::PrinterNotFound(_)));

    let err = app_config.get_printer("nonexistent").unwrap_err();
    assert!(matches!(err, ConfigError::PrinterNotFound(_)));

    let err = app_config.set_default_printer("nonexistent").unwrap_err();
    assert!(matches!(err, ConfigError::PrinterNotFound(_)));

    // Remove all printers and test NoDefaultPrinter
    app_config.remove_printer("test_printer").unwrap();
    let err = app_config.get_default_printer().unwrap_err();
    assert!(matches!(err, ConfigError::NoDefaultPrinter(_)));
}