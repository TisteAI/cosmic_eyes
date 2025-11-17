# Cosmic Eyes v0.1.0 - Initial Architecture Release

**Release Date:** 2025-11-17
**Release Type:** Alpha / Architecture Phase

## 🎯 Overview

Cosmic Eyes is a break reminder application for COSMIC Desktop designed to prevent eye strain and promote healthy work habits through regular, customizable break reminders.

This initial v0.1.0 release establishes the complete project architecture and implements core components, providing a solid foundation for future development.

## ✨ What's Included

### Core Features
- **Dual Timer System**: Independent short (20 min) and long (60 min) break timers
- **COSMIC Panel Integration**: Native applet with icon button and popup controls
- **Configuration Management**: Human-readable RON format with auto-save
- **CLI Interface**: Command-line tool for break control and status checking
- **Break Screen**: Fullscreen overlay with countdown and controls
- **Thread-Safe Architecture**: Async Rust with tokio and Arc<RwLock> patterns

### Components Implemented

#### 1. Configuration System (`src/config.rs`)
- RON-based configuration storage
- Auto-creation of config directory
- Default settings with customization support
- Settings: break intervals, durations, idle detection, strict mode

#### 2. Timer Service (`src/timer.rs`)
- Dual independent break timers
- State management: Running, Paused, InBreak, Postponed
- Methods for start, skip, postpone operations
- Time tracking and break scheduling logic

#### 3. COSMIC Panel Applet (`src/applet/mod.rs`)
- Panel icon with hover popup
- Timer display and status information
- Quick action buttons (Start Break, etc.)
- Real-time updates via subscription
- Proper COSMIC applet integration

#### 4. Break Screen UI (`src/break_screen/mod.rs`)
- Fullscreen break overlay
- Countdown timer display
- Contextual messages for different break types
- Optional skip/postpone buttons (config-driven)

#### 5. CLI Tool (`src/cli/main.rs`)
- Subcommands: `break`, `skip`, `postpone`, `status`, `pause`, `resume`, `config`, `set`
- Argument parsing with clap v4
- D-Bus IPC architecture (structure ready)

### Documentation

- **README.md**: Comprehensive user guide (320+ lines)
  - Installation instructions
  - Usage examples
  - Configuration reference
  - Development guide

- **CLAUDE.md**: Developer documentation (376+ lines)
  - Architecture overview
  - Component descriptions
  - Design patterns
  - Future extensibility plans

- **CONTRIBUTING.md**: Contribution guidelines
- **config.example.ron**: Annotated example configuration
- **CHANGELOG.md**: Version history and changes

## 🔧 Technical Stack

- **Language**: Rust 2021 Edition
- **GUI Framework**: libcosmic (COSMIC Desktop native)
- **Async Runtime**: tokio 1.35+
- **UI Pattern**: Model-View-Update (iced-based)
- **CLI**: clap 4.5
- **Configuration**: RON (Rusty Object Notation)
- **IPC**: zbus 4.0 (D-Bus)
- **Build System**: cargo + justfile

## 📋 Project Status

### ✅ Complete
- Project structure and architecture
- Configuration system with persistence
- Timer service logic and state management
- UI components (applet and break screen)
- CLI argument parsing
- Comprehensive documentation

### 🚧 In Progress
- D-Bus IPC communication between CLI and applet
- Automatic break triggering integration
- Real-time timer display updates
- Idle detection system integration

### 📅 Planned for Future Releases
- Statistics tracking (breaks taken/skipped, streaks)
- Desktop notifications with warnings
- Smart scheduling (calendar integration, focus time)
- Plugin system for break activities (exercises, breathing)
- Multi-monitor support
- Internationalization (i18n)

## 📦 Installation

### System Requirements

**COSMIC Desktop Environment**
- COSMIC Alpha 4 or later recommended

**System Dependencies**:
```bash
# Debian/Ubuntu
sudo apt install libxkbcommon-dev wayland-protocols \
                 libwayland-dev pkg-config

# Fedora
sudo dnf install libxkbcommon-devel wayland-protocols-devel \
                 wayland-devel pkgconfig
```

**Rust Toolchain**:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install just  # Optional: for build automation
```

### Building

```bash
# Clone repository
git clone https://github.com/TisteAI/cosmic_eyes.git
cd cosmic_eyes

# Build release version
cargo build --release
# OR use justfile
just build-release

# Install system-wide (optional)
sudo just install
```

### Configuration

Configuration file: `~/.config/cosmic-eyes/config.ron`

See `config.example.ron` for all available options.

## 🎮 Usage

### Panel Applet
```bash
# Run the applet
cosmic-eyes
# OR
just run
```

The applet will appear in the COSMIC panel. Click or hover to see controls.

### CLI Commands
```bash
# Start a break immediately
cosmic-eyes-cli break short
cosmic-eyes-cli break long

# Check status
cosmic-eyes-cli status

# Pause/resume timers
cosmic-eyes-cli pause
cosmic-eyes-cli resume

# View configuration
cosmic-eyes-cli config
```

## ⚠️ Known Limitations

1. **D-Bus Integration**: CLI-to-applet communication requires implementation
2. **Build Dependencies**: System libraries must be installed (xkbcommon, wayland)
3. **Idle Detection**: System integration pending
4. **Author Info**: Placeholder in Cargo.toml needs customization
5. **Alpha Status**: This is an architectural release; full integration pending

## 🤝 Contributing

Contributions are welcome! This is an early-stage project with many opportunities:

- Implement D-Bus IPC for CLI-applet communication
- Add idle detection using system APIs
- Integrate break triggering with timer service
- Add unit tests for timer logic
- Improve UI/UX with animations and transitions
- Add statistics tracking
- Implement notification system

See `CONTRIBUTING.md` for guidelines.

## 📄 License

GPL-3.0 - Following COSMIC Desktop ecosystem conventions

## 🔗 Links

- **Repository**: https://github.com/TisteAI/cosmic_eyes
- **Issues**: https://github.com/TisteAI/cosmic_eyes/issues
- **COSMIC Desktop**: https://system76.com/cosmic

## 🙏 Acknowledgments

Inspired by:
- **Workrave**: Independent break timers and statistics tracking
- **SafeEyes**: Strict mode and exercise suggestions
- **GNOME Break Timer**: Clean UI and desktop integration

Built with:
- **libcosmic**: COSMIC Desktop toolkit by System76
- **iced**: Rust GUI framework
- Rust community crates and tools

---

**Note to Users**: This is an alpha release focused on establishing the project foundation. The core architecture is solid, but integration work is needed for full functionality. We welcome contributions and feedback!
