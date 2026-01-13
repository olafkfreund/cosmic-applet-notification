# CLAUDE.md - AI Assistant Context

This file provides comprehensive context for AI assistants (like Claude, ChatGPT, Copilot) working on the COSMIC Notification Applet project. It contains project-specific knowledge, conventions, and guidelines to ensure consistent and high-quality assistance.

## Project Identity

**Project Name**: COSMIC Notification Applet
**Primary Language**: Rust
**Target Platform**: NixOS with COSMIC Desktop Environment
**License**: [TBD - likely GPL-3.0]
**Status**: In Development (Phase 1)

## Project Purpose

This applet enhances the COSMIC Desktop notification experience by providing:
- Custom notification displays with better visibility
- Flexible notification placement options
- Enhanced interaction (clickable URLs, action buttons)
- Notification history management
- Per-application customization

## Technology Stack

### Core Technologies
- **Rust** (Edition 2021, MSRV 1.75+)
- **libcosmic** - COSMIC desktop toolkit (based on iced)
- **zbus 4.x** - Async D-Bus communication
- **tokio** - Async runtime
- **Nix/NixOS** - Build system and deployment

### Key Libraries
- `cosmic-config` - Configuration management
- `cosmic-time` - Time/date utilities
- `serde` - Serialization/deserialization
- `chrono` - Date/time handling
- `thiserror` - Error handling
- `tracing` - Structured logging

## Architecture Overview

The applet has three main components that communicate via async channels:

1. **D-Bus Listener** (zbus)
   - Subscribes to org.freedesktop.Notifications signals
   - Runs in separate async task
   - Sends notifications via mpsc channel to manager

2. **Notification Manager**
   - Receives notifications from D-Bus listener
   - Manages state, history, filtering
   - Sends UI updates via channel

3. **UI Renderer** (libcosmic)
   - COSMIC panel applet
   - Popup window for notifications
   - Handles user interactions

## Code Organization

```
cosmic-notification-applet/
├── src/
│   ├── main.rs              # Entry point, applet initialization
│   ├── dbus/
│   │   ├── mod.rs          # D-Bus module
│   │   ├── listener.rs     # Notification listener
│   │   └── types.rs        # D-Bus type definitions
│   ├── manager/
│   │   ├── mod.rs          # Manager module
│   │   ├── state.rs        # State management
│   │   ├── filter.rs       # Filtering logic
│   │   └── history.rs      # History management
│   ├── ui/
│   │   ├── mod.rs          # UI module
│   │   ├── applet.rs       # Main applet implementation
│   │   ├── popup.rs        # Popup window
│   │   └── widgets/
│   │       ├── notification_card.rs
│   │       └── notification_list.rs
│   ├── config/
│   │   ├── mod.rs          # Configuration module
│   │   └── schema.rs       # Config data structures
│   └── error.rs            # Error types
├── tests/
│   ├── dbus_tests.rs       # D-Bus integration tests
│   └── manager_tests.rs    # Manager unit tests
├── docs/
│   └── skills/             # AI assistant skills (see below)
├── Cargo.toml              # Rust dependencies
├── flake.nix               # Nix flake for dev environment
├── justfile                # Build automation
└── README.md               # User-facing documentation
```

## Coding Standards

### Rust Style Guide
- Follow official Rust style guide (rustfmt)
- Use clippy for linting (`clippy::pedantic` + exceptions)
- Maximum line length: 100 characters
- Use meaningful variable names (no single letters except in closures/short scopes)

### Naming Conventions
- **Types**: PascalCase (`NotificationManager`)
- **Functions/Methods**: snake_case (`process_notification`)
- **Constants**: SCREAMING_SNAKE_CASE (`MAX_HISTORY_SIZE`)
- **Modules**: snake_case (`notification_card`)

### Error Handling
- Use `Result<T, AppletError>` for fallible operations
- Use `thiserror` for error definitions
- Always provide context in error messages
- Don't panic in library code; return errors
- Use `.expect()` only when truly impossible to fail

### Async Patterns
- Use `tokio::spawn` for long-running tasks
- Use `tokio::select!` for concurrent operations
- Prefer structured concurrency (don't leak tasks)
- Use channels (`mpsc`) for inter-task communication
- Document which functions must run in async context

### Documentation
- Doc comments for all public items (`///`)
- Include examples in doc comments when helpful
- Document safety requirements for unsafe code
- Use `#[must_use]` for important return values

## libcosmic Patterns

### Applet Structure
```rust
impl cosmic::Application for NotificationApplet {
    type Message = Message;
    type Executor = cosmic::executor::Default;
    type Flags = ();
    
    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        // Initialize state, start D-Bus listener
        // Return initial command (if any)
    }
    
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        // Handle messages, update state
        // Return commands for side effects
    }
    
    fn view(&self) -> Element<Self::Message> {
        // Return panel icon view
        // Use core.applet.icon_button()
    }
    
    fn view_window(&self, id: window::Id) -> Element<Self::Message> {
        // Return popup window view
        // Use core.applet.popup_container()
    }
    
    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }
}
```

### Message Pattern
```rust
#[derive(Debug, Clone)]
pub enum Message {
    // D-Bus events
    NotificationReceived(Notification),
    NotificationClosed(u32),
    
    // User interactions
    TogglePopup,
    DismissNotification(u32),
    ClickAction { notification_id: u32, action_key: String },
    
    // Internal events
    UpdateConfig(AppletConfig),
    Tick(Instant),
}
```

### Popup Window Pattern
```rust
fn view_window(&self, id: window::Id) -> Element<Message> {
    if self.popup_id == Some(id) {
        self.core
            .applet
            .popup_container(self.notification_list_view())
            .into()
    } else {
        // Other windows (settings, etc)
    }
}
```

## D-Bus Implementation

### Signal Subscription
```rust
// Subscribe to notification signals
let notifications = connection
    .receive_signal::<NotificationSignal>()
    .await?;

// Process signals
while let Some(signal) = notifications.next().await {
    // Handle notification
}
```

### Method Implementation
```rust
#[zbus::interface(name = "org.freedesktop.Notifications")]
impl NotificationService {
    async fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: Vec<&str>,
        hints: HashMap<&str, zbus::zvariant::Value>,
        expire_timeout: i32,
    ) -> zbus::fdo::Result<u32> {
        // Process notification
    }
}
```

## Testing Strategy

### Unit Tests
- Test each module independently
- Mock D-Bus interactions
- Test configuration loading/saving
- Test filter logic

### Integration Tests
- Test D-Bus communication end-to-end
- Test notification flow from reception to display
- Test configuration persistence

### Manual Testing Checklist
- Send test notification: `notify-send "Test" "Message"`
- Test urgency levels: `notify-send -u critical "Critical"`
- Test actions: `notify-send -A "action1=Click Me" "Test"`
- Test rapid notifications: 10+ notifications in 1 second
- Test long messages: 500+ character body
- Test special characters: emojis, unicode, HTML

## Common Tasks

### Adding a New Feature
1. Update `ARCHITECTURE.md` with design
2. Add types to appropriate module
3. Implement logic with tests
4. Update UI if needed
5. Add configuration option if user-facing
6. Document in user guide

### Debugging
- Enable trace logging: `RUST_LOG=trace cargo run`
- Use `tracing::debug!()` for diagnostic output
- Test D-Bus with: `dbus-monitor "interface='org.freedesktop.Notifications'"`
- Profile with: `cargo flamegraph`

### Adding a Dependency
1. Check license compatibility (GPL-3.0 compatible)
2. Verify maintenance status
3. Add to `Cargo.toml`
4. Update `flake.nix` if system dependency required
5. Document in `ARCHITECTURE.md`

## NixOS Specifics

### Development Environment
- Use `nix develop` to enter dev shell
- Dependencies automatically provided
- `rust-analyzer` configured automatically

### Building
- `nix build` for release build
- `nix develop -c cargo build` for incremental builds
- `nix run` to run without installing

### Testing on NixOS
- Install to system: `sudo just install`
- Log out and back in to reload COSMIC
- Check logs: `journalctl --user -u cosmic-panel`

## Troubleshooting

### Common Issues

**Applet doesn't appear in panel**
- Check desktop entry has `X-CosmicApplet=true`
- Verify install location matches COSMIC paths
- Check COSMIC panel config

**D-Bus connection fails**
- Verify `DBUS_SESSION_BUS_ADDRESS` set
- Check session bus is running
- Test with `busctl --user tree org.freedesktop.Notifications`

**Build errors with libcosmic**
- Ensure using latest libcosmic from git
- Check all system dependencies installed
- Verify rustc version >= 1.75

**Performance issues**
- Profile with `cargo flamegraph`
- Check notification queue size
- Verify no notification leaks

## Related Documentation

### External Resources
- [libcosmic Book](https://pop-os.github.io/libcosmic-book/)
- [libcosmic API Docs](https://pop-os.github.io/libcosmic/)
- [zbus Documentation](https://dbus2.github.io/zbus/)
- [freedesktop.org Notifications Spec](https://specifications.freedesktop.org/notification/latest/)
- [COSMIC Epoch GitHub](https://github.com/pop-os/cosmic-epoch)

### Skills Files
Located in `docs/skills/`, these provide deep-dive knowledge:
- `zbus_skill.md` - D-Bus communication with zbus
- `libcosmic_applet_skill.md` - COSMIC applet development
- `notification_spec_skill.md` - freedesktop notification spec
- `nixos_rust_skill.md` - NixOS + Rust development

### Project Documents
- `PROJECT_PLAN.md` - Development roadmap and phases
- `ARCHITECTURE.md` - Detailed technical design
- `DEVELOPMENT.md` - Development workflows and commands

## Best Practices Checklist

When implementing a feature:
- [ ] Design documented in ARCHITECTURE.md
- [ ] Types defined with appropriate derives
- [ ] Error handling uses Result types
- [ ] Public APIs have doc comments
- [ ] Unit tests written
- [ ] Integration test if crossing boundaries
- [ ] Manual testing performed
- [ ] Configuration option added if user-facing
- [ ] Logging added for key operations
- [ ] Performance considered
- [ ] Memory safety verified
- [ ] Updated relevant documentation

## Getting Help

When you need assistance:

1. **Check existing documentation**
   - This file (CLAUDE.md)
   - Architecture document
   - Skills files

2. **Search codebase**
   - Look for similar implementations
   - Check test files for examples

3. **External resources**
   - libcosmic examples
   - zbus examples
   - COSMIC applet repositories

4. **Community**
   - COSMIC Matrix channel
   - NixOS Discourse
   - Rust forums

## Version History

- **v1.0** (2025-01-13): Initial version
  - Comprehensive project context
  - Architecture overview
  - Coding standards
  - Development workflows

---

**Note to AI Assistants**: This document is your primary source of truth for this project. When providing assistance:
- Reference this document for project conventions
- Consult architecture document for design decisions
- Check skills files for domain-specific knowledge
- Maintain consistency with established patterns
- Ask for clarification if project context is unclear
- Update this document when learning new project-specific patterns

**Last Updated**: 2025-01-13
