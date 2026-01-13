# Development Guide

This guide covers the development workflow, testing strategies, and best practices for the COSMIC Notification Applet.

## Quick Start

### Prerequisites

- NixOS with flakes enabled
- Git
- COSMIC Desktop installed
- Basic Rust knowledge

### Setup Development Environment

```bash
# Clone the repository
git clone <repository-url>
cd cosmic-notification-applet

# Enter development shell
nix develop

# Build the project
just build

# Run in development mode
just run
```

## Development Workflow

### Daily Development Loop

```bash
# 1. Start development shell
nix develop

# 2. Make changes to code

# 3. Check your changes
just check    # Run clippy linter

# 4. Run tests
just test

# 5. Build and run
just run

# 6. Format code before committing
just fmt
```

### Using direnv (Recommended)

For automatic environment activation:

```bash
# Create .envrc file
echo "use flake" > .envrc

# Allow direnv
direnv allow

# Now the environment activates automatically when you cd into the directory
```

## Building

### Development Build

```bash
just build
# Or manually:
cargo build
```

### Release Build

```bash
just build-release
# Or manually:
cargo build --release
```

### Nix Build

```bash
# Build with Nix (reproducible)
nix build

# Result will be in ./result/
ls -la result/bin/
```

## Running

### Run Directly

```bash
just run
# Or:
cargo run
```

### Run with Logging

```bash
# Trace level (verbose)
RUST_LOG=trace just run

# Debug level
RUST_LOG=debug just run

# Specific module only
RUST_LOG=cosmic_notification_applet::dbus=debug just run
```

### Run in COSMIC Panel

After installing:

```bash
# Kill existing panel
pkill cosmic-panel

# Start panel (will launch your applet)
cosmic-panel
```

## Testing

### Unit Tests

```bash
just test
# Or:
cargo test
```

### Integration Tests

```bash
# Run integration tests only
cargo test --test '*'
```

### Test with Real Notifications

```bash
# Simple notification
notify-send "Test" "This is a test notification"

# Notification with action
notify-send -A "action1=Click Me" "Test" "Click the button"

# Critical urgency
notify-send -u critical "Critical" "Important message"

# With app icon
notify-send -i firefox "Firefox" "Download complete"

# Custom timeout (milliseconds)
notify-send -t 10000 "Long" "This stays for 10 seconds"
```

### D-Bus Testing

Monitor D-Bus notifications:

```bash
# Watch all notification signals
dbus-monitor "interface='org.freedesktop.Notifications'"

# Test if applet is receiving
dbus-send --print-reply --dest=org.freedesktop.Notifications \
  /org/freedesktop/Notifications \
  org.freedesktop.Notifications.GetServerInformation
```

## Code Quality

### Linting

```bash
just check
# Or:
cargo clippy --all-targets -- -D warnings
```

### Formatting

```bash
just fmt
# Or:
cargo fmt --all
```

### Check Everything

```bash
# Run all checks before committing
just check-all
```

## Debugging

### Enable Debug Logging

```rust
// In src/main.rs
tracing_subscriber::fmt()
    .with_env_filter("cosmic_notification_applet=trace")
    .init();
```

### Use tracing Macros

```rust
use tracing::{debug, info, warn, error, trace};

trace!("Detailed trace information");
debug!("Debug message: {}", value);
info!("General information");
warn!("Warning message");
error!("Error occurred: {:?}", error);
```

### GDB Debugging

```bash
# Build with debug symbols
cargo build

# Run with GDB
gdb --args target/debug/cosmic-applet-notifications
```

### Performance Profiling

```bash
# Install flamegraph (if not in nix shell)
cargo install flamegraph

# Profile the application
just flamegraph

# View flamegraph.svg in browser
firefox flamegraph.svg
```

## Testing Strategies

### Test Notification Reception

Create a test script `test-notifications.sh`:

```bash
#!/usr/bin/env bash

echo "Testing notification reception..."

# Test 1: Simple notification
notify-send "Test 1" "Simple notification"
sleep 2

# Test 2: With actions
notify-send -A "open=Open" -A "dismiss=Dismiss" "Test 2" "With actions"
sleep 2

# Test 3: Different urgencies
notify-send -u low "Test 3" "Low urgency"
notify-send -u normal "Test 3" "Normal urgency"
notify-send -u critical "Test 3" "Critical urgency"
sleep 2

# Test 4: Rapid notifications
for i in {1..10}; do
    notify-send "Rapid $i" "Testing rapid notifications"
done
sleep 2

# Test 5: Long text
notify-send "Test 5" "$(head -c 1000 < /dev/urandom | base64)"
sleep 2

echo "Test complete!"
```

```bash
chmod +x test-notifications.sh
./test-notifications.sh
```

### Stress Testing

```bash
# Send 100 notifications rapidly
for i in {1..100}; do
    notify-send "Stress $i" "Message $i" &
done
```

## Installation

### Local Installation

```bash
just install
# Or with custom prefix:
just install PREFIX=$HOME/.local
```

### System Installation

```bash
sudo just install
```

### NixOS System Integration

Add to your NixOS configuration:

```nix
{ inputs, ... }:
{
  environment.systemPackages = [
    inputs.cosmic-notification-applet.packages.${pkgs.system}.default
  ];
}
```

## Updating Dependencies

### Update Rust Dependencies

```bash
# Update all dependencies
cargo update

# Update specific dependency
cargo update libcosmic

# Check for outdated dependencies
cargo outdated
```

### Update Nix Dependencies

```bash
# Update all flake inputs
nix flake update

# Update specific input
nix flake update rust-overlay

# Check what will be updated
nix flake update --dry-run
```

## Common Development Tasks

### Adding a New Feature

1. Create feature branch:
   ```bash
   git checkout -b feature/my-feature
   ```

2. Implement feature with tests

3. Run all checks:
   ```bash
   just check-all
   ```

4. Commit with conventional commit message:
   ```bash
   git commit -m "feat: add custom notification positioning"
   ```

### Fixing a Bug

1. Create bug fix branch:
   ```bash
   git checkout -b fix/issue-123
   ```

2. Write failing test that reproduces bug

3. Fix the bug

4. Verify test passes:
   ```bash
   just test
   ```

5. Commit:
   ```bash
   git commit -m "fix: resolve notification memory leak (fixes #123)"
   ```

### Refactoring

1. Ensure all tests pass:
   ```bash
   just test
   ```

2. Make changes

3. Verify tests still pass

4. Run clippy:
   ```bash
   just check
   ```

5. Commit:
   ```bash
   git commit -m "refactor: simplify notification manager"
   ```

## Troubleshooting

### Applet Not Appearing in Panel

1. Check desktop entry installed:
   ```bash
   ls /usr/share/applications/com.system76.CosmicAppletNotifications.desktop
   ```

2. Verify desktop entry has correct fields:
   ```bash
   grep "X-CosmicApplet" /usr/share/applications/*.desktop
   ```

3. Restart COSMIC panel:
   ```bash
   pkill cosmic-panel && cosmic-panel &
   ```

4. Check logs:
   ```bash
   journalctl --user -u cosmic-panel -f
   ```

### D-Bus Connection Issues

1. Verify D-Bus session bus running:
   ```bash
   echo $DBUS_SESSION_BUS_ADDRESS
   ```

2. Test D-Bus connection:
   ```bash
   dbus-send --session --print-reply \
     --dest=org.freedesktop.DBus \
     /org/freedesktop/DBus \
     org.freedesktop.DBus.ListNames
   ```

3. Check for conflicting notification daemons:
   ```bash
   ps aux | grep notification
   ```

### Build Errors

1. Clean build artifacts:
   ```bash
   just clean
   cargo clean
   ```

2. Update dependencies:
   ```bash
   cargo update
   ```

3. Rebuild from scratch:
   ```bash
   just build
   ```

### Runtime Errors

1. Enable debug logging:
   ```bash
   RUST_LOG=debug cargo run 2>&1 | tee debug.log
   ```

2. Check for panic messages

3. Verify configuration file:
   ```bash
   cat ~/.config/cosmic/com.system76.CosmicAppletNotifications/config.ron
   ```

## Performance Optimization

### Profiling

```bash
# CPU profiling
cargo flamegraph

# Memory profiling
valgrind --tool=massif target/debug/cosmic-applet-notifications

# Benchmark
cargo bench
```

### Optimization Checklist

- [ ] Use `Cow` for strings that might not need allocation
- [ ] Batch UI updates
- [ ] Limit notification history size
- [ ] Use efficient data structures
- [ ] Profile before optimizing
- [ ] Benchmark changes

## Documentation

### Generate API Documentation

```bash
cargo doc --no-deps --open
```

### Update Documentation

When adding features:
1. Update `ARCHITECTURE.md`
2. Add to `README.md` if user-facing
3. Update relevant skill files
4. Add doc comments to code

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Run all tests: `just check-all`
4. Tag release: `git tag v0.1.0`
5. Push: `git push --tags`
6. Build release: `nix build`
7. Test installation
8. Create GitHub release

## Getting Help

- Check `CLAUDE.md` for AI assistant guidance
- Review `ARCHITECTURE.md` for design decisions
- Read skill files in `docs/skills/`
- Check COSMIC documentation
- Ask in #cosmic:nixos.org Matrix channel

---

**Last Updated**: 2025-01-13
