use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

#[cfg(test)]
mod tests;

#[derive(Debug, Error)]
pub enum MessageError {
    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("Unknown message type: {0}")]
    #[allow(dead_code)]
    UnknownMessageType(String),

    #[error("Missing required field: {0}")]
    #[allow(dead_code)]
    MissingField(String),

    #[error("Invalid message format")]
    #[allow(dead_code)]
    InvalidFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceMessage {
    #[serde(rename = "print")]
    pub print: Option<PrintInfo>,

    #[serde(rename = "system")]
    pub system: Option<SystemInfo>,

    #[serde(rename = "info")]
    pub info: Option<DeviceInfo>,

    #[serde(rename = "pushing")]
    pub pushing: Option<PushingInfo>,

    #[serde(rename = "sequence_id")]
    pub sequence_id: Option<String>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintInfo {
    #[serde(rename = "command")]
    pub command: Option<String>,

    #[serde(rename = "msg")]
    pub msg: Option<u32>,

    #[serde(rename = "state")]
    pub state: Option<String>,

    #[serde(rename = "fail_reason")]
    pub fail_reason: Option<String>,

    #[serde(rename = "utc_time")]
    pub utc_time: Option<u64>,

    #[serde(rename = "gcode_state")]
    pub gcode_state: Option<String>,

    #[serde(rename = "percent")]
    pub percent: Option<u32>,

    #[serde(rename = "eta")]
    pub eta: Option<String>,

    #[serde(rename = "total_time")]
    pub total_time: Option<u32>,

    #[serde(rename = "remaining_time")]
    pub remaining_time: Option<u32>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    #[serde(rename = "command")]
    pub command: Option<String>,

    #[serde(rename = "msg")]
    pub msg: Option<u32>,

    #[serde(rename = "sequence_id")]
    pub sequence_id: Option<String>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    #[serde(rename = "command")]
    pub command: Option<String>,

    #[serde(rename = "sequence_id")]
    pub sequence_id: Option<String>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushingInfo {
    #[serde(rename = "command")]
    pub command: Option<String>,

    #[serde(rename = "version")]
    pub version: Option<u32>,

    #[serde(rename = "sequence_id")]
    pub sequence_id: Option<String>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub enum MessageType {
    PrintPushStatus,
    PushingPushAll,
    SystemPushAll,
    Unknown(String),
}

impl DeviceMessage {
    pub fn parse(json_data: &str) -> Result<Self, MessageError> {
        let message: DeviceMessage = serde_json::from_str(json_data)?;
        Ok(message)
    }

    pub fn get_message_type(&self) -> MessageType {
        if let Some(print) = &self.print {
            if print.command.as_deref() == Some("push_status") {
                return MessageType::PrintPushStatus;
            }
        }

        if let Some(pushing) = &self.pushing {
            if pushing.command.as_deref() == Some("pushall") {
                return MessageType::PushingPushAll;
            }
        }

        if let Some(system) = &self.system {
            if system.command.as_deref() == Some("pushall") {
                return MessageType::SystemPushAll;
            }
        }

        let command = self
            .print
            .as_ref()
            .and_then(|p| p.command.as_ref())
            .or_else(|| self.pushing.as_ref().and_then(|p| p.command.as_ref()))
            .or_else(|| self.system.as_ref().and_then(|s| s.command.as_ref()))
            .or_else(|| self.info.as_ref().and_then(|i| i.command.as_ref()));

        match command {
            Some(cmd) => MessageType::Unknown(cmd.clone()),
            None => MessageType::Unknown("no_command".to_string()),
        }
    }

    pub fn get_sequence_id(&self) -> Option<&str> {
        self.sequence_id
            .as_deref()
            .or_else(|| self.system.as_ref()?.sequence_id.as_deref())
            .or_else(|| self.info.as_ref()?.sequence_id.as_deref())
            .or_else(|| self.pushing.as_ref()?.sequence_id.as_deref())
    }
}

#[derive(Debug, Clone)]
pub struct PrinterStatus {
    pub state: PrintState,
    pub progress: Option<u32>,
    pub eta: Option<String>,
    pub remaining_time: Option<u32>,
    #[allow(dead_code)]
    pub total_time: Option<u32>,
    pub fail_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrintState {
    Idle,
    Printing,
    Paused,
    Failed,
    Finished,
    Unknown(String),
}

impl From<&str> for PrintState {
    fn from(state: &str) -> Self {
        match state {
            "idle" => PrintState::Idle,
            "printing" => PrintState::Printing,
            "paused" => PrintState::Paused,
            "failed" => PrintState::Failed,
            "finished" => PrintState::Finished,
            other => PrintState::Unknown(other.to_string()),
        }
    }
}

impl PrinterStatus {
    pub fn from_device_message(msg: &DeviceMessage) -> Option<Self> {
        let print = msg.print.as_ref()?;

        let state = print
            .state
            .as_deref()
            .map(PrintState::from)
            .unwrap_or(PrintState::Unknown("no_state".to_string()));

        Some(PrinterStatus {
            state,
            progress: print.percent,
            eta: print.eta.clone(),
            remaining_time: print.remaining_time,
            total_time: print.total_time,
            fail_reason: print.fail_reason.clone(),
        })
    }
}
