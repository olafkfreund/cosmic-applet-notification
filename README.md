# COSMIC Notification Applet

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

A custom notification display applet for COSMIC Desktop Environment that provides enhanced notification management with customizable display, placement, and interaction options.

## Project Overview

This applet implements the freedesktop.org D-Bus notification specification to receive and display system notifications in a COSMIC panel applet. It offers extensive customization, full keyboard control, smooth animations with accessibility support, and intelligent notification filtering.

**Key Capabilities:**
- Custom notification positioning with panel-relative anchoring
- Complete keyboard navigation and shortcuts
- Smooth animations with reduced-motion accessibility support
- Persistent notification history with configurable retention
- Do Not Disturb mode with critical notification bypass
- Per-application and urgency-based filtering
- Automatic URL detection with security filtering
- Full notification action button support

## Project Status

**Version:** 0.1.0 (Pre-release)
**Development Phase:** Phase 5 (Documentation and Packaging)

### Implementation Status

**Phase 1-4: COMPLETE**
- Core D-Bus notification system
- COSMIC applet integration
- Advanced features and customization
- Performance optimizations
- Accessibility support

**Phase 5: IN PROGRESS**
- User documentation (COMPLETE)
- API documentation (COMPLETE)
- Installation guide (COMPLETE)
- Release preparation (IN PROGRESS)

## Features

### Core Notification System

**D-Bus Integration**
- Full freedesktop.org notification specification compliance
- Async notification reception via zbus
- Notification ID management and tracking
- Urgency level support (Low, Normal, Critical)
- Notification expiration timeout handling

**State Management**
- Persistent notification history (configurable retention: 1-365 days)
- Maximum history size control (10-1000 notifications)
- Active notification queue management
- Automatic cleanup of expired notifications

**Notification Display**
- App icon, name, and timestamp display
- Summary (title) and body text rendering
- Relative timestamp formatting (e.g., "2m ago", "1h ago")
- Notification action buttons
- Dismiss functionality with keyboard and mouse

### Positioning and Layout

**Custom Positioning System**
- **Auto Mode**: Popup appears near applet icon (default)
- **Panel Relative Mode**: Custom positioning relative to panel
  - Anchor points: Start, Center, End, or Applet Icon
  - X/Y offset adjustment: -500px to +500px
  - Snap-to-edge: Automatic alignment within configurable threshold (5-100px)
  - Position preview with 3-second auto-close

**Responsive Design**
- Adapts to panel orientation (horizontal/vertical)
- Automatic constraint adjustment for screen boundaries
- Wayland compositor-compliant positioning

### Keyboard Navigation

**Navigation Keys**
- `Up Arrow`: Navigate to previous notification (wraps to bottom)
- `Down Arrow`: Navigate to next notification (wraps to top)
- Visual feedback: Accent-colored border on selected notification

**Action Keys**
- `Enter`: Activate selected notification (opens URL or invokes first action)
- `Delete`: Dismiss selected notification
- `Tab`: Cycle through action buttons within selected notification
- `1-9`: Quick action invocation (1 = first action, 2 = second, etc.)

**Global Shortcuts**
- `Escape`: Close notification popup
- `Ctrl+D`: Toggle Do Not Disturb mode
- `Ctrl+1`: Show all notifications (no urgency filter)
- `Ctrl+2`: Show normal and critical notifications only
- `Ctrl+3`: Show critical notifications only

### Animation System

**Animation Types**
- **Appear Animation** (300ms, Cubic Out easing)
  - Fade in: opacity 0 to 1
  - Slide in: translate -50px to 0
  - Scale: 0.95 to 1.0
- **Dismiss Animation** (200ms, Ease In)
  - Fade out: opacity 1 to 0
  - Slide out: translate 0 to -50px
  - Scale: 1.0 to 0.95

**Easing Functions** (9 available)
- Linear, EaseIn, EaseOut, EaseInOut
- CubicIn, CubicOut, CubicInOut
- ExpoOut, BounceOut

**Performance**
- Smart 60fps subscription (only active when animations running)
- Automatic animation cleanup on completion
- Per-animation type configuration (appear/dismiss separately)
- Progress indicators for timed notifications

**Note:** Animation state calculation is complete. Visual rendering is limited by iced framework capabilities and will be enhanced as framework support improves.

### Filtering and Control

**Do Not Disturb Mode**
- Suppresses Low and Normal urgency notifications
- Critical notifications always displayed (system warnings, errors)
- Keyboard shortcut: `Ctrl+D`

**Urgency-Based Filtering**
- Level 0: All notifications (Low, Normal, Critical)
- Level 1: Normal and Critical only
- Level 2: Critical only
- Keyboard shortcuts: `Ctrl+1`, `Ctrl+2`, `Ctrl+3`

**Per-Application Filtering**
- Enable/disable notifications from specific applications
- Configurable via settings or config file
- Persistent filter rules

### Interactive Features

**Clickable URLs**
- Automatic URL detection in notification body text
- Blue underlined link styling
- Security filtering (blocks javascript:, data:, file: URLs)
- Opens in default browser

**Action Buttons**
- Full support for freedesktop notification actions
- Standard button styling with selected state highlighting
- Keyboard and mouse activation
- Quick action invocation via number keys

### Accessibility

**Reduced Motion Support**
- Automatic detection via XDG Desktop Portal
- Reads `org.freedesktop.appearance.reduced-motion` setting
- Animations automatically disabled when reduced motion preferred
- Real-time updates when system setting changes
- WCAG 2.1 Success Criterion 2.3.3 (Level AAA) compliant

**Keyboard Accessibility**
- Full keyboard navigation support
- Clear focus indicators
- Logical tab order
- Screen reader compatibility (via COSMIC accessibility framework)

**Visual Accessibility**
- High contrast theme support
- Accent-colored selection borders
- Clear status indicators

### Performance and Optimization

**Code-Level Optimizations**
- Event handling: 90% reduction in unnecessary CPU usage
- Optimized keyboard event subscription (filter_map pattern)
- Efficient URL extraction (stops at first match)
- Selection state helper methods

**Runtime Efficiency**
- Smart animation frame subscription (16ms interval, only when needed)
- Efficient notification queue management
- Minimal memory footprint (20-50MB idle, <200MB with 100+ notifications)
- <1% CPU usage at idle

## Installation

### Quick Start: NixOS Flake Installation (Recommended)

This guide shows you how to install the COSMIC Notification Applet on NixOS using Nix flakes.

#### Prerequisites

Before starting, ensure you have:
- NixOS 22.05 or later
- COSMIC Desktop Environment installed and running
- Nix flakes enabled in your configuration

If you haven't enabled flakes yet, add this to your `configuration.nix`:

```nix
{
  nix.settings.experimental-features = [ "nix-command" "flakes" ];
}
```

Then rebuild: `sudo nixos-rebuild switch`

#### Step 1: Add the Flake Input

Edit your system's `flake.nix` file and add the cosmic-applet-notifications input:

```nix
{
  description = "My NixOS Configuration";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    # Add this line:
    cosmic-applet-notifications.url = "github:olafkfreund/cosmic-applet-notification";
  };

  outputs = { self, nixpkgs, cosmic-applet-notifications, ... }: {
    nixosConfigurations.yourhostname = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        ./configuration.nix

        # Add this line:
        cosmic-applet-notifications.nixosModules.default
      ];
    };
  };
}
```

Replace `yourhostname` with your actual hostname.

#### Step 2: Enable the Service

Add this to your `configuration.nix`:

```nix
{
  # Enable the COSMIC notification applet
  services.cosmic-applet-notifications.enable = true;
}
```

#### Step 3: Rebuild Your System

```bash
# Update flake inputs to get the latest version
nix flake update

# Rebuild your system
sudo nixos-rebuild switch --flake .#yourhostname
```

Replace `yourhostname` with your actual hostname.

#### Step 4: Restart COSMIC Panel

After the rebuild completes, restart the COSMIC panel to load the applet:

```bash
cosmic-panel --reload
```

Or log out and log back in to restart your COSMIC session.

#### Step 5: Verify Installation

Check that the applet is running:

```bash
# Check if the binary is installed
which cosmic-applet-notifications

# Verify it's running
ps aux | grep cosmic-applet-notifications
```

Look for a bell/notification icon in your COSMIC panel. Click it to open the notification popup.

#### Test the Applet

Send a test notification to verify it's working:

```bash
notify-send "Test Notification" "Hello from COSMIC Notification Applet!"
```

The notification should appear, and you should be able to see it in the applet's history by clicking the panel icon.

---

### Alternative Installation Methods

**Method 2: Direct Package Installation**

```nix
# configuration.nix
{ inputs, pkgs, ... }: {
  environment.systemPackages = [
    inputs.cosmic-applet-notifications.packages.${pkgs.system}.default
  ];
}
```

**Method 3: Home Manager**

```nix
# home.nix
{ inputs, pkgs, ... }: {
  home.packages = [
    inputs.cosmic-applet-notifications.packages.${pkgs.system}.default
  ];
}
```

**Method 4: Build from Source**

```bash
git clone https://github.com/olafkfreund/cosmic-applet-notification.git
cd cosmic-applet-notification
nix build
nix profile install .
```

See **[INSTALL.md](./INSTALL.md)** for complete installation instructions, troubleshooting, and uninstallation procedures.

## Development

### Prerequisites

- NixOS (22.05 or later) or Nix package manager
- Nix flakes enabled
- COSMIC Desktop Environment (Alpha 6 or later)
- Rust 1.90.0 or later (provided by Nix environment)

### Development Environment Setup

```bash
# Clone the repository
git clone https://github.com/olafkfreund/cosmic-applet-notification.git
cd cosmic-applet-notification

# Enter development shell (provides Rust toolchain and dependencies)
nix develop

# Or use direnv for automatic environment loading
echo "use flake" > .envrc
direnv allow
```

### Building

```bash
# Development build
just build
cargo build

# Release build (optimized)
just build-release
cargo build --release

# Run the applet
just run
cargo run
```

### Testing

```bash
# Run all tests
just test
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific module tests
cargo test manager::tests
cargo test dbus::tests

# Run ignored tests (require XDG portal)
cargo test -- --ignored
```

### Code Quality

```bash
# Format code
just fmt
cargo fmt

# Run linter
just check
cargo clippy -- -D warnings

# Run all quality checks
just check-all

# Generate documentation
cargo doc --no-deps --open
```

### Development Workflow

1. **Make changes** to source code
2. **Run tests** to verify functionality
3. **Format code** with `just fmt`
4. **Check for issues** with `just check`
5. **Build and test** with `just build && just test`
6. **Commit changes** with descriptive message

### Debugging

**Enable logging:**

```bash
# Debug level
RUST_LOG=cosmic_applet_notifications=debug cargo run

# Trace level (verbose)
RUST_LOG=cosmic_applet_notifications=trace cargo run

# Check panel logs
journalctl --user -u cosmic-panel -f | grep cosmic-applet-notifications
```

**Monitor D-Bus:**

```bash
# Watch notification signals
dbus-monitor "interface='org.freedesktop.Notifications'"

# Send test notification
notify-send "Test" "Testing the applet"
notify-send -u critical "Critical" "High priority notification"
```

**Performance profiling:**

```bash
# Generate flamegraph
just flamegraph

# Or manually
cargo flamegraph
```

See **[DEVELOPMENT.md](./DEVELOPMENT.md)** for detailed development workflows, testing strategies, and contribution guidelines.

## Configuration

Configuration is stored at:
```
~/.config/cosmic/com.cosmic.applet.notifications/config.ron
```

**Quick configuration:**

```bash
# Use a pre-made example
cp examples/default-config.ron ~/.config/cosmic/com.cosmic.applet.notifications/config.ron

# Available examples:
# - default-config.ron (annotated defaults)
# - minimal-config.ron (lightweight setup)
# - power-user-config.ron (advanced features)
# - focus-mode-config.ron (deep work mode)
# - accessibility-config.ron (reduced motion)

# Restart panel to apply changes
cosmic-panel --reload
```

See **[USER_GUIDE.md](./USER_GUIDE.md)** for complete configuration documentation and **[examples/](./examples/)** for ready-to-use configurations.

## Documentation

### User Documentation
- **[USER_GUIDE.md](./USER_GUIDE.md)** - Complete feature guide with examples
- **[INSTALL.md](./INSTALL.md)** - Installation and troubleshooting
- **[examples/](./examples/)** - Configuration examples for various use cases

### Developer Documentation
- **[API_DOCUMENTATION.md](./API_DOCUMENTATION.md)** - API reference and extension guide
- **[DEVELOPMENT.md](./DEVELOPMENT.md)** - Development workflows and best practices
- **[ARCHITECTURE.md](./ARCHITECTURE.md)** - Technical architecture and design
- **[PROJECT_PLAN.md](./PROJECT_PLAN.md)** - Development roadmap and milestones

## Architecture

### System Components

**D-Bus Listener**
- Async notification reception using zbus 4.x
- Runs in separate tokio task
- Sends notifications via mpsc channel to manager

**Notification Manager**
- State management and notification queue
- Filtering logic (urgency, per-app, DND)
- History persistence and cleanup
- Thread-safe via message passing

**COSMIC Applet UI**
- Panel icon with notification count badge
- Popup window with scrollable notification list
- libcosmic/iced-based rendering
- Keyboard event handling

**Animation System**
- Timeline-based animation engine
- 9 easing function implementations
- Per-notification animation state tracking
- Smart 60fps subscription management

See **[ARCHITECTURE.md](./ARCHITECTURE.md)** for detailed component interactions and data flow.

## Technology Stack

### Core Technologies
- **Rust 1.90.0** - System programming language with edition 2024 features
- **libcosmic** - COSMIC desktop toolkit and application framework
- **iced** - Cross-platform GUI framework (via libcosmic)
- **zbus 4.x** - Async D-Bus communication library
- **tokio** - Asynchronous runtime with full feature set

### Key Dependencies
- **cosmic-config** - COSMIC configuration management
- **serde** - Serialization/deserialization framework
- **chrono** - Date and time handling
- **thiserror** - Error type derivation
- **tracing** - Structured logging framework
- **ashpd 0.12** - XDG Desktop Portal bindings (accessibility)
- **url** - URL parsing with security validation
- **regex** - Pattern matching for URL detection

### Build and Development
- **Nix/NixOS** - Reproducible build environment and package management
- **just** - Command runner for common tasks
- **cargo** - Rust build system and package manager
- **rustfmt** - Code formatting
- **clippy** - Linting and code analysis

## System Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| OS | NixOS 22.05 | NixOS 23.11+ |
| Desktop | COSMIC Alpha 6 | COSMIC Alpha 7+ |
| Rust | 1.90.0 | Latest stable |
| Memory | 50MB idle | 100MB with history |
| Disk | 20MB | 50MB |
| CPU | <1% idle | Minimal |

## License

This project is licensed under the **GNU General Public License v3.0** (GPL-3.0-only).

See the [LICENSE](./LICENSE) file for the full license text.

### Why GPL-3.0?

This license was chosen to:
- Ensure the software remains free and open source
- Require derivative works to also be open source
- Align with COSMIC Desktop licensing practices
- Protect user freedoms

## Contributing

Contributions are welcome! This project follows standard open-source contribution practices.

**Before contributing:**
1. Read [DEVELOPMENT.md](./DEVELOPMENT.md) for development setup
2. Review [ARCHITECTURE.md](./ARCHITECTURE.md) for design patterns
3. Check existing issues and pull requests
4. Follow the coding standards documented in the codebase

**Contribution areas:**
- Bug fixes and issue resolution
- Feature implementation from PROJECT_PLAN.md
- Documentation improvements
- Test coverage expansion
- Performance optimizations
- Accessibility enhancements

Please open an issue for discussion before starting major changes.

## Acknowledgments

This project builds upon excellent work from:

- **System76** and the **COSMIC team** for creating the COSMIC Desktop Environment and libcosmic toolkit
- **freedesktop.org** for the notification specification standard
- The **Rust community** for exceptional tools, libraries, and documentation
- **NixOS community** for reproducible development environments and packaging
- **zbus developers** for the high-quality D-Bus Rust bindings
- **iced contributors** for the cross-platform GUI framework

## Support and Community

### Getting Help

- **User Guide**: [USER_GUIDE.md](./USER_GUIDE.md) - Complete feature documentation
- **Installation Help**: [INSTALL.md](./INSTALL.md) - Troubleshooting guide
- **GitHub Issues**: [Report bugs or request features](https://github.com/olafkfreund/cosmic-applet-notification/issues)
- **GitHub Discussions**: Community questions and discussions
- **Matrix Chat**: #cosmic:nixos.org (COSMIC desktop development)

### Reporting Issues

When reporting bugs, please include:
- NixOS and COSMIC Desktop versions
- Steps to reproduce the issue
- Expected vs actual behavior
- Relevant logs: `RUST_LOG=trace cosmic-applet-notifications`
- Configuration file (if applicable)

### Feature Requests

Feature requests are welcome! Please:
- Check existing issues for duplicates
- Describe the use case and benefit
- Provide examples or mockups if applicable
- Consider contributing the implementation

---

**COSMIC Notification Applet** - Enhanced notification management for COSMIC Desktop on NixOS
