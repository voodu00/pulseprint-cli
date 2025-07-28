# PulsePrint-CLI

A Rust-based command-line tool for monitoring Bambu Labs 3D printers via MQTT with TLS encryption.

## Features

- 🔐 **Secure MQTT Connection**: TLS-encrypted connections to Bambu Labs printers
- 🔄 **Auto-Retry Logic**: Robust connection handling with configurable retry attempts
- 📊 **Real-time Monitoring**: Live status updates from printer MQTT topics
- 🛠️ **CLI Interface**: Easy-to-use command-line interface with comprehensive help

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

### Basic Monitoring

Monitor a Bambu Labs printer by providing the required connection details:

```bash
cargo run -- monitor \
  --printer <PRINTER_IP> \
  --device-id <DEVICE_ID> \
  --access-code <ACCESS_CODE>
```

### Example

```bash
cargo run -- monitor \
  --printer 192.168.1.100 \
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

### Monitor Command

Monitor a Bambu Labs printer via MQTT.

**Required Arguments:**
- `-p, --printer <IP>`: Printer IP address
- `-d, --device-id <ID>`: Device ID of the printer  
- `-a, --access-code <CODE>`: LAN access code for authentication

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
- Unit tests: `src/mqtt/tests.rs` - Test individual components and functions
- Integration tests: `tests/integration_tests.rs` - Test CLI commands and full workflows

**Test Coverage:**
- ✅ MQTT client configuration and creation
- ✅ Connection parameter validation
- ✅ CLI argument parsing and validation
- ✅ Help command functionality
- ✅ Error handling for invalid inputs
- ✅ Topic format generation

### Code Quality

```bash
cargo clippy         # Run linter
cargo fmt            # Format code
```

### Project Structure

```
src/
├── main.rs          # CLI entry point and command handling
└── mqtt.rs          # MQTT client implementation with TLS
```

## Technical Details

### MQTT Implementation

- **Client Library**: [rumqttc](https://crates.io/crates/rumqttc) v0.24
- **TLS Support**: Native TLS with certificate validation
- **Connection Handling**: Automatic retry with exponential backoff
- **Topic Subscription**: Automatic subscription to device report topics

### Dependencies

**Runtime Dependencies:**
- **clap**: Command-line argument parsing
- **tokio**: Async runtime for MQTT operations
- **rumqttc**: MQTT client with TLS support
- **serde**: JSON serialization for MQTT messages
- **tokio-native-tls**: TLS connector for secure connections

**Development Dependencies:**
- **tokio-test**: Testing utilities for async code
- **mockall**: Mock object library for testing

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

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Status

🚧 **Currently in Development**

- ✅ MQTT connection with TLS support
- ✅ Basic printer monitoring
- ✅ CLI interface with help system
- ✅ Error handling and retry logic
- 🚧 Message parsing and display (in progress)
- 🚧 Configuration file support (planned)
- 🚧 Multiple printer support (planned)
- 🚧 Command sending capabilities (planned)

## Acknowledgments

- [OpenBambuAPI](https://github.com/Doridian/OpenBambuAPI) for the API specification
- Bambu Labs for creating awesome 3D printers
- The Rust community for excellent crates and tools