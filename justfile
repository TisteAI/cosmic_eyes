# Build and run commands for cosmic-eyes

# Default recipe: build release version
default: build-release

# Build the applet in release mode
build-release:
    cargo build --release

# Build in debug mode
build:
    cargo build

# Run the applet in debug mode
run:
    cargo run --bin cosmic-eyes

# Run the CLI in debug mode
run-cli *ARGS:
    cargo run --bin cosmic-eyes-cli -- {{ARGS}}

# Install binaries to ~/.cargo/bin
install-bins:
    cargo install --path . --locked

# Install system files (requires sudo)
install-system:
    install -Dm644 res/cosmic-eyes.desktop /usr/share/applications/cosmic-eyes.desktop
    install -Dm644 res/icons/hicolor/scalable/apps/cosmic-eyes.svg /usr/share/icons/hicolor/scalable/apps/cosmic-eyes.svg

# Full install (run install-bins, then sudo just install-system)
install: install-bins
    @echo "Binaries installed to ~/.cargo/bin"
    @echo "Now run: sudo just install-system"

# Run clippy linter
check:
    cargo clippy --all-features

# Format code
fmt:
    cargo fmt

# Run tests
test:
    cargo test

# Clean build artifacts
clean:
    cargo clean

# Create vendor archive for packaging
vendor:
    mkdir -p .cargo
    cargo vendor | head -n -1 > .cargo/config
    echo 'directory = "vendor"' >> .cargo/config
    tar pcfJ vendor.tar.xz vendor
    rm -rf vendor

# Watch and rebuild on changes
watch:
    cargo watch -x 'build --bin cosmic-eyes'
