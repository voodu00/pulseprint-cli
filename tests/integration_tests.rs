use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_cli_help_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("PulsePrint-CLI"));
    assert!(stdout.contains("monitor"));
}

#[test]
fn test_cli_version_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("pulseprint-cli"));
}

#[test]
fn test_monitor_help_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "monitor", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("Monitor a Bambu Labs printer via MQTT"));
    assert!(stdout.contains("--printer"));
    assert!(stdout.contains("--device-id"));
    assert!(stdout.contains("--access-code"));
}

#[test]
fn test_monitor_missing_arguments() {
    let output = Command::new("cargo")
        .args(&["run", "--", "monitor"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");
    assert!(stderr.contains("required arguments were not provided"));
}

// Note: Skipping actual connection test to avoid hanging
// In a real environment, you would test with a mock MQTT broker

#[test]
fn test_cli_help_shows_new_commands() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("add"));
    assert!(stdout.contains("list"));
    assert!(stdout.contains("remove"));
    assert!(stdout.contains("set-default"));
}

#[test]
fn test_add_command_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "add", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("Add a new printer configuration"));
    assert!(stdout.contains("--name"));
    assert!(stdout.contains("--ip"));
    assert!(stdout.contains("--device-id"));
    assert!(stdout.contains("--access-code"));
    assert!(stdout.contains("--set-default"));
}

#[test]
fn test_list_command_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "list", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("List all configured printers"));
}

#[test]
fn test_remove_command_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "remove", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("Remove a printer configuration"));
}

#[test]
fn test_set_default_command_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "set-default", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("Set the default printer"));
}

#[test]
fn test_list_empty_printers() {
    // Set a temporary config directory to isolate this test
    let temp_dir = tempdir().expect("Failed to create temp dir");

    let output = Command::new("cargo")
        .args(&["run", "--", "list"])
        .env("HOME", temp_dir.path()) // This won't work cross-platform, but good for basic testing
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("No printers configured"));
}

#[test]
fn test_add_command_validation_invalid_ip() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "add",
            "--name",
            "test",
            "--ip",
            "999.999.999.999",
            "--device-id",
            "01S00A000000000",
            "--access-code",
            "12345678",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");
    assert!(stderr.contains("Invalid IP address"));
}

#[test]
fn test_add_command_validation_invalid_access_code() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "add",
            "--name",
            "test",
            "--ip",
            "192.168.1.100",
            "--device-id",
            "01S00A000000000",
            "--access-code",
            "123",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");
    assert!(stderr.contains("Access code should be exactly 8 characters"));
}

#[test]
fn test_add_command_missing_arguments() {
    let output = Command::new("cargo")
        .args(&["run", "--", "add", "--name", "test"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");
    assert!(stderr.contains("required arguments were not provided"));
}

#[test]
fn test_remove_nonexistent_printer() {
    let output = Command::new("cargo")
        .args(&["run", "--", "remove", "nonexistent"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");
    assert!(stderr.contains("Printer not found"));
}

#[test]
fn test_set_default_nonexistent_printer() {
    let output = Command::new("cargo")
        .args(&["run", "--", "set-default", "nonexistent"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");
    assert!(stderr.contains("Printer not found"));
}
