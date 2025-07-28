use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[cfg(test)]
mod tests;

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

    #[allow(dead_code)] // Will be used in future features
    pub fn mqtt_url(&self) -> String {
        let protocol = if self.use_tls { "mqtts" } else { "mqtt" };
        format!("{}://{}:{}", protocol, self.ip, self.port)
    }

    pub fn report_topic(&self) -> String {
        format!("device/{}/report", self.device_id)
    }

    #[allow(dead_code)] // Will be used in future features  
    pub fn request_topic(&self) -> String {
        format!("device/{}/request", self.device_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

impl AppConfig {
    pub fn load_from_file(path: &PathBuf) -> Result<Self, ConfigError> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(path)
            .map_err(|e| ConfigError::IoError(format!("Failed to read config file: {e}")))?;

        let config: AppConfig = serde_json::from_str(&contents)
            .map_err(|e| ConfigError::ParseError(format!("Failed to parse config: {e}")))?;

        Ok(config)
    }

    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), ConfigError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                ConfigError::IoError(format!("Failed to create config directory: {e}"))
            })?;
        }

        let contents = serde_json::to_string_pretty(self)
            .map_err(|e| ConfigError::SerializeError(format!("Failed to serialize config: {e}")))?;

        fs::write(path, contents)
            .map_err(|e| ConfigError::IoError(format!("Failed to write config file: {e}")))?;

        Ok(())
    }

    pub fn add_printer(&mut self, name: String, printer: PrinterConfig) -> Result<(), ConfigError> {
        if self.printers.contains_key(&name) {
            return Err(ConfigError::PrinterExists(format!(
                "Printer '{name}' already exists"
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
            .ok_or_else(|| ConfigError::PrinterNotFound(format!("Printer '{name}' not found")))?;

        if self.default_printer.as_ref() == Some(&name.to_string()) {
            self.default_printer = self.printers.keys().next().cloned();
        }

        Ok(printer)
    }

    #[allow(dead_code)] // Will be used in future features
    pub fn get_printer(&self, name: &str) -> Result<&PrinterConfig, ConfigError> {
        self.printers
            .get(name)
            .ok_or_else(|| ConfigError::PrinterNotFound(format!("Printer '{name}' not found")))
    }

    #[allow(dead_code)] // Will be used in future features
    pub fn get_default_printer(&self) -> Result<&PrinterConfig, ConfigError> {
        let name = self.default_printer.as_ref().ok_or_else(|| {
            ConfigError::NoDefaultPrinter("No default printer configured".to_string())
        })?;

        self.get_printer(name)
    }

    pub fn set_default_printer(&mut self, name: &str) -> Result<(), ConfigError> {
        if !self.printers.contains_key(name) {
            return Err(ConfigError::PrinterNotFound(format!(
                "Printer '{name}' not found"
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
    #[allow(dead_code)] // Will be used in future features
    NoDefaultPrinter(String),
}
