# Testing Cosmic Eyes on COSMIC Desktop

This guide explains how to test Cosmic Eyes v0.1.0 on a system with COSMIC Desktop installed.

## Prerequisites

### System Requirements
- **COSMIC Desktop** environment (Pop!_OS 24.04+ or COSMIC Alpha)
- **Rust toolchain** 1.70 or later
- **System libraries** for building

### Install System Dependencies

**On Debian/Ubuntu/Pop!_OS:**
```bash
sudo apt update
sudo apt install -y \
    libxkbcommon-dev \
    wayland-protocols \
    libwayland-dev \
    pkg-config \
    git \
    build-essential
```

**On Fedora:**
```bash
sudo dnf install -y \
    libxkbcommon-devel \
    wayland-protocols-devel \
    wayland-devel \
    pkgconfig \
    git \
    gcc
```

### Install Rust (if not already installed)

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload shell environment
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### Install Just (Optional but Recommended)

```bash
cargo install just

# Verify installation
just --version
```

## Clone and Build

### 1. Clone the Repository

```bash
# Clone from GitHub
git clone https://github.com/TisteAI/cosmic_eyes.git
cd cosmic_eyes

# Optional: checkout specific release
git checkout v0.1.0
```

### 2. Build the Project

**Using just (recommended):**
```bash
# Build release version
just build-release

# Binaries will be in: target/release/cosmic-eyes and target/release/cosmic-eyes-cli
```

**Using cargo directly:**
```bash
# Build release version
cargo build --release

# Verify build succeeded
ls -lh target/release/cosmic-eyes*
```

Expected output:
```
-rwxr-xr-x 1 user user  XXM ... target/release/cosmic-eyes
-rwxr-xr-x 1 user user  XXM ... target/release/cosmic-eyes-cli
```

### 3. Verify Configuration

Check that example config is present:
```bash
cat config.example.ron
```

## Installation Options

### Option A: System-Wide Installation (Recommended for Testing)

```bash
# Install using just
sudo just install
```

This will:
- Install binaries to `/usr/local/bin/`
- Install desktop entry to `/usr/share/applications/`
- Install icon to `/usr/share/icons/hicolor/scalable/apps/`

### Option B: Local Testing (No Installation)

```bash
# Copy desktop entry to local applications
mkdir -p ~/.local/share/applications
cp res/cosmic-eyes.desktop ~/.local/share/applications/

# Update desktop entry to point to local build
sed -i "s|Exec=cosmic-eyes|Exec=$PWD/target/release/cosmic-eyes|" \
    ~/.local/share/applications/cosmic-eyes.desktop

# Copy icon
mkdir -p ~/.local/share/icons/hicolor/scalable/apps
cp res/icons/hicolor/scalable/apps/cosmic-eyes.svg \
    ~/.local/share/icons/hicolor/scalable/apps/
```

### Option C: Run Directly (Development)

```bash
# Run without installing
just run
# OR
cargo run --release --bin cosmic-eyes
```

## Starting the Applet

### Restart COSMIC Panel

After installation, restart the COSMIC Panel to load the applet:

```bash
# Restart panel to pick up new applet
pkill cosmic-panel

# OR if that doesn't work, restart the entire COSMIC session:
systemctl --user restart cosmic-session
# OR: Log out and log back in
```

### Verify Applet Loaded

```bash
# Check if cosmic-eyes process is running
ps aux | grep cosmic-eyes

# Check COSMIC Panel logs for errors
journalctl --user -u cosmic-panel -f
```

## Testing the Applet

### 1. Locate the Applet

- Look for the **Cosmic Eyes icon** in your COSMIC Panel
- Icon should appear as an eye icon (if available) or fallback icon

### 2. Test Popup Interaction

- **Hover** or **click** on the icon
- A popup window should appear with:
  - Title: "Cosmic Eyes"
  - Status information (currently shows "Loading...")
  - Two buttons: "Short Break" and "Long Break"

### 3. Test Manual Break Triggering

**What works (v0.1.0):**
- Click "Short Break" or "Long Break" buttons
- Timer service will process the command
- State will change to `InBreak`

**What's pending:**
- Break screen won't automatically display (integration pending)
- No visual countdown yet (integration pending)

### 4. Check Configuration

```bash
# Config should auto-create on first run
cat ~/.config/cosmic-eyes/config.ron

# Should show default configuration with all settings
```

## Testing the CLI

### 1. Verify CLI Installation

```bash
# Check CLI is available
which cosmic-eyes-cli

# Try help command
cosmic-eyes-cli --help
```

### 2. Test CLI Commands

**v0.1.0 Status - CLI outputs placeholder messages:**

```bash
# Try starting a break
cosmic-eyes-cli break short
# Output: "Starting short break..."

# Check status
cosmic-eyes-cli status
# Output: Placeholder status information

# Try pause
cosmic-eyes-cli pause
# Output: "Pausing timer..."

# View config
cosmic-eyes-cli config
# Output: Placeholder configuration display
```

**Note:** CLI-to-applet D-Bus communication is not yet implemented, so commands won't actually control the running applet.

## Testing Configuration Changes

### 1. Edit Configuration

```bash
# Open config in editor
nano ~/.config/cosmic-eyes/config.ron

# Example: Change short break interval to 15 minutes
# Change: interval: 20 to interval: 15
```

### 2. Reload Configuration

Currently, you need to restart the applet:

```bash
# Kill running applet
pkill cosmic-eyes

# Restart COSMIC Panel (will restart applet)
pkill cosmic-panel
```

### 3. Verify Changes

Check that the new settings are loaded (would be visible when timer display is integrated).

## Expected Behavior (v0.1.0)

### ✅ What Works

1. **Applet appears in panel**
   - Icon shows in COSMIC Panel
   - Popup opens on hover/click

2. **Configuration system**
   - Config auto-creates on first run
   - Manual edits are saved and loaded
   - RON format parses correctly

3. **Manual break buttons**
   - Buttons are clickable
   - Timer service processes commands
   - State changes occur internally

4. **CLI commands parse**
   - All commands accept arguments correctly
   - Help text displays properly
   - Placeholder messages confirm command received

### 🚧 What's Pending Integration

1. **Timer display in popup**
   - Currently shows "Loading..."
   - Real-time countdown not yet displayed

2. **Automatic break triggering**
   - Timer logic works internally
   - Automatic display trigger pending

3. **Break screen**
   - Component exists but won't show automatically
   - Needs integration with applet state

4. **CLI-to-applet communication**
   - CLI commands don't yet control running applet
   - D-Bus interface needs implementation

5. **Idle detection**
   - Config exists but not monitoring system
   - System API integration needed

6. **Pre-break notifications**
   - Config exists but notifications not shown
   - Notification system pending

## Troubleshooting

### Applet Doesn't Appear in Panel

**Check if applet is running:**
```bash
ps aux | grep cosmic-eyes
```

**Check panel logs for errors:**
```bash
journalctl --user -u cosmic-panel -n 50
```

**Verify desktop entry:**
```bash
cat /usr/share/applications/cosmic-eyes.desktop
# OR
cat ~/.local/share/applications/cosmic-eyes.desktop

# Should have: X-CosmicApplet=true
```

**Restart panel:**
```bash
pkill cosmic-panel
```

### Build Fails with Missing Libraries

**Error: `xkbcommon.pc not found`**
```bash
# Install missing dependency
sudo apt install libxkbcommon-dev  # Debian/Ubuntu
sudo dnf install libxkbcommon-devel  # Fedora
```

**Error: `wayland protocols not found`**
```bash
# Install wayland dependencies
sudo apt install wayland-protocols libwayland-dev
```

### Popup Doesn't Open

**Check if icon is clickable:**
- Try clicking vs hovering
- Desktop entry has `X-CosmicHoverPopup=Auto`

**Check applet logs:**
```bash
# Run applet manually to see output
killall cosmic-eyes
cargo run --release --bin cosmic-eyes
```

### Configuration Not Loading

**Check config file exists:**
```bash
ls -la ~/.config/cosmic-eyes/
cat ~/.config/cosmic-eyes/config.ron
```

**Check for syntax errors:**
```bash
# Config should be valid RON format
# Copy from config.example.ron if corrupted
cp config.example.ron ~/.config/cosmic-eyes/config.ron
```

### Rust Version Too Old

```bash
# Update Rust
rustup update stable
rustup default stable

# Verify version
rustc --version  # Should be 1.70+
```

## Testing Checklist

Use this checklist to verify the build:

- [ ] System dependencies installed
- [ ] Rust toolchain 1.70+ installed
- [ ] Repository cloned successfully
- [ ] Project builds without errors (`cargo build --release`)
- [ ] Binaries created in `target/release/`
- [ ] Configuration example exists
- [ ] Desktop entry installed (system or local)
- [ ] Icon installed
- [ ] COSMIC Panel restarted
- [ ] Applet icon appears in panel
- [ ] Popup window opens on interaction
- [ ] "Short Break" and "Long Break" buttons visible
- [ ] Buttons are clickable
- [ ] Config file auto-created at `~/.config/cosmic-eyes/config.ron`
- [ ] CLI help command works (`cosmic-eyes-cli --help`)
- [ ] CLI commands accept arguments and show output
- [ ] Manual config edits persist after restart

## Providing Feedback

When testing, please note:

**What to report:**
- Build errors with full output
- Missing dependencies
- Panel crashes or errors
- Applet not appearing
- Unexpected behavior
- System configuration (distro, COSMIC version)

**What's expected (not bugs):**
- Timer showing "Loading..." (integration pending)
- CLI not controlling applet (D-Bus pending)
- Breaks not triggering automatically (integration pending)
- No idle detection (implementation pending)
- No notifications (implementation pending)

## Next Steps After Testing

If testing is successful:

1. **Report Results**: Note what worked vs issues encountered
2. **Check Logs**: Share any relevant error logs
3. **Test Customization**: Try modifying config values
4. **Development**: Consider contributing integration work!

## Clean Uninstall

To remove Cosmic Eyes:

```bash
# Remove binaries
sudo rm /usr/local/bin/cosmic-eyes
sudo rm /usr/local/bin/cosmic-eyes-cli

# Remove desktop entry
sudo rm /usr/share/applications/cosmic-eyes.desktop
rm ~/.local/share/applications/cosmic-eyes.desktop

# Remove icon
sudo rm /usr/share/icons/hicolor/scalable/apps/cosmic-eyes.svg
rm ~/.local/share/icons/hicolor/scalable/apps/cosmic-eyes.svg

# Remove configuration (optional)
rm -rf ~/.config/cosmic-eyes/

# Restart panel
pkill cosmic-panel
```

---

**Version:** v0.1.0 Testing Guide
**Last Updated:** 2025-11-17
**Status:** Alpha Architecture Release
