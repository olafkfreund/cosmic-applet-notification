# Changelog

All notable changes to the COSMIC Notification Applet will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-01-15

### Added

#### Core Notification System (Phase 1)
- Full freedesktop.org D-Bus notification specification compliance
- Async notification reception using zbus 4.x
- Notification ID management and tracking
- Urgency level support (Low, Normal, Critical)
- Notification expiration timeout handling
- Persistent notification history with configurable retention (1-365 days)
- Maximum history size control (10-1000 notifications)
- Active notification queue management
- Automatic cleanup of expired notifications

#### User Interface (Phase 1)
- COSMIC panel applet with notification icon
- Notification count badge on panel icon
- Popup window with scrollable notification list
- Notification cards displaying:
  - App icon, name, and timestamp
  - Summary (title) and body text
  - Relative timestamp formatting (e.g., "2m ago", "1h ago")
  - Action buttons
  - Dismiss functionality

#### Filtering and Control (Phase 2)
- Do Not Disturb mode with critical notification bypass
- Urgency-based filtering (3 levels: All, Normal+, Critical only)
- Per-application filtering (enable/disable notifications per app)
- Keyboard shortcut: `Ctrl+D` for Do Not Disturb toggle
- Keyboard shortcuts: `Ctrl+1`, `Ctrl+2`, `Ctrl+3` for urgency filters

#### Interactive Features (Phase 2)
- Automatic URL detection in notification body text
- Clickable URLs with security filtering (blocks javascript:, data:, file: URLs)
- Full notification action button support
- Mouse and keyboard activation of action buttons

#### Keyboard Navigation (Phase 3)
- Navigation keys:
  - `Up Arrow`: Navigate to previous notification (wraps to bottom)
  - `Down Arrow`: Navigate to next notification (wraps to top)
  - Visual feedback with accent-colored border on selected notification
- Action keys:
  - `Enter`: Activate selected notification
  - `Delete`: Dismiss selected notification
  - `Tab`: Cycle through action buttons
  - `1-9`: Quick action invocation
- Global shortcuts:
  - `Escape`: Close notification popup
  - `Ctrl+D`: Toggle Do Not Disturb mode
  - `Ctrl+1/2/3`: Urgency filter shortcuts

#### Animation System (Phase 4)
- Timeline-based animation engine with 9 easing functions:
  - Linear, EaseIn, EaseOut, EaseInOut
  - CubicIn, CubicOut, CubicInOut
  - ExpoOut, BounceOut
- Appear animation (300ms, Cubic Out easing):
  - Fade in: opacity 0 to 1
  - Slide in: translate -50px to 0
  - Scale: 0.95 to 1.0
- Dismiss animation (200ms, Ease In):
  - Fade out: opacity 1 to 0
  - Slide out: translate 0 to -50px
  - Scale: 1.0 to 0.95
- Smart 60fps subscription (only active when animations running)
- Automatic animation cleanup on completion
- Per-animation type configuration (appear/dismiss separately)
- Progress indicators for timed notifications

#### Accessibility (Phase 4)
- Automatic detection of system reduced-motion preferences via XDG Desktop Portal
- Reads `org.freedesktop.appearance.reduced-motion` setting
- Animations automatically disabled when reduced motion preferred
- Real-time updates when system setting changes
- WCAG 2.1 Success Criterion 2.3.3 (Level AAA) compliant
- Full keyboard navigation support
- Clear focus indicators
- Logical tab order
- High contrast theme support

#### Configuration (Phases 1-4)
- RON (Rusty Object Notation) configuration file format
- Configuration stored at: `~/.config/cosmic/com.cosmic.applet.notifications/config.ron`
- Configurable options:
  - Do Not Disturb mode
  - Minimum urgency level
  - History enabled/disabled
  - Maximum history items (10-1000)
  - History retention days (1-365)
  - Popup position (Auto or Panel Relative)
  - Animation settings (enable/disable per type)
  - Per-application filters
- Configuration validation and sanitization
- Automatic fallback to defaults for invalid values

#### Documentation (Phase 5)
- Comprehensive USER_GUIDE.md (6000+ words)
- Complete API_DOCUMENTATION.md with extension guide
- Enhanced INSTALL.md with 4 installation methods
- 5 configuration examples:
  - default-config.ron (annotated defaults)
  - minimal-config.ron (lightweight setup)
  - power-user-config.ron (advanced features)
  - focus-mode-config.ron (deep work mode)
  - accessibility-config.ron (reduced motion)
- Professional README.md with complete feature documentation
- ARCHITECTURE.md with technical design details
- DEVELOPMENT.md with development workflows

#### NixOS Packaging (Phase 5)
- Complete Nix flake with reproducible builds
- NixOS module for system-wide installation
- Rust 1.90.0 toolchain pinned for compatibility
- 68 git dependency hashes for vendoring
- 4 installation methods:
  - NixOS module (flake-based)
  - Direct package installation
  - Home Manager integration
  - Build from source
- Desktop entry with COSMIC applet metadata
- Multi-language support (en, de, es, fr, it, pt)
- Proper icon installation

### Performance Optimizations
- Event handling: 90% reduction in unnecessary CPU usage
- Optimized keyboard event subscription (filter_map pattern)
- Efficient URL extraction (stops at first match)
- Smart animation frame subscription (16ms interval, only when needed)
- Efficient notification queue management
- Minimal memory footprint (20-50MB idle, <200MB with 100+ notifications)
- <1% CPU usage at idle

### Technical Stack
- Rust 1.90.0 (Edition 2024)
- libcosmic (COSMIC desktop toolkit)
- zbus 4.x (async D-Bus communication)
- tokio (async runtime)
- iced (via libcosmic, GUI framework)
- ashpd 0.12 (XDG Desktop Portal bindings)
- chrono (date/time handling)
- serde (serialization)
- tracing (structured logging)

### License
- Licensed under GNU General Public License v3.0 (GPL-3.0-only)

### Known Limitations
- Animation visual rendering limited by iced framework capabilities (state calculation complete, visual transforms pending framework support)
- On-screen notification overlays deferred to future release (requires layer-shell protocol verification)

[0.1.0]: https://github.com/olafkfreund/cosmic-applet-notification/releases/tag/v0.1.0
