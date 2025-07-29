# PulsePrint-CLI Roadmap

## Project Vision
PulsePrint-CLI aims to be the premier command-line tool for monitoring and managing Bambu Labs 3D printers, offering real-time status updates, background monitoring, notifications, and a beautiful terminal user interface for comprehensive printer fleet management.

## Release Schedule

### Alpha Phase (v0.1.0 - v0.3.0)
**Timeline: Q3-Q4 2025**

#### v0.1.0 - Foundation (Target: August 2025)
- [x] Basic CLI structure with clap
- [x] MQTT client implementation (issue #14)
- [x] Basic printer connection via local network
- [x] Simple status polling (`monitor` command)
- [x] Configuration file support (JSON)
- [x] Single printer monitoring

#### v0.2.0 - Core Monitoring (Target: September 2025)
- [x] Real-time status updates via MQTT subscriptions
- [x] Print progress tracking (issue #15)
- [ ] Temperature monitoring
- [x] Error/warning detection
- [ ] Basic logging functionality
- [x] JSON message parsing (issue #15)

#### v0.3.0 - Multi-Printer Support (Target: October 2025)
- [x] Multiple printer configuration
- [x] Printer management commands (add, remove, list)
- [x] Set default printer
- [ ] Concurrent monitoring of multiple printers
- [ ] Basic printer grouping

### Beta Phase (v0.4.0 - v0.6.0)
**Timeline: Q4 2025 - Q1 2026**

#### v0.4.0 - Background Operations (Target: November 2025)
- [ ] Daemon mode for background monitoring
- [ ] System tray integration (where supported)
- [ ] Log rotation and management
- [ ] Basic notification system (desktop notifications)
- [ ] Print completion alerts
- [ ] Error/warning notifications

#### v0.5.0 - Enhanced Features (Target: December 2025)
- [ ] Print history tracking
- [ ] Statistics and analytics
- [ ] Export capabilities (CSV, JSON)
- [ ] Basic automation hooks
- [ ] Custom alert conditions
- [ ] Webhook support for notifications

#### v0.6.0 - TUI Preview (Target: January 2026)
- [ ] Initial Terminal User Interface using ratatui
- [ ] Dashboard view with printer overview
- [ ] Real-time status updates in TUI
- [ ] Keyboard navigation
- [ ] Theme support

### v1.0.0 - Stable Release
**Target: February 2026**

#### Core Features
- ✅ Stable MQTT communication
- ✅ Comprehensive printer monitoring
- ✅ Multi-printer management
- ✅ Background monitoring with notifications
- ✅ Beautiful, responsive TUI
- ✅ Comprehensive documentation
- ✅ Cross-platform support (Windows, macOS, Linux)

#### TUI Features
- **Dashboard View**: Overview of all configured printers
- **Printer Detail View**: Deep dive into individual printer status
- **Print Queue Management**: View and manage print jobs
- **Temperature Graphs**: Real-time temperature visualization
- **Log Viewer**: Integrated log viewing with filtering
- **Configuration Editor**: In-TUI configuration management

#### CLI Commands
- `pulseprint monitor [PRINTER]` - Monitor printer(s) status
- `pulseprint tui` - Launch Terminal User Interface
- `pulseprint daemon start|stop|status` - Background service management
- `pulseprint printer add|remove|list|set-default` - Printer management
- `pulseprint config` - Configuration management
- `pulseprint history` - View print history
- `pulseprint stats` - View statistics and analytics

## Future Nice-to-Haves (Post v1.0)

### Enhanced Monitoring Features
- [ ] Predictive maintenance alerts based on usage patterns
- [ ] Detailed filament usage tracking and cost calculations
- [ ] Print failure analysis with common issue detection
- [ ] Camera stream integration (if Bambu API supports it)
- [ ] G-code preview in TUI

### Integration Possibilities
- [ ] Home Assistant integration
- [ ] Discord/Slack bot for notifications
- [ ] Simple web dashboard for remote monitoring
- [ ] Mobile-friendly web interface
- [ ] Integration with OctoPrint-style interfaces

### Advanced Automation
- [ ] Simple scripting for custom actions
- [ ] IFTTT-style automation rules
- [ ] Print queue management from CSV/folder
- [ ] Automatic print retry on failure
- [ ] Time-based printing schedules

### Community Features
- [ ] Plugin system for community extensions
- [ ] Shared printer profiles
- [ ] Community automation recipes
- [ ] Multi-language support

## Release Strategy

### GitHub Releases
- **v0.1.0-alpha** (August 2025): GitHub releases only with clear alpha warnings
- **Alpha/Beta phases**: Continue GitHub releases with pre-release tags (v0.1.0-alpha.1, v0.2.0-beta.1)
- **Early access**: `cargo install --git` for Rust developers and early adopters

### Package Manager Distribution Timeline

#### Limited Distribution (v0.4.0+)
- **AUR (Arch User Repository)**: Community expects bleeding-edge software
- **GitHub releases**: Continue with beta tags

#### Homebrew Distribution (v0.6.0+)
- **v0.6.0**: Consider Homebrew tap for early TUI users
- **v1.0.0**: Official Homebrew formula submission

#### Major Package Managers (v1.0.0)
- **Linux**: apt (Ubuntu/Debian), dnf (Fedora), pacman (Arch official)
- **macOS**: Homebrew official formula
- **Windows**: winget, chocolatey
- **Rust**: crates.io official release

### Distribution Philosophy
- **Alpha/Beta**: GitHub-only with experimental warnings
- **v0.6.0+**: Limited package managers for enthusiasts
- **v1.0.0**: Full distribution - users expect stability and polish

## Development Priorities

1. **Stability First**: Core MQTT functionality must be rock-solid
2. **User Experience**: Both CLI and TUI should be intuitive and responsive
3. **Performance**: Minimal resource usage, especially in daemon mode
4. **Documentation**: Comprehensive guides and examples
5. **Testing**: Extensive test coverage for reliability

## Contributing
We welcome contributions! Please see our [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to help shape the future of PulsePrint-CLI.

## Feedback
Have ideas or suggestions? Please open an issue on our [GitHub repository](https://github.com/voodu00/pulseprint-cli) to discuss new features or improvements.