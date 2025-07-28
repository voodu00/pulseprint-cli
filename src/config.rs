use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[cfg(test)]
mod tests {
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterConfig {
    pub name: String,
    pub ip: String,
    pub device_id: String,
    pub access_code: String,
    pub port: u16,
    pub use_tls: bool,
    pub model: Option<String>,
    pub firmware_version: Option<String>,
}

impl PrinterConfig {
    pub fn new(name: String, ip: String, device_id: String, access_code: String) -> Self {
        Self {
            name,
            ip,
            device_id,
            access_code,
            port: 8883,
            use_tls: true,
            model: None,
            firmware_version: None,
        }
    }

    pub fn mqtt_url(&self) -> String {
        let protocol = if self.use_tls { "mqtts" } else { "mqtt" };
        format!("{}://{}:{}", protocol, self.ip, self.port)
    }

    pub fn report_topic(&self) -> String {
        format!("device/{}/report", self.device_id)
    }

    pub fn request_topic(&self) -> String {
        format!("device/{}/request", self.device_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub printers: HashMap<String, PrinterConfig>,
    pub default_printer: Option<String>,
    pub mqtt_settings: MqttSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttSettings {
    pub keep_alive_secs: u64,
    pub connection_timeout_secs: u64,
    pub retry_attempts: u32,
    pub retry_delay_secs: u64,
    pub queue_size: usize,
}

impl Default for MqttSettings {
    fn default() -> Self {
        Self {
            keep_alive_secs: 30,
            connection_timeout_secs: 10,
            retry_attempts: 5,
            retry_delay_secs: 5,
            queue_size: 10,
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            printers: HashMap::new(),
            default_printer: None,
            mqtt_settings: MqttSettings::default(),
        }
    }
}

impl AppConfig {
    pub fn load_from_file(path: &PathBuf) -> Result<Self, ConfigError> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(path)
            .map_err(|e| ConfigError::IoError(format!("Failed to read config file: {}", e)))?;

        let config: AppConfig = serde_json::from_str(&contents)
            .map_err(|e| ConfigError::ParseError(format!("Failed to parse config: {}", e)))?;

        Ok(config)
    }

    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), ConfigError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                ConfigError::IoError(format!("Failed to create config directory: {}", e))
            })?;
        }

        let contents = serde_json::to_string_pretty(self).map_err(|e| {
            ConfigError::SerializeError(format!("Failed to serialize config: {}", e))
        })?;

        fs::write(path, contents)
            .map_err(|e| ConfigError::IoError(format!("Failed to write config file: {}", e)))?;

        Ok(())
    }

    pub fn add_printer(&mut self, name: String, printer: PrinterConfig) -> Result<(), ConfigError> {
        if self.printers.contains_key(&name) {
            return Err(ConfigError::PrinterExists(format!(
                "Printer '{}' already exists",
                name
            )));
        }

        self.printers.insert(name.clone(), printer);

        if self.default_printer.is_none() {
            self.default_printer = Some(name);
        }

        Ok(())
    }

    pub fn remove_printer(&mut self, name: &str) -> Result<PrinterConfig, ConfigError> {
        let printer = self
            .printers
            .remove(name)
            .ok_or_else(|| ConfigError::PrinterNotFound(format!("Printer '{}' not found", name)))?;

        if self.default_printer.as_ref() == Some(&name.to_string()) {
            self.default_printer = self.printers.keys().next().map(|k| k.clone());
        }

        Ok(printer)
    }

    pub fn get_printer(&self, name: &str) -> Result<&PrinterConfig, ConfigError> {
        self.printers
            .get(name)
            .ok_or_else(|| ConfigError::PrinterNotFound(format!("Printer '{}' not found", name)))
    }

    pub fn get_default_printer(&self) -> Result<&PrinterConfig, ConfigError> {
        let name = self.default_printer.as_ref().ok_or_else(|| {
            ConfigError::NoDefaultPrinter("No default printer configured".to_string())
        })?;

        self.get_printer(name)
    }

    pub fn set_default_printer(&mut self, name: &str) -> Result<(), ConfigError> {
        if !self.printers.contains_key(name) {
            return Err(ConfigError::PrinterNotFound(format!(
                "Printer '{}' not found",
                name
            )));
        }

        self.default_printer = Some(name.to_string());
        Ok(())
    }

    pub fn list_printers(&self) -> Vec<(&String, &PrinterConfig)> {
        self.printers.iter().collect()
    }

    pub fn get_config_path() -> PathBuf {
        if let Some(config_dir) = dirs::config_dir() {
            config_dir.join("pulseprint-cli").join("config.json")
        } else {
            PathBuf::from(".pulseprint-config.json")
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    IoError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Serialize error: {0}")]
    SerializeError(String),

    #[error("Printer already exists: {0}")]
    PrinterExists(String),

    #[error("Printer not found: {0}")]
    PrinterNotFound(String),

    #[error("No default printer: {0}")]
    NoDefaultPrinter(String),
}
