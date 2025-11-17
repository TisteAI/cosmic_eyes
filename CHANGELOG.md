# Changelog

All notable changes to Cosmic Eyes will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-11-17

### Added

#### Core Architecture
- Complete project architecture and component design
- Comprehensive documentation (README, CLAUDE.md, CONTRIBUTING.md)
- Rust workspace with libcosmic integration for COSMIC Desktop
- Build system using cargo and justfile for task automation

#### Configuration System
- RON-based configuration with human-readable format
- Configuration file management (`~/.config/cosmic-eyes/config.ron`)
- Configurable short and long break intervals
- Settings for idle detection, skip/postpone behavior, and strict mode
- Example configuration file with inline documentation

#### Timer Service
- Dual independent timers for short and long breaks
- Thread-safe state management using Arc<RwLock<T>>
- Timer states: Running, Paused, InBreak, Postponed
- Break scheduling and time tracking logic
- Methods for starting, skipping, and postponing breaks
- Idle detection support (structure ready)

#### COSMIC Panel Applet
- Native COSMIC Desktop panel integration
- Icon button in system panel
- Popup window with timer information and controls
- Timer subscription structure (display integration pending)
- Quick action buttons for manual break triggering
- Proper applet desktop entry configuration

#### Break Screen UI
- Fullscreen break overlay component
- Countdown timer display
- Contextual break messages and suggestions
- Optional skip/postpone buttons (respects configuration)
- Different layouts for short vs long breaks

#### CLI Interface
- Command-line tool for controlling the applet
- Subcommands: break, skip, postpone, status, pause, resume, config, set
- Argument parsing with clap v4
- D-Bus IPC architecture (structure ready, implementation pending)

#### Development Infrastructure
- Comprehensive developer documentation (CLAUDE.md)
- Project structure following Rust and COSMIC best practices
- Git repository with proper .gitignore
- Desktop entry file for applet registration
- Application icon (SVG format)
- GPL-3.0 license

### Project Status

This release represents the **architectural and design phase** of Cosmic Eyes:

- ✅ **Complete**: Configuration system, timer logic, UI components, project structure
- 🚧 **In Progress**: D-Bus IPC integration, automatic break triggering, idle detection
- 📋 **Planned**: Statistics tracking, notifications, plugin system, smart scheduling

### Technical Details

- **Language**: Rust 2021 Edition
- **Framework**: libcosmic (COSMIC Desktop toolkit)
- **Async Runtime**: tokio
- **UI Pattern**: Model-View-Update (from iced)
- **Configuration**: RON (Rusty Object Notation)
- **IPC**: zbus (D-Bus)

### Dependencies

**System Requirements**:
- COSMIC Desktop environment
- System libraries: libxkbcommon, wayland-protocols, libwayland, pkg-config

**Rust Crates**:
- libcosmic (from git)
- tokio 1.35+
- clap 4.5
- serde 1.0
- ron 0.8
- chrono 0.4
- zbus 4.0
- tracing 0.1
- dirs 5.0

### Documentation

- **README.md**: User-facing documentation with features, installation, and usage
- **CLAUDE.md**: Developer guide with architecture and implementation details
- **CONTRIBUTING.md**: Guidelines for contributors
- **config.example.ron**: Annotated example configuration
- Inline code documentation throughout source files

### Known Limitations

- D-Bus communication between CLI and applet requires implementation
- Idle detection configuration ready but system API integration needed
- Automatic break triggering logic ready but subscription integration needed
- Real-time timer display in applet popup needs integration
- Pre-break notification system pending implementation
- Author information in Cargo.toml needs customization
- Build requires system libraries: libxkbcommon, wayland-protocols, libwayland (documented in Prerequisites)

### Notes for Users

This is an **alpha release** focused on establishing the project foundation. The core components are functional but require integration work to create a fully operational application. Contributions are welcome!

---

[0.1.0]: https://github.com/TisteAI/cosmic_eyes/releases/tag/v0.1.0
