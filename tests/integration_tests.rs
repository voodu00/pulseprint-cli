use std::process::Command;

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