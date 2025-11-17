# Cosmic Eyes - Claude Development Guide

## Project Overview

**Cosmic Eyes** is a break reminder application for the COSMIC Desktop environment, designed to help users prevent eye strain and maintain healthy work habits by reminding them to take regular breaks.

The application is inspired by tools like Workrave and SafeEyes, but built natively for COSMIC Desktop using Rust and libcosmic.

## Implementation Status (v0.1.0)

This is an **alpha release** establishing the project architecture. The codebase contains:

### ✅ Fully Implemented
- Configuration system with RON format (parsing, saving, loading)
- Timer service with thread-safe state management
- Break scheduling logic (intervals, durations, state transitions)
- COSMIC Panel applet structure with popup window
- Break screen UI component with countdown display
- CLI argument parsing with clap (command structure)
- Project documentation and build system

### 🚧 Partial / Integration Needed
- **Real-time timer display**: Logic exists, UI integration pending
- **Automatic break triggering**: Check logic ready, subscription integration needed
- **CLI-to-applet communication**: D-Bus architecture designed, implementation pending
- **Idle detection**: Configuration ready, system API integration needed
- **Pre-break notifications**: Configuration ready, notification system pending

### 📝 Design/Planning Phase
- Statistics tracking
- Plugin system
- Multi-monitor support
- Smart scheduling
- Advanced notification system

This document describes both **implemented features** and **architectural design** for planned features. When in doubt about implementation status, check the source code or see the status indicators above.

## Architecture

### Core Components

1. **Timer Service** (`src/timer.rs`) ✅
   - Manages break intervals using async Rust with tokio
   - Tracks time until next short and long breaks
   - Handles break state: Running, Paused, InBreak, Postponed
   - Thread-safe using Arc<RwLock<T>> for shared state
   - Methods: `start_break()`, `end_break()`, `skip_break()`, `postpone_break()`, `pause()`, `resume()`

2. **Configuration** (`src/config.rs`) ✅
   - RON format for human-readable configuration
   - Stored in `~/.config/cosmic-eyes/config.ron`
   - Supports short and long break settings
   - Configuration fields for idle detection and break policies (integration pending)

3. **Applet** (`src/applet/mod.rs`) 🚧
   - COSMIC Panel integration using libcosmic ✅
   - Icon button in panel + popup window for controls ✅
   - MVU (Model-View-Update) pattern from iced ✅
   - Timer subscription exists, display integration pending 🚧
   - Quick action buttons functional ✅

4. **Break Screen** (`src/break_screen/mod.rs`) ✅
   - Fullscreen overlay component structure
   - Countdown timer display UI
   - Optional skip/postpone buttons (respects config)
   - Break type messages ("Time for a short break!", etc.)

5. **CLI Interface** (`src/cli/main.rs`) 🚧
   - Command-line parsing using clap ✅
   - Command structure defined (break, skip, postpone, status, pause, resume) ✅
   - D-Bus IPC architecture designed 🚧
   - Currently outputs placeholder messages for testing

### Technology Stack

- **Language**: Rust 2021 edition
- **GUI Framework**: libcosmic (built on iced)
- **Async Runtime**: tokio
- **CLI**: clap v4
- **IPC**: zbus (D-Bus)
- **Configuration**: ron (Rusty Object Notation)
- **Time**: chrono
- **Build**: cargo + just

## Key Design Patterns

### 1. Break Types
Two independent break types with separate timers:
- **Short breaks**: Frequent, brief (default: every 20 min, 20 sec)
- **Long breaks**: Less frequent, longer (default: every 60 min, 5 min)

### 2. Break Control
- **Start Early**: Trigger break immediately via CLI or applet
- **Skip**: End current break early (if allowed)
- **Postpone**: Delay break by configurable duration (default: 5 min)
- **Strict Mode**: Disable skip/postpone for enforced breaks

### 3. Idle Detection 🚧
**Status**: Configuration ready, system integration pending

**Design**:
- Monitor user activity to pause timers when idle
- Configurable idle threshold (default: 5 minutes)
- Prevents break notifications during natural breaks

**Implementation needs**:
- Query system idle time via D-Bus (`org.freedesktop.ScreenSaver`)
- Or use Wayland `ext-idle-notify-v1` protocol
- Or X11 XScreenSaver extension for X11 sessions
- Periodic check in applet subscription
- Auto-call `timer.pause()` when threshold exceeded
- Auto-call `timer.resume()` when activity detected

### 4. State Management
Using Arc<RwLock<T>> pattern for thread-safe state:
```rust
pub struct TimerService {
    config: Arc<RwLock<Config>>,
    state: Arc<RwLock<TimerState>>,
    short_break_next: Arc<RwLock<DateTime<Local>>>,
    long_break_next: Arc<RwLock<DateTime<Local>>>,
    break_end_time: Arc<RwLock<Option<DateTime<Local>>>>,
}
```

## COSMIC Desktop Integration

### Panel Applet
The application runs as a COSMIC Panel applet:

1. **Icon Button**: Shows in panel with cosmic-eyes icon
2. **Popup Window**: Hover/click reveals controls
3. **Desktop Entry**: Configured with X-CosmicApplet=true

### Desktop Entry Keys
```ini
NoDisplay=true              # Don't show in app launcher
X-CosmicApplet=true        # Register as applet
X-CosmicHoverPopup=Auto    # Popup behavior
X-OverflowPriority=10      # Panel overflow handling
```

### Applet Features
- Uses `cosmic::applet::run()` instead of `cosmic::app::run()`
- Transparent background via `cosmic::applet::style()`
- Multi-window support (main icon + popup)
- Wayland compositor integration

## Configuration System

### Default Settings
```ron
Config(
    short_break: BreakConfig(
        interval: 20,      // minutes
        duration: 20,      // seconds
        enabled: true,
    ),
    long_break: BreakConfig(
        interval: 60,      // minutes
        duration: 300,     // seconds (5 min)
        enabled: true,
    ),
    idle_detection: true,
    idle_threshold: 300,   // seconds
    notification_before_break: 10,  // seconds
    allow_skip: true,
    allow_postpone: true,
    postpone_duration: 5,  // minutes
    strict_mode: false,
)
```

### Configuration Location
- Linux: `~/.config/cosmic-eyes/config.ron`
- Auto-created on first run with defaults

## CLI Usage

**Current Status**: Command structure implemented, D-Bus communication pending

The CLI currently accepts all commands and outputs placeholder messages:

```bash
# Start a break immediately (outputs confirmation)
cosmic-eyes-cli break short
cosmic-eyes-cli break long

# Skip current break (outputs confirmation)
cosmic-eyes-cli skip

# Postpone next break (outputs confirmation)
cosmic-eyes-cli postpone short
cosmic-eyes-cli postpone long

# Check status (shows placeholder data)
cosmic-eyes-cli status

# Pause/resume timers (outputs confirmation)
cosmic-eyes-cli pause
cosmic-eyes-cli resume

# View configuration (shows placeholder)
cosmic-eyes-cli config

# Set configuration values (planned)
cosmic-eyes-cli set short_break.interval 15
cosmic-eyes-cli set strict_mode true
```

**To make CLI functional**, implement D-Bus interface in `src/cli/main.rs`:
- Uncomment and complete `send_dbus_command()` function
- Implement `get_dbus_status()` and `get_dbus_config()` functions
- Define D-Bus interface in applet for receiving commands
- See `zbus` documentation for examples

## Building and Development

### Prerequisites

**System Dependencies**:
```bash
# Debian/Ubuntu
sudo apt install libxkbcommon-dev wayland-protocols-devel \
                 libwayland-dev pkg-config

# Fedora
sudo dnf install libxkbcommon-devel wayland-protocols-devel \
                 wayland-devel pkgconfig
```

**Rust Toolchain**:
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install just (task runner)
cargo install just
```

**COSMIC Desktop**:
- libcosmic will be fetched from GitHub during build
- Requires COSMIC Desktop environment for applet functionality

### Build Commands
```bash
# Build debug version
just build

# Build release version
just build-release

# Run the applet
just run

# Run CLI
just run-cli status

# Install system-wide
sudo just install

# Run linter
just check

# Format code
just fmt

# Run tests
just test
```

### Project Structure
```
cosmic-eyes/
├── Cargo.toml              # Rust dependencies
├── justfile                # Build recipes
├── src/
│   ├── main.rs            # Applet entry point
│   ├── applet/            # COSMIC Panel applet
│   │   └── mod.rs
│   ├── break_screen/      # Break overlay UI
│   │   └── mod.rs
│   ├── cli/               # CLI interface
│   │   └── main.rs
│   ├── config.rs          # Configuration management
│   └── timer.rs           # Timer service
├── res/
│   ├── cosmic-eyes.desktop # Desktop entry
│   └── icons/             # Application icons
└── i18n/                  # Internationalization
    └── en/
```

## Future Extensibility

The architecture is designed to support future features:

### 1. Plugin System
```rust
pub trait BreakActivity {
    fn name(&self) -> &str;
    fn execute(&self, break_type: BreakType) -> Result<()>;
}
```

Potential plugins:
- Eye exercises with animations
- Breathing exercises
- Stretching routines
- Integration with fitness apps

### 2. Statistics Tracking
```rust
pub struct Statistics {
    breaks_taken: HashMap<BreakType, u64>,
    breaks_skipped: HashMap<BreakType, u64>,
    total_break_time: Duration,
    longest_streak: u64,
}
```

### 3. Smart Scheduling
- Calendar integration (don't interrupt meetings)
- Focus time detection (Pomodoro integration)
- Activity-based scheduling (pause during video calls)

### 4. Notifications
- Pre-break warnings (configurable time)
- Sound effects
- Desktop notifications (using COSMIC notification API)

### 5. Multi-Monitor Support
- Show break screen on all monitors
- Per-monitor configuration
- Monitor-specific break activities

## Best Practices Followed

### COSMIC Design Guidelines
1. **Responsive Design**: UI adapts to different panel sizes
2. **Theme Integration**: Uses COSMIC theme colors and spacing
3. **Accessibility**: Keyboard navigation and screen reader support
4. **Cross-Platform**: Works on X11 and Wayland

### Rust Best Practices
1. **Memory Safety**: No unsafe code (except in libcosmic internals)
2. **Error Handling**: Proper Result types throughout
3. **Async/Await**: Non-blocking operations with tokio
4. **Type Safety**: Strong typing for break types and states
5. **Testing**: Unit tests for timer logic (to be added)

### Similar App Inspiration
- **Workrave**: Independent short/long break timers, statistics
- **SafeEyes**: Strict mode, exercise suggestions, auto-skip during calls
- **GNOME Break Timer**: Clean UI, GNOME Shell integration

## Development Workflow

### Adding a New Feature

1. **Design**: Add to architecture docs
2. **Config**: Add new fields to `Config` struct
3. **State**: Update `TimerService` or create new service
4. **UI**: Add to applet popup and/or break screen
5. **CLI**: Add new subcommand if needed
6. **Test**: Add unit tests
7. **Document**: Update README and CLAUDE.md

### Code Style
- Run `just fmt` before committing
- Run `just check` to catch linting issues
- Follow Rust API Guidelines
- Use descriptive variable names
- Add doc comments for public APIs

## Debugging

### Enable Logging
```bash
RUST_LOG=debug cosmic-eyes
```

### Common Issues

1. **Applet doesn't appear in panel**
   - Check desktop entry installed correctly
   - Restart COSMIC Panel: `systemctl --user restart cosmic-panel`

2. **Configuration not loading**
   - Check `~/.config/cosmic-eyes/config.ron` syntax
   - Remove file to regenerate defaults

3. **Timer not triggering**
   - Check timer subscription in applet
   - Verify tokio runtime is running

## Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_timer_starts_break() {
        let config = Config::default();
        let timer = TimerService::new(config);

        timer.start_break(BreakType::Short).await;
        let state = timer.state().await;

        assert!(matches!(state, TimerState::InBreak(BreakType::Short)));
    }
}
```

### Integration Tests
- Test CLI commands
- Test D-Bus IPC
- Test configuration loading/saving

## Contributing Guidelines

1. Follow Rust and COSMIC conventions
2. Add tests for new features
3. Update documentation
4. Keep commits atomic and well-described
5. Ensure `just check` passes

## Resources

- [libcosmic Book](https://pop-os.github.io/libcosmic-book/)
- [COSMIC UX Guidelines](https://system76.com/cosmic/ux)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
- [iced Framework](https://github.com/iced-rs/iced)
- [Workrave Design](https://workrave.org/)
- [SafeEyes](https://github.com/slgobinath/SafeEyes)

## License

GPL-3.0 - Following COSMIC Desktop ecosystem conventions
