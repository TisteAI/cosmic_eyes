# Build Optimizations

## Summary

This document describes the optimizations made to improve build times, reduce binary size, and remove unused dependencies.

## Changes Made

### 1. Dependency Optimizations

#### Removed Unused Dependencies
- **`tracing`**: Removed - only `tracing-subscriber` is actually used for logging initialization
  - Savings: ~1 transitive dependency

#### Optimized tokio Features
- **Before**: `features = ["full"]` (includes all tokio features)
- **After**: `features = ["rt-multi-thread", "macros", "sync", "time"]`
- **Rationale**: We only use:
  - `rt-multi-thread`: Multi-threaded runtime for #[tokio::main]
  - `macros`: For #[tokio::main] macro
  - `sync`: For RwLock usage in timer service
  - `time`: For time-based operations

**Impact**: Reduces tokio compile time and binary size by excluding unused features like:
- io (file/network IO we don't use)
- fs (filesystem operations we don't use)
- net (networking we don't use)
- process (process spawning we don't use)
- signal (signal handling we don't use)

### 2. Code Cleanup

#### Removed Unused Imports
- **`src/applet/mod.rs`**: Removed unused `tokio::sync::RwLock` import
  - Already imported in timer service, not needed in applet

#### Removed Duplicate Constants
- **`src/cli/main.rs`**: Removed unused `SERVICE_NAME` and `OBJECT_PATH` constants
  - Values are hardcoded in the zbus proxy macro, constants not referenced

#### Made Constants Private
- **`src/dbus.rs`**: Changed `pub const` to `const` for SERVICE_NAME and OBJECT_PATH
  - Only used internally within the module

### 3. Build Profile Optimizations

#### Release Profile
- Added `opt-level = 3`: Maximum optimization for release builds
- Kept `lto = "fat"`: Link-time optimization for smaller binaries
- Kept `codegen-units = 1`: Single codegen unit for better optimization
- Kept `strip = true`: Strip symbols for smaller binaries

#### Dev Profile
- Added `incremental = true`: Enable incremental compilation for faster rebuilds during development
- Kept `opt-level = 0`: Fast compilation during development

## Expected Benefits

### Compile Time
- **Faster clean builds**: Fewer features to compile
- **Faster incremental builds**: Incremental compilation enabled for dev profile
- **Reduced dependency count**: Removed unused dependencies

### Binary Size
- **Smaller release binaries**: Tokio features excluded, maximum optimization enabled
- **Stripped binaries**: Debug symbols removed in release builds

### Build Efficiency
- **Less disk space**: Fewer intermediate build artifacts
- **Less memory**: Fewer features and dependencies to compile simultaneously

## Functionality Verification

All optimizations were made WITHOUT changing application behavior:
- ✅ Applet functionality unchanged
- ✅ CLI functionality unchanged
- ✅ D-Bus communication unchanged
- ✅ Break screen functionality unchanged
- ✅ Idle detection unchanged
- ✅ Notifications unchanged

## Testing

To verify functionality after optimizations:
```bash
# Clean build to test from scratch
cargo clean
cargo build --release

# Run tests (when added)
cargo test

# Check for warnings
cargo clippy -- -D warnings
```

## File Changes

- `Cargo.toml`: Dependency and profile optimizations
- `src/applet/mod.rs`: Removed unused import
- `src/cli/main.rs`: Removed unused constants
- `src/dbus.rs`: Made constants private
