# Cosmic Eyes 👁️

**Break Reminder for COSMIC Desktop**

Cosmic Eyes is a native break reminder application for the COSMIC Desktop environment that helps prevent eye strain and promotes healthy work habits by reminding you to take regular breaks.

Inspired by tools like Workrave and SafeEyes, Cosmic Eyes is built from the ground up using Rust and libcosmic for seamless integration with the COSMIC Desktop experience.

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![License](https://img.shields.io/badge/license-GPL--3.0-green)
![Platform](https://img.shields.io/badge/platform-COSMIC%20Desktop-purple)

## ✨ Features

### Current Features (v0.1.0)

- **Dual Break System**
  - Short breaks: Quick eye rest (default: every 20 min, 20 sec)
  - Long breaks: Extended rest with stretching (default: every 60 min, 5 min)
  - Independently configurable timers

- **Flexible Break Control**
  - Start breaks ahead of time
  - Skip breaks when needed (configurable)
  - Postpone breaks by a set duration (configurable)
  - Strict mode to enforce breaks without skip/postpone

- **COSMIC Panel Integration**
  - Native applet with icon in panel
  - Hover popup with quick controls and status
  - Shows time until next break
  - Quick action buttons for manual breaks

- **CLI Interface**
  - Control breaks from terminal
  - Check timer status
  - Manage configuration
  - Perfect for scripting and automation

- **Smart Timer Management**
  - Idle detection to pause timers
  - Countdown display during breaks
  - Pre-break notifications

- **Configurable Settings**
  - Customizable break intervals and durations
  - Toggle idle detection
  - Control skip/postpone permissions
  - Persistent configuration

### Planned Features

- [ ] Break exercises and suggestions
- [ ] Statistics and tracking
- [ ] Calendar/meeting integration
- [ ] Multi-monitor support
- [ ] Plugin system for custom break activities
- [ ] Focus mode / Pomodoro integration
- [ ] Sound effects and custom notifications
- [ ] Auto-skip during video calls (webcam detection)

## 🚀 Installation

### Prerequisites

- **COSMIC Desktop Environment** (Pop!_OS 24.04+ or compatible distribution)
- **Rust toolchain** (1.70 or later)
- **just** command runner (optional but recommended)

### From Source

1. Clone the repository:
```bash
git clone https://github.com/yourusername/cosmic-eyes.git
cd cosmic-eyes
```

2. Install just (task runner):
```bash
cargo install just
```

3. Build and install:
```bash
just build-release
sudo just install
```

4. Restart COSMIC Panel:
```bash
systemctl --user restart cosmic-panel
```

The Cosmic Eyes icon should now appear in your COSMIC Panel!

### Development Build

For development and testing:
```bash
just build
just run
```

## 📖 Usage

### Panel Applet

- **Click/Hover** the Cosmic Eyes icon in your panel to open controls
- **Start Break**: Click "Short Break" or "Long Break" to start immediately
- **View Status**: See time remaining until next break

### During a Break

When a break starts:
- Screen is blanked with a countdown timer
- Shows break type and suggested activity
- Options to **Skip** or **Postpone** (if enabled in config)

### Command Line Interface

Control Cosmic Eyes from your terminal:

```bash
# Start a break immediately
cosmic-eyes-cli break short
cosmic-eyes-cli break long

# Skip the current break
cosmic-eyes-cli skip

# Postpone the next break
cosmic-eyes-cli postpone short
cosmic-eyes-cli postpone long

# Check current status
cosmic-eyes-cli status

# Pause timer (e.g., during a meeting)
cosmic-eyes-cli pause

# Resume timer
cosmic-eyes-cli resume

# View current configuration
cosmic-eyes-cli config

# Modify configuration
cosmic-eyes-cli set short_break.interval 15
cosmic-eyes-cli set strict_mode true
```

## ⚙️ Configuration

Configuration file location: `~/.config/cosmic-eyes/config.ron`

### Default Configuration

```ron
Config(
    short_break: BreakConfig(
        interval: 20,      // Every 20 minutes
        duration: 20,      // 20 seconds long
        enabled: true,
    ),
    long_break: BreakConfig(
        interval: 60,      // Every 60 minutes (1 hour)
        duration: 300,     // 5 minutes long
        enabled: true,
    ),
    idle_detection: true,
    idle_threshold: 300,   // 5 minutes idle before pausing
    notification_before_break: 10,  // 10 second warning
    allow_skip: true,
    allow_postpone: true,
    postpone_duration: 5,  // Postpone by 5 minutes
    strict_mode: false,    // Enable to disallow skip/postpone
)
```

### Editing Configuration

1. **Manually**: Edit `~/.config/cosmic-eyes/config.ron`
2. **Via CLI**: Use `cosmic-eyes-cli set <key> <value>`
3. **Via UI**: Settings panel (coming soon)

After editing, the applet will automatically reload the configuration.

### Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `short_break.interval` | minutes | 20 | Time between short breaks |
| `short_break.duration` | seconds | 20 | Length of short break |
| `short_break.enabled` | bool | true | Enable short breaks |
| `long_break.interval` | minutes | 60 | Time between long breaks |
| `long_break.duration` | seconds | 300 | Length of long break |
| `long_break.enabled` | bool | true | Enable long breaks |
| `idle_detection` | bool | true | Pause timers when idle |
| `idle_threshold` | seconds | 300 | Idle time before pausing |
| `notification_before_break` | seconds | 10 | Warning time before break |
| `allow_skip` | bool | true | Allow skipping breaks |
| `allow_postpone` | bool | true | Allow postponing breaks |
| `postpone_duration` | minutes | 5 | How long to postpone |
| `strict_mode` | bool | false | Enforce breaks (no skip/postpone) |

## 🛠️ Development

### Building

```bash
# Debug build
just build

# Release build
just build-release

# Run applet
just run

# Run CLI
just run-cli status

# Run tests
just test

# Check code (clippy)
just check

# Format code
just fmt
```

### Project Structure

```
cosmic-eyes/
├── src/
│   ├── main.rs           # Applet entry point
│   ├── applet/           # COSMIC Panel applet
│   ├── break_screen/     # Break overlay UI
│   ├── cli/              # CLI interface
│   ├── config.rs         # Configuration management
│   └── timer.rs          # Timer service logic
├── res/
│   ├── cosmic-eyes.desktop
│   └── icons/
├── Cargo.toml
├── justfile
├── README.md
└── CLAUDE.md             # Developer documentation
```

### Architecture

Cosmic Eyes uses a modular architecture:

- **Timer Service**: Manages break intervals with async Rust (tokio)
- **Config System**: RON format for human-readable settings
- **Applet**: libcosmic-based panel integration with MVU pattern
- **Break Screen**: Fullscreen overlay during breaks
- **CLI**: Command-line interface with D-Bus IPC

For detailed development information, see [CLAUDE.md](CLAUDE.md).

## 🤝 Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and linter (`just check && just test`)
5. Format code (`just fmt`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

Please ensure:
- Code follows Rust and COSMIC conventions
- New features include tests
- Documentation is updated
- Commits are atomic and well-described

## 🐛 Bug Reports & Feature Requests

Please use GitHub Issues to report bugs or request features:

- **Bug Report**: Describe the issue, steps to reproduce, and expected behavior
- **Feature Request**: Describe the feature and why it would be useful

## 📚 Resources

- [libcosmic Documentation](https://pop-os.github.io/libcosmic-book/)
- [COSMIC Desktop](https://system76.com/cosmic)
- [COSMIC UX Guidelines](https://system76.com/cosmic/ux)
- [Rust Programming Language](https://www.rust-lang.org/)

## 💡 Inspiration

Cosmic Eyes is inspired by:
- [Workrave](https://workrave.org/) - Break reminder with exercise suggestions
- [SafeEyes](https://github.com/slgobinath/SafeEyes) - Eye strain prevention for Linux
- [GNOME Break Timer](https://wiki.gnome.org/Apps/BreakTimer) - Simple break reminders

## 📄 License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- System76 and the Pop!_OS team for COSMIC Desktop and libcosmic
- The Rust community for excellent tooling and libraries
- Contributors to Workrave and SafeEyes for inspiration

## 📧 Contact

For questions or discussions, please open an issue on GitHub.

---

**Remember to take breaks!** Your eyes and body will thank you. 👁️💚