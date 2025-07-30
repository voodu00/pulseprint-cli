# PulsePrint-CLI

A Rust-based command-line tool for monitoring Bambu Labs 3D printers via MQTT with TLS encryption.

## Features

- 🔐 **Secure MQTT Connection**: TLS-encrypted connections to Bambu Labs printers
- 🔄 **Auto-Reconnection**: Automatic reconnection with exponential backoff
- 📊 **Real-time Monitoring**: Event-driven MQTT subscriptions for instant status updates
- 📨 **Message Queue**: Built-in backpressure handling for reliable message processing
- 🛠️ **CLI Interface**: Easy-to-use command-line interface with comprehensive help
- ⚙️ **Configuration Management**: TOML/JSON-based configuration system for managing multiple printers
- 🏠 **Cross-Platform Config**: Automatic configuration directory detection (Linux/macOS/Windows)

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- Access to a Bambu Labs printer on your local network

### Build from Source

```bash
git clone <repository-url>
cd pulseprint-cli
cargo build --release
```

## Usage

### Printer Management

First, add your printer to the configuration:

```bash
cargo run -- add \
  --name "my-x1c" \
  --ip 192.168.1.100 \
  --device-id 01S00A000000000 \
  --access-code 12345678 \
  --set-default
```

List all configured printers:

```bash
cargo run -- list
```

### Basic Monitoring

Monitor your default printer:

```bash
cargo run -- monitor
```

Monitor a specific printer by name:

```bash
cargo run -- monitor --name "my-x1c"
```

Or provide connection details directly:

```bash
cargo run -- monitor \
  --ip 192.168.1.100 \
  --device-id 01S00A000000000 \
  --access-code 12345678
```

### Getting Help

```bash
# General help
cargo run -- --help

# Monitor command help
cargo run -- monitor --help
```

## Connection Requirements

PulsePrint-CLI connects to Bambu Labs printers using the following specifications:

- **Protocol**: MQTT over TLS
- **Port**: 8883
- **Username**: `bblp`
- **Password**: Your printer's LAN access code
- **Topic**: `device/{DEVICE_ID}/report` (for status monitoring)

### Finding Your Printer Details

1. **Printer IP**: Check your router's admin panel or use network discovery tools
2. **Device ID**: Found in your printer's network settings or Bambu Studio
3. **Access Code**: Located in your printer's network settings (LAN access code)

## Command Reference

### Global Options

- `-h, --help`: Print help information
- `-V, --version`: Print version information

### Add Command

Add a new printer configuration.

**Arguments:**
- `-n, --name <NAME>`: Printer name (unique identifier)
- `-i, --ip <IP>`: Printer IP address
- `-d, --device-id <ID>`: Device ID of the printer
- `-a, --access-code <CODE>`: LAN access code for the printer
- `--set-default`: Set as default printer (optional)

### List Command

List all configured printers.

### Remove Command

Remove a printer configuration.

**Arguments:**
- `<NAME>`: Name of the printer to remove

### Set-Default Command

Set the default printer.

**Arguments:**
- `<NAME>`: Name of the printer to set as default

### Monitor Command

Monitor a Bambu Labs printer via MQTT.

**Arguments (all optional):**
- `-n, --name <NAME>`: Printer name from config (uses default if not specified)
- `-i, --ip <IP>`: Printer IP address (overrides config)
- `-d, --device-id <ID>`: Device ID of the printer (overrides config)
- `-a, --access-code <CODE>`: LAN access code for authentication (overrides config)

**Usage patterns:**
- `monitor` - Monitor the default printer
- `monitor --name my-printer` - Monitor a specific configured printer
- `monitor --ip 192.168.1.100 --device-id ... --access-code ...` - Direct connection without config

**Monitor output example:**
```
🖨️ Print Status: Printing - Remaining: 16m 55s
🖨️ Printer Status: 🌡️ Nozzle: 219.8°C | 🛏️ Bed: 45.0°C | 📄 Layer: 10 | ⏱️ Remaining: 16m | 📶 WiFi: -30dBm
```

## Development

### Building

```bash
cargo build          # Debug build
cargo build --release # Release build
```

### Testing

```bash
cargo test                    # Run all tests (unit + integration)
cargo test --bin pulseprint-cli  # Run unit tests only
cargo test --tests           # Run integration tests only
cargo test test_printer_config   # Run specific test by name
cargo check                   # Check code without building
```

**Test Structure:**
- Unit tests: `src/config/tests.rs` - Configuration management tests
- Unit tests: `src/mqtt/tests.rs` - MQTT client and connection tests  
- Integration tests: `tests/integration_tests.rs` - CLI commands and full workflows

**Test Coverage:**
- ✅ Printer configuration creation and validation
- ✅ Configuration file save/load operations
- ✅ Printer management (add, remove, default selection)  
- ✅ MQTT client configuration and creation
- ✅ Connection parameter validation
- ✅ CLI argument parsing and validation
- ✅ Help command functionality
- ✅ Error handling for invalid inputs
- ✅ Topic format generation
- ✅ MQTT message parsing (JSON structure)
- ✅ Bambu Labs printer status parsing
- ✅ Print state inference and display
- ✅ Temperature and progress monitoring

### Code Quality

```bash
cargo clippy         # Run linter
cargo fmt            # Format code
```

### Project Structure

```
src/
├── main.rs          # CLI entry point and command handling
├── config/
│   ├── mod.rs       # Configuration management and data structures
│   └── tests.rs     # Configuration unit tests
├── messages/
│   ├── mod.rs       # MQTT message parsing and printer status
│   └── tests.rs     # Message parsing unit tests
└── mqtt/
    ├── mod.rs       # MQTT client implementation with TLS
    ├── subscription.rs # Real-time subscription management
    └── tests.rs     # MQTT-specific unit tests
```

## Technical Details

### MQTT Implementation

- **Client Library**: [rumqttc](https://crates.io/crates/rumqttc) v0.24
- **TLS Support**: Native TLS with certificate validation
- **Connection Handling**: Automatic reconnection with exponential backoff
- **Topic Subscription**: Real-time event-driven subscriptions to device report topics
- **Message Processing**: Async message handling with separate subscription and processing tasks
- **Reliability**: Built-in message queue with 100-message buffer for backpressure handling

### Dependencies

**Runtime Dependencies:**
- **clap**: Command-line argument parsing with derive macros
- **tokio**: Async runtime for MQTT operations
- **rumqttc**: MQTT client with TLS support
- **serde**: JSON serialization for configuration and MQTT messages
- **serde_json**: JSON parsing and generation
- **rustls**: Modern TLS library for secure connections
- **thiserror**: Derive macros for error handling
- **dirs**: Cross-platform configuration directory detection

**Development Dependencies:**
- **tokio-test**: Testing utilities for async code
- **mockall**: Mock object library for testing
- **tempfile**: Temporary file creation for testing file operations

## API Reference

This project implements the [OpenBambuAPI](https://github.com/Doridian/OpenBambuAPI) specification for communicating with Bambu Labs printers.

### Supported Topics

- `device/{DEVICE_ID}/report` - Device status and information (subscribed automatically)

### Future Topic Support

- `device/{DEVICE_ID}/request` - Commands to device (planned)

## Troubleshooting

### Connection Issues

1. **"Connection failed"**: Verify printer IP address and network connectivity
2. **"Authentication failed"**: Check that the access code is correct
3. **"TLS handshake failed"**: Ensure the printer supports TLS on port 8883

### Network Requirements

- Printer and client must be on the same network
- Port 8883 must be accessible (check firewall settings)
- Printer must have network access enabled

## Contributing

We follow a Git flow branching model:
- `main` - Stable releases only
- `develop` - Active development branch

1. Fork the repository
2. Create a feature branch from `develop` (`git checkout -b feature/amazing-feature develop`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request targeting the `develop` branch

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Configuration System

PulsePrint-CLI includes a comprehensive configuration system for managing multiple printers with support for both TOML and JSON formats.

### Configuration File Formats

PulsePrint-CLI supports both **TOML** (preferred) and **JSON** configuration formats:

- **New installations**: Default to TOML format (`config.toml`)
- **Existing installations**: Continue to support JSON format (`config.json`) 
- **Format detection**: Automatic based on file extension
- **Backward compatibility**: Existing JSON configs work seamlessly

### Configuration File Location

Configuration files are automatically stored in the appropriate location for your operating system:

- **Linux**: `~/.config/pulseprint-cli/config.toml` (or `config.json`)
- **macOS**: `~/Library/Application Support/pulseprint-cli/config.toml` (or `config.json`)
- **Windows**: `%APPDATA%\pulseprint-cli\config.toml` (or `config.json`)

### Configuration Structure

#### TOML Format (Preferred)

```toml
default_printer = "my_printer"

[printers.my_printer]
name = "my_printer"
ip = "192.168.1.100"
device_id = "01S00A000000000"
access_code = "12345678"
port = 8883
use_tls = true

[mqtt_settings]
keep_alive_secs = 30
connection_timeout_secs = 10
retry_attempts = 5
retry_delay_secs = 5
queue_size = 10
```

#### JSON Format (Legacy Support)

```json
{
  "printers": {
    "my_printer": {
      "name": "my_printer",
      "ip": "192.168.1.100",
      "device_id": "01S00A000000000",
      "access_code": "12345678",
      "port": 8883,
      "use_tls": true,
      "model": null,
      "firmware_version": null
    }
  },
  "default_printer": "my_printer",
  "mqtt_settings": {
    "keep_alive_secs": 30,
    "connection_timeout_secs": 10,
    "retry_attempts": 5,
    "retry_delay_secs": 5,
    "queue_size": 10
  }
}
```

## Status

🚧 **Currently in Development**

- ✅ MQTT connection with TLS support (issue #14)
- ✅ Basic printer monitoring
- ✅ CLI interface with help system
- ✅ Error handling and retry logic
- ✅ Configuration management system with TOML and JSON support (issue #17)
- ✅ Multiple printer support with named configurations
- ✅ Printer management CLI commands (add, remove, list, set-default)
- ✅ Message parsing for MQTT JSON messages (issue #15)
- ✅ Real-time status display with print progress
- ✅ Simple status polling functionality (issue #16)
- ✅ Bambu Labs printer status parsing with temperatures, layers, and timing
- ✅ Print state inference and visual status indicators
- ✅ Real-time MQTT subscriptions with event-driven updates (issue #19)
- 🚧 Advanced status monitoring and display (planned for issue #6)
- 🚧 Command sending capabilities (planned)
- 🚧 File upload support (planned)

## Acknowledgments

- [OpenBambuAPI](https://github.com/Doridian/OpenBambuAPI) for the API specification
- Bambu Labs for creating awesome 3D printers
- The Rust community for excellent crates and tools