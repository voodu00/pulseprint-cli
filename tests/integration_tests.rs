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
    assert!(stdout.contains("--name"));
    assert!(stdout.contains("--ip"));
    assert!(stdout.contains("--device-id"));
    assert!(stdout.contains("--access-code"));
}

#[test]
fn test_monitor_no_config() {
    use std::time::Duration;
    use std::io::{BufRead, BufReader};
    
    // Create a temporary empty config directory to ensure no printers are configured
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    
    // Spawn the monitor command with a custom config directory
    let mut child = Command::new("cargo")
        .args(&["run", "--", "monitor"])
        .env("PULSEPRINT_TEST_CONFIG_DIR", temp_dir.path())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    // Read stderr to check for the expected error message
    let stderr = child.stderr.take().expect("Failed to get stderr");
    let reader = BufReader::new(stderr);
    
    let mut found_error = false;
    let start = std::time::Instant::now();
    
    for line in reader.lines() {
        if start.elapsed() > Duration::from_secs(5) {
            break; // Timeout after 5 seconds
        }
        
        if let Ok(line) = line {
            if line.contains("No printers configured") || 
               line.contains("Error loading printer configuration") {
                found_error = true;
                break;
            }
        }
    }
    
    // Kill the process if it's still running
    let _ = child.kill();
    let _ = child.wait();
    
    assert!(found_error, "Expected 'No printers configured' error message");
}

#[test]
fn test_monitor_with_direct_params() {
    use std::time::Duration;
    use std::io::{BufRead, BufReader};
    
    // Create a temporary empty config directory
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    
    // Spawn monitor with direct parameters (will fail to connect but that's expected)
    let mut child = Command::new("cargo")
        .args(&[
            "run", "--", "monitor",
            "--ip", "192.168.1.100",
            "--device-id", "01S00A000000000", 
            "--access-code", "12345678"
        ])
        .env("PULSEPRINT_TEST_CONFIG_DIR", temp_dir.path())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    // Read both stdout and stderr to check for connection attempt
    let stdout = child.stdout.take().expect("Failed to get stdout");
    let stderr = child.stderr.take().expect("Failed to get stderr");
    
    let stdout_reader = BufReader::new(stdout);
    let stderr_reader = BufReader::new(stderr);
    
    let mut found_connection_attempt = false;
    let start = std::time::Instant::now();
    
    // Check stdout
    for line in stdout_reader.lines() {
        if start.elapsed() > Duration::from_secs(5) {
            break; // Timeout after 5 seconds
        }
        
        if let Ok(line) = line {
            if line.contains("Connecting to printer") && 
               line.contains("192.168.1.100") &&
               line.contains("01S00A000000000") {
                found_connection_attempt = true;
                break;
            }
        }
    }
    
    // If not found in stdout, check stderr (in case of early errors)
    if !found_connection_attempt {
        for line in stderr_reader.lines() {
            if start.elapsed() > Duration::from_secs(5) {
                break;
            }
            
            if let Ok(line) = line {
                // Also accept connection error messages as proof the connection was attempted
                if (line.contains("Connecting to printer") || 
                    line.contains("connection error") ||
                    line.contains("192.168.1.100")) &&
                   line.contains("01S00A000000000") {
                    found_connection_attempt = true;
                    break;
                }
            }
        }
    }
    
    // Kill the process
    let _ = child.kill();
    let _ = child.wait();
    
    assert!(found_connection_attempt, "Expected connection attempt with provided parameters");
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
        .env("PULSEPRINT_TEST_CONFIG_DIR", temp_dir.path())
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
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let output = Command::new("cargo")
        .args(&["run", "--", "remove", "nonexistent"])
        .env("PULSEPRINT_TEST_CONFIG_DIR", temp_dir.path())
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");
    assert!(stderr.contains("Printer not found"));
}

#[test]
fn test_set_default_nonexistent_printer() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let output = Command::new("cargo")
        .args(&["run", "--", "set-default", "nonexistent"])
        .env("PULSEPRINT_TEST_CONFIG_DIR", temp_dir.path())
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");
    assert!(stderr.contains("Printer not found"));
}
