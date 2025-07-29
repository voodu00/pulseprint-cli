use super::*;

#[test]
fn test_parse_print_push_status_message() {
    let json_data = r#"{
        "print": {
            "command": "push_status",
            "msg": 1,
            "state": "printing",
            "percent": 45,
            "eta": "15:30",
            "remaining_time": 1800,
            "total_time": 3600
        },
        "sequence_id": "12345"
    }"#;

    let message = DeviceMessage::parse(json_data).unwrap();

    assert!(message.print.is_some());
    let print = message.print.unwrap();
    assert_eq!(print.command, Some("push_status".to_string()));
    assert_eq!(print.state, Some("printing".to_string()));
    assert_eq!(print.percent, Some(45));
    assert_eq!(message.sequence_id, Some("12345".to_string()));

    // Check message type detection
    let message = DeviceMessage::parse(json_data).unwrap();
    match message.get_message_type() {
        MessageType::PrintPushStatus => (),
        _ => panic!("Expected PrintPushStatus message type"),
    }
}

#[test]
fn test_parse_pushing_pushall_message() {
    let json_data = r#"{
        "pushing": {
            "command": "pushall",
            "version": 1,
            "sequence_id": "98765"
        }
    }"#;

    let message = DeviceMessage::parse(json_data).unwrap();

    assert!(message.pushing.is_some());
    let pushing = message.pushing.unwrap();
    assert_eq!(pushing.command, Some("pushall".to_string()));
    assert_eq!(pushing.version, Some(1));

    // Check message type detection
    let message = DeviceMessage::parse(json_data).unwrap();
    match message.get_message_type() {
        MessageType::PushingPushAll => (),
        _ => panic!("Expected PushingPushAll message type"),
    }
}

#[test]
fn test_parse_system_pushall_message() {
    let json_data = r#"{
        "system": {
            "command": "pushall",
            "msg": 2,
            "sequence_id": "11111"
        }
    }"#;

    let message = DeviceMessage::parse(json_data).unwrap();

    assert!(message.system.is_some());
    let system = message.system.unwrap();
    assert_eq!(system.command, Some("pushall".to_string()));
    assert_eq!(system.msg, Some(2));
    assert_eq!(system.sequence_id, Some("11111".to_string()));

    // Check message type detection
    let message = DeviceMessage::parse(json_data).unwrap();
    match message.get_message_type() {
        MessageType::SystemPushAll => (),
        _ => panic!("Expected SystemPushAll message type"),
    }
}

#[test]
fn test_parse_message_with_extra_fields() {
    let json_data = r#"{
        "print": {
            "command": "push_status",
            "state": "idle",
            "custom_field": "value",
            "another_field": 123
        },
        "extra_top_level": true,
        "sequence_id": "54321"
    }"#;

    let message = DeviceMessage::parse(json_data).unwrap();

    assert!(message.print.is_some());
    assert!(message.extra.contains_key("extra_top_level"));
    assert_eq!(
        message.extra.get("extra_top_level"),
        Some(&serde_json::json!(true))
    );
}

#[test]
fn test_printer_status_from_message() {
    let json_data = r#"{
        "print": {
            "command": "push_status",
            "state": "printing",
            "percent": 67,
            "eta": "12:45",
            "remaining_time": 900,
            "total_time": 2700
        }
    }"#;

    let message = DeviceMessage::parse(json_data).unwrap();
    let status = PrinterStatus::from_device_message(&message).unwrap();

    assert_eq!(status.state, PrintState::Printing);
    assert_eq!(status.progress, Some(67));
    assert_eq!(status.eta, Some("12:45".to_string()));
    assert_eq!(status.remaining_time, Some(900));
    assert_eq!(status.total_time, Some(2700));
    assert_eq!(status.fail_reason, None);
}

#[test]
fn test_printer_status_failed_state() {
    let json_data = r#"{
        "print": {
            "command": "push_status",
            "state": "failed",
            "fail_reason": "Filament runout detected"
        }
    }"#;

    let message = DeviceMessage::parse(json_data).unwrap();
    let status = PrinterStatus::from_device_message(&message).unwrap();

    assert_eq!(status.state, PrintState::Failed);
    assert_eq!(
        status.fail_reason,
        Some("Filament runout detected".to_string())
    );
}

#[test]
fn test_print_state_from_str() {
    assert_eq!(PrintState::from("idle"), PrintState::Idle);
    assert_eq!(PrintState::from("printing"), PrintState::Printing);
    assert_eq!(PrintState::from("paused"), PrintState::Paused);
    assert_eq!(PrintState::from("failed"), PrintState::Failed);
    assert_eq!(PrintState::from("finished"), PrintState::Finished);
    assert_eq!(
        PrintState::from("custom_state"),
        PrintState::Unknown("custom_state".to_string())
    );
}

#[test]
fn test_get_sequence_id() {
    // From top level
    let json_data = r#"{"sequence_id": "top123"}"#;
    let message = DeviceMessage::parse(json_data).unwrap();
    assert_eq!(message.get_sequence_id(), Some("top123"));

    // From system
    let json_data = r#"{"system": {"sequence_id": "sys456"}}"#;
    let message = DeviceMessage::parse(json_data).unwrap();
    assert_eq!(message.get_sequence_id(), Some("sys456"));

    // From pushing
    let json_data = r#"{"pushing": {"sequence_id": "push789"}}"#;
    let message = DeviceMessage::parse(json_data).unwrap();
    assert_eq!(message.get_sequence_id(), Some("push789"));
}

#[test]
fn test_invalid_json() {
    let json_data = "not valid json";
    let result = DeviceMessage::parse(json_data);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        MessageError::JsonParseError(_)
    ));
}

#[test]
fn test_empty_message() {
    let json_data = "{}";
    let message = DeviceMessage::parse(json_data).unwrap();

    assert!(message.print.is_none());
    assert!(message.system.is_none());
    assert!(message.info.is_none());
    assert!(message.pushing.is_none());
    assert!(message.sequence_id.is_none());

    match message.get_message_type() {
        MessageType::Unknown(cmd) => assert_eq!(cmd, "no_command"),
        _ => panic!("Expected Unknown message type"),
    }
}

#[test]
fn test_unknown_command() {
    let json_data = r#"{
        "print": {
            "command": "custom_command"
        }
    }"#;

    let message = DeviceMessage::parse(json_data).unwrap();

    match message.get_message_type() {
        MessageType::Unknown(cmd) => assert_eq!(cmd, "custom_command"),
        _ => panic!("Expected Unknown message type"),
    }
}
