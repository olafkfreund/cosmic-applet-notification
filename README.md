# COSMIC Notification Applet

[![Rust CI](https://github.com/olafkfreund/cosmic-applet-notification/workflows/Rust%20CI/badge.svg)](https://github.com/olafkfreund/cosmic-applet-notification/actions/workflows/rust-ci.yml)
[![Nix Build](https://github.com/olafkfreund/cosmic-applet-notification/workflows/Nix%20Build/badge.svg)](https://github.com/olafkfreund/cosmic-applet-notification/actions/workflows/nix-build.yml)
[![codecov](https://codecov.io/gh/olafkfreund/cosmic-applet-notification/branch/main/graph/badge.svg)](https://codecov.io/gh/olafkfreund/cosmic-applet-notification)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

A custom notification display applet for COSMIC Desktop Environment that provides enhanced notification management with customizable display, placement, and interaction options.

## Project Overview

This applet listens to the freedesktop.org D-Bus notification interface and displays notifications in a customizable panel applet with features including:

- **Larger notification displays** with better readability
- **Customizable placement** - position notifications anywhere on screen
- **Enhanced interaction** - click to open URLs, view full messages
- **Notification history** - review past notifications
- **Flexible styling** - match your COSMIC theme or customize further
- **Action button support** - interact with notification actions directly

## Project Status

✅ **Phase 1-3 Complete** - Core functionality implemented
🚧 **Phase 4-5 In Progress** - Polish, optimization, and packaging

### Completed Features
- ✅ D-Bus notification listener
- ✅ Notification manager with state management
- ✅ COSMIC applet integration with panel icon
- ✅ Popup window with notification list
- ✅ Configuration system with COSMIC Config
- ✅ Notification history with persistence
- ✅ Clickable URL support in notification bodies
- ✅ Action button support for notification interactions
- ✅ Per-application notification filtering
- ✅ Keyboard navigation and shortcuts
- ✅ Comprehensive test suite (100+ tests)
- ✅ NixOS package derivation
- ✅ CI/CD with GitHub Actions

### In Progress
- 🚧 Performance optimization and profiling
- 🚧 User documentation
- 🚧 Notification animations
- 🚧 Custom positioning options

## Quick Start

### Prerequisites

- NixOS with flakes enabled
- COSMIC Desktop Environment installed
- Basic Rust knowledge

### Development Setup

```bash
# Clone the repository
git clone <your-repo-url>
cd cosmic-notification-applet

# Enter development environment
nix develop

# Build the project
just build

# Run in development mode
just run
```

See [DEVELOPMENT.md](./DEVELOPMENT.md) for detailed development workflows.

## Features

### Core Functionality
- **Notification Display**: Receive and display system notifications via D-Bus
- **History Management**: Persistent notification history with configurable retention
- **Interactive Notifications**: Clickable URLs and action buttons
- **Smart Filtering**: Per-application filters and urgency-based filtering
- **Do Not Disturb**: DND mode with critical notification bypass

### User Interface
- **COSMIC Integration**: Native panel applet with COSMIC design language
- **Popup Window**: Expandable notification list with smooth interactions
- **Keyboard Shortcuts**: Full keyboard navigation support
  - `Escape`: Close popup
  - `Ctrl+D`: Toggle Do Not Disturb
  - `Ctrl+1/2/3`: Set urgency filter level (All/Normal+/Critical only)
- **Visual Feedback**: Clear status indicators and responsive design

### Configuration
- **Flexible Settings**: Extensive configuration options via COSMIC Config
- **App Filters**: Enable/disable notifications per application
- **History Settings**: Configurable history size and retention period
- **Display Options**: Customize notification appearance and behavior

### Advanced Features
- **URL Detection**: Automatic detection and clickable links in notification text
- **Security**: Blocked javascript:, data:, and file: URLs for safety
- **Action Support**: Full support for notification actions (reply, dismiss, etc.)
- **Urgency Levels**: Respect and filter by Low/Normal/Critical urgency

## Documentation

- **[PROJECT_PLAN.md](./PROJECT_PLAN.md)** - Overall project roadmap and milestones
- **[ARCHITECTURE.md](./ARCHITECTURE.md)** - Technical architecture and design decisions
- **[DEVELOPMENT.md](./DEVELOPMENT.md)** - Development workflows and best practices
- **[CLAUDE.md](./CLAUDE.md)** - AI assistant context and project knowledge
- **[docs/skills/](./docs/skills/)** - Specialized knowledge for development aspects

## Architecture

The applet consists of three main components:

1. **D-Bus Listener** - Subscribes to org.freedesktop.Notifications signals
2. **COSMIC Applet UI** - Panel integration with libcosmic
3. **Notification Manager** - State management and display logic

See [ARCHITECTURE.md](./ARCHITECTURE.md) for detailed technical design.

## Technology Stack

- **Rust** - Primary programming language
- **libcosmic** - COSMIC desktop toolkit
- **zbus** - D-Bus communication
- **iced** - GUI framework (via libcosmic)
- **tokio** - Async runtime
- **Nix** - Build and development environment

## NixOS Integration

This project includes comprehensive NixOS support:

- Flake-based development environment
- Reproducible builds
- Easy installation on NixOS systems
- Integration with COSMIC desktop configuration

See `flake.nix` for details.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## License

[Specify your license here - GPL-3.0 is common for COSMIC projects]

## Acknowledgments

- System76 and the COSMIC team for the excellent desktop environment
- The Rust community for amazing tools and libraries
- NixOS community for reproducible development environments

## Support

- **Issues**: [GitHub Issues](your-repo/issues)
- **Discussions**: [GitHub Discussions](your-repo/discussions)
- **Matrix**: #cosmic:nixos.org (COSMIC development)

---

Built with ❤️ for COSMIC Desktop on NixOS
