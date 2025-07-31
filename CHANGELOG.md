# Changelog

All notable changes to PulsePrint-CLI will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Real-time MQTT subscriptions for printer monitoring (#19)
  - Event-driven architecture replacing polling mechanism
  - Automatic reconnection with exponential backoff
  - Message queue with backpressure handling (100 message buffer)
  - Separate async tasks for subscription management and message processing
  - Graceful shutdown on interrupt signal (Ctrl+C)

### Changed
- Refactored monitor command to use new subscription-based approach
- Improved error handling with proper Send/Sync trait bounds
- Optimized memory usage by boxing large enum variants

### Fixed
- Connection stability issues with automatic reconnection logic

## [0.1.0-alpha.1] - 2024-12-20

### Added
- Initial alpha release
- Basic CLI structure with subcommands
- MQTT connection to Bambu Labs printers
- Printer configuration management (add, remove, list, set-default)
- TOML configuration file support with JSON backward compatibility
- Simple status polling for printer monitoring
- Multi-printer support with default printer selection
- Secure TLS connections with self-signed certificate support

### Security
- LAN access codes stored locally in configuration files
- TLS encryption for MQTT connections

[Unreleased]: https://github.com/voodu00/pulseprint-cli/compare/v0.1.0-alpha.1...HEAD
[0.1.0-alpha.1]: https://github.com/voodu00/pulseprint-cli/releases/tag/v0.1.0-alpha.1