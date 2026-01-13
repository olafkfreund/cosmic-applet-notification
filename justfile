# justfile - Build automation for COSMIC Notification Applet

# Default recipe (runs when you just type `just`)
default:
    @just --list

# Build the project in debug mode
build:
    cargo build

# Build in release mode with optimizations
build-release:
    cargo build --release

# Run the applet in debug mode
run:
    cargo run

# Run in release mode
run-release:
    cargo run --release

# Run with trace-level logging
run-trace:
    RUST_LOG=trace cargo run

# Run with debug-level logging
run-debug:
    RUST_LOG=debug cargo run

# Run all tests
test:
    cargo test

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# Run integration tests only
test-integration:
    cargo test --test '*'

# Run clippy linter
check:
    cargo clippy --all-targets -- -D warnings

# Format code with rustfmt
fmt:
    cargo fmt --all

# Check formatting without making changes
fmt-check:
    cargo fmt --all -- --check

# Run all checks (format, clippy, tests)
check-all: fmt-check check test

# Clean build artifacts
clean:
    cargo clean
    rm -rf target/

# Generate and open documentation
doc:
    cargo doc --no-deps --open

# Install to system (requires sudo for /usr/local)
install PREFIX="/usr/local":
    #!/usr/bin/env bash
    set -euo pipefail
    
    echo "Installing COSMIC Notification Applet to {{PREFIX}}..."
    
    # Build release version
    cargo build --release
    
    # Create directories
    install -Dm755 target/release/cosmic-applet-notifications \
        "{{PREFIX}}/bin/cosmic-applet-notifications"
    
    # Install desktop entry
    install -Dm644 data/com.system76.CosmicAppletNotifications.desktop \
        "{{PREFIX}}/share/applications/com.system76.CosmicAppletNotifications.desktop"
    
    # Install icon
    install -Dm644 data/icons/com.system76.CosmicAppletNotifications.svg \
        "{{PREFIX}}/share/icons/hicolor/scalable/apps/com.system76.CosmicAppletNotifications.svg"
    
    echo "✓ Installation complete!"
    echo "Restart COSMIC panel to see the applet: pkill cosmic-panel && cosmic-panel"

# Uninstall from system
uninstall PREFIX="/usr/local":
    #!/usr/bin/env bash
    echo "Uninstalling COSMIC Notification Applet from {{PREFIX}}..."
    rm -f "{{PREFIX}}/bin/cosmic-applet-notifications"
    rm -f "{{PREFIX}}/share/applications/com.system76.CosmicAppletNotifications.desktop"
    rm -f "{{PREFIX}}/share/icons/hicolor/scalable/apps/com.system76.CosmicAppletNotifications.svg"
    echo "✓ Uninstallation complete!"

# Profile with flamegraph
flamegraph:
    cargo flamegraph

# Watch for changes and rebuild
watch:
    cargo watch -x build

# Watch and run tests on changes
watch-test:
    cargo watch -x test

# Send test notifications for development
test-notifications:
    #!/usr/bin/env bash
    echo "Sending test notifications..."
    
    notify-send "Test 1" "Simple notification"
    sleep 1
    
    notify-send -u low "Test 2" "Low urgency"
    sleep 1
    
    notify-send -u critical "Test 3" "Critical urgency"
    sleep 1
    
    notify-send -A "open=Open" "Test 4" "With action button"
    sleep 1
    
    for i in {1..5}; do
        notify-send "Rapid $i" "Testing rapid notifications"
    done
    
    echo "✓ Test notifications sent!"

# Monitor D-Bus notifications
dbus-monitor:
    dbus-monitor "interface='org.freedesktop.Notifications'"

# Check D-Bus server info
dbus-info:
    dbus-send --print-reply --dest=org.freedesktop.Notifications \
        /org/freedesktop/Notifications \
        org.freedesktop.Notifications.GetServerInformation

# Restart COSMIC panel
restart-panel:
    pkill cosmic-panel || true
    cosmic-panel &

# View applet logs
logs:
    journalctl --user -u cosmic-panel -f

# Update dependencies
update:
    cargo update

# Check for outdated dependencies
outdated:
    cargo outdated

# Security audit
audit:
    cargo audit

# Benchmark
bench:
    cargo bench

# Setup development environment
setup:
    #!/usr/bin/env bash
    echo "Setting up development environment..."
    
    # Create necessary directories
    mkdir -p src/{dbus,manager,ui/widgets,config}
    mkdir -p tests
    mkdir -p data/icons
    
    # Create .envrc if it doesn't exist
    if [ ! -f .envrc ]; then
        echo "use flake" > .envrc
        echo "Created .envrc file. Run 'direnv allow' to enable."
    fi
    
    echo "✓ Setup complete!"

# Show current configuration
config:
    #!/usr/bin/env bash
    CONFIG_FILE="$HOME/.config/cosmic/com.system76.CosmicAppletNotifications/config.ron"
    if [ -f "$CONFIG_FILE" ]; then
        cat "$CONFIG_FILE"
    else
        echo "Configuration file not found: $CONFIG_FILE"
    fi

# Reset configuration to defaults
config-reset:
    rm -f "$HOME/.config/cosmic/com.system76.CosmicAppletNotifications/config.ron"
    echo "Configuration reset. Restart the applet to regenerate defaults."

# Run pre-commit checks
pre-commit: fmt-check check test
    @echo "✓ All pre-commit checks passed!"
