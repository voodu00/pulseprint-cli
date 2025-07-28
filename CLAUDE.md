# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
PulsePrint-CLI is a Rust-based command-line tool for monitoring Bambu Labs 3D printers via MQTT. The project is in early development with a basic CLI structure in place.

## Development Commands

### Build and Run
```bash
cargo build          # Build the project
cargo run            # Run the application
cargo run -- --help  # Show help information
cargo run -- monitor # Run monitor command
```

### Testing and Quality
```bash
cargo test           # Run tests
cargo clippy         # Run linter
cargo fmt            # Format code
cargo check          # Check code without building
```

## Architecture

### Project Structure
- **src/main.rs**: Entry point with CLI argument parsing using clap
- **Cargo.toml**: Project configuration and dependencies

### Current Implementation
The application uses clap's derive API for command-line parsing with a single `Monitor` subcommand. The architecture is designed to be extended with MQTT functionality for printer monitoring.

### Key Dependencies
- **clap 4.5.20**: Command-line argument parsing with derive features

## API Documentation
This project implements the OpenBambuAPI specification: https://github.com/Doridian/OpenBambuAPI

### MQTT Connection Details
**Local Connection:**
- URL: `mqtt://{PRINTER_IP}:8883` 
- TLS: Required
- Username: `bblp`
- Password: Device's LAN access code

### Key MQTT Topics
- `device/{DEVICE_ID}/report` - Device status and information
- `device/{DEVICE_ID}/request` - Commands to device

### Important Message Types
- `pushing.pushall` - Complete printer status (limit to every 5 minutes on P1 series)
- `print.push_status` - Print status updates
- All messages are JSON encoded with sequence_id for tracking

### Future Expansion Areas
The codebase is structured to accommodate:
- MQTT client implementation for printer communication
- Printer status monitoring and reporting
- Configuration management for multiple printers