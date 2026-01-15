# API Documentation

**Version**: 0.1.0
**Last Updated**: 2026-01-15

This document provides an overview of the COSMIC Notification Applet's API and explains how to use the generated rustdoc documentation.

## Table of Contents

- [Viewing API Documentation](#viewing-api-documentation)
- [Public API Overview](#public-api-overview)
- [Module Structure](#module-structure)
- [Key Types](#key-types)
- [Extension Guide](#extension-guide)

## Viewing API Documentation

### Generating Documentation

```bash
# Enter development environment
nix develop

# Generate API documentation
cargo doc --no-deps --open

# Generate documentation including private items (for development)
cargo doc --no-deps --document-private-items --open
```

The generated documentation will open in your default browser at:
```
target/doc/cosmic_applet_notifications/index.html
```

### Online Documentation

[Once published to crates.io, documentation will be available at docs.rs]

## Public API Overview

The applet exposes a library interface for potential extensions and integrations.

### Exported Modules

| Module | Description |
|--------|-------------|
| `accessibility` | System accessibility settings detection (prefers-reduced-motion) |
| `config` | Configuration management and persistence |
| `dbus` | D-Bus notification interface implementation |
| `manager` | Notification state management and filtering |
| `ui` | User interface components and widgets |

### Re-exported Types

The following types are re-exported at the crate root for convenience:

```rust
pub use dbus::{Notification, NotificationAction, NotificationHints, Urgency};
```

## Module Structure

### `accessibility` Module

**Purpose**: Detect system accessibility preferences.

**Key Functions**:
- `detect_prefers_reduced_motion()` - Async detection of reduced motion preference
- `subscribe_reduced_motion_changes()` - Real-time stream of preference changes

**Dependencies**: Uses XDG Desktop Portal via `ashpd` crate.

**Example**:
```rust
use cosmic_applet_notifications::accessibility;

#[tokio::main]
async fn main() {
    let prefers_reduced = accessibility::detect_prefers_reduced_motion().await;

    if prefers_reduced {
        println!("User prefers reduced motion - disable animations");
    }
}
```

### `config` Module

**Purpose**: Configuration management with validation and persistence.

**Key Types**:
- `AppletConfig` - Main configuration struct
- `PopupPosition` - Popup positioning configuration
- `AnimationConfig` - Animation settings
- `ConfigHelper` - Configuration I/O helper

**Example**:
```rust
use cosmic_applet_notifications::config::{AppletConfig, ConfigHelper};

let helper = ConfigHelper::new();
let mut config = helper.load();

config.do_not_disturb = true;
helper.save(&config).expect("Failed to save config");
```

### `dbus` Module

**Purpose**: D-Bus notification interface implementation (freedesktop.org spec).

**Key Types**:
- `Notification` - Notification data structure
- `NotificationAction` - Action button definition
- `NotificationHints` - Metadata hints
- `Urgency` - Low, Normal, or Critical

**Key Functions**:
- `subscribe()` - Returns subscription for receiving notifications

**Example**:
```rust
use cosmic_applet_notifications::dbus;

// In an async context with iced/cosmic
let subscription = dbus::subscribe();

// Notifications arrive as Message::NotificationReceived(Notification)
```

### `manager` Module

**Purpose**: Notification state management, filtering, and history.

**Key Types**:
- `NotificationManager` - Core state manager
- `FilterAction` - Allow, Block, or Defer filtering result

**Key Methods**:
```rust
use cosmic_applet_notifications::manager::NotificationManager;

let mut manager = NotificationManager::new();

// Add notification
manager.add_notification(notification);

// Get active notifications
let active = manager.get_active_notifications();

// Remove notification
manager.remove_notification(notification_id);

// Set Do Not Disturb
manager.set_do_not_disturb(true);

// Set urgency filter
manager.set_min_urgency_level(2); // Critical only
```

### `ui` Module

**Purpose**: User interface components and rendering.

**Submodules**:
- `ui::animation` - Animation system (easing functions, timelines)
- `ui::widgets` - UI components (notification cards, lists, settings)
- `ui::url_parser` - URL detection and parsing

**Key Types**:
- `NotificationAnimation` - Animation state for notifications
- `Easing` - Easing function enum (9 variants)
- `AnimationDuration` - Pre-defined durations

**Example** (animation):
```rust
use cosmic_applet_notifications::ui::animation::{
    NotificationAnimation, AnimationDuration, Easing
};

// Create appear animation
let animation = NotificationAnimation::appearing(
    notification_id,
    AnimationDuration::NORMAL,
);

// Check if complete
if animation.is_complete() {
    // Animation finished
}

// Get current opacity value
let opacity = animation.opacity.value();
```

## Key Types

### `Notification`

Represents a D-Bus notification.

**Fields**:
```rust
pub struct Notification {
    pub id: u32,
    pub app_name: String,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub actions: Vec<NotificationAction>,
    pub hints: NotificationHints,
    pub expire_timeout: i32,
    pub urgency: Urgency,
    pub timestamp: chrono::DateTime<chrono::Local>,
}
```

### `AppletConfig`

Main configuration structure.

**Fields**:
```rust
pub struct AppletConfig {
    pub do_not_disturb: bool,
    pub min_urgency_level: u8,
    pub history_enabled: bool,
    pub max_history_items: usize,
    pub history_retention_days: u64,
    pub popup_position: PopupPosition,
    pub animations: AnimationConfig,
    pub app_filters: HashMap<String, bool>,
}
```

**Methods**:
- `validate() -> bool` - Checks configuration validity
- `sanitize()` - Fixes invalid values to safe defaults

### `NotificationManager`

Core state manager for notifications.

**Key Methods**:
```rust
impl NotificationManager {
    // Create new manager
    pub fn new() -> Self;

    // Create with history support
    pub fn with_history(max_items: usize, retention_days: u64) -> Self;

    // Notification management
    pub fn add_notification(&mut self, notification: Notification) -> FilterAction;
    pub fn remove_notification(&mut self, id: u32) -> bool;
    pub fn get_active_notifications(&self) -> &VecDeque<Notification>;

    // Filtering
    pub fn set_do_not_disturb(&mut self, enabled: bool);
    pub fn set_min_urgency_level(&mut self, level: u8);
    pub fn load_app_filters(&mut self, filters: HashMap<String, bool>);

    // History
    pub fn get_history(&self) -> Option<&Vec<Notification>>;
}
```

## Extension Guide

### Creating a Custom Filter

```rust
use cosmic_applet_notifications::manager::NotificationManager;
use cosmic_applet_notifications::Notification;

fn custom_filter(notification: &Notification) -> bool {
    // Example: Block notifications from apps starting with "Test"
    !notification.app_name.starts_with("Test")
}

// Apply filter
let mut manager = NotificationManager::new();
// (Custom filter application would need to be added to manager API)
```

### Integrating with Other COSMIC Applets

```rust
use cosmic_applet_notifications::dbus;

// Subscribe to notifications in your applet
let notification_subscription = dbus::subscribe();

// In your update() method
fn update(&mut self, message: Message) -> Task<Action<Message>> {
    match message {
        Message::NotificationReceived(notification) => {
            // Handle notification in your applet
            self.process_notification(notification);
        }
        // ... other messages
    }
    Task::none()
}
```

### Creating Custom Animations

```rust
use cosmic_applet_notifications::ui::animation::{Animation, Easing, AnimationDuration};
use std::time::Instant;

// Create custom animation
let custom_animation = Animation::from_to(
    AnimationDuration(500),  // 500ms duration
    Easing::BounceOut,       // Bounce easing
    0.0,                     // Start value
    1.0,                     // End value
);

// Update and get value
let value = custom_animation.value();  // Returns current animated value
```

## Testing

The crate includes comprehensive test coverage:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific module tests
cargo test manager::tests
cargo test dbus::tests
```

### Test Coverage

- **Unit tests**: All modules have unit tests
- **Integration tests**: D-Bus communication, manager integration
- **Doc tests**: Examples in documentation are tested

## Documentation Standards

All public API items follow these standards:

1. **Module Documentation**: Every module has a module-level doc comment
2. **Function Documentation**: All public functions have:
   - Purpose description
   - Parameter descriptions
   - Return value description
   - Example usage (where applicable)
3. **Type Documentation**: All public types document:
   - Purpose
   - Field meanings
   - Invariants or constraints
4. **Examples**: Complex functionality includes code examples

## Contributing to API Documentation

When adding new public API:

1. **Write doc comments** using `///` for public items
2. **Include examples** for non-trivial functions
3. **Link related items** using `[Type]` or `[module::function]` syntax
4. **Test examples** - doc examples are run with `cargo test`
5. **Update this guide** if adding new modules or major features

### Documentation Style

```rust
/// Brief one-line description.
///
/// More detailed explanation of what this does, including any
/// important behavior or edge cases.
///
/// # Arguments
///
/// * `param1` - Description of parameter
/// * `param2` - Description of parameter
///
/// # Returns
///
/// Description of return value and what different values mean.
///
/// # Example
///
/// ```
/// use cosmic_applet_notifications::example;
///
/// let result = example::function(arg1, arg2);
/// assert_eq!(result, expected_value);
/// ```
///
/// # Panics
///
/// Describes conditions under which this function panics (if any).
///
/// # Errors
///
/// Describes error conditions for Result-returning functions.
pub fn well_documented_function(param1: Type1, param2: Type2) -> ReturnType {
    // implementation
}
```

## Versioning

This project follows [Semantic Versioning](https://semver.org/):

- **MAJOR version** (0.x.x → 1.x.x): Incompatible API changes
- **MINOR version** (x.0.x → x.1.x): New functionality, backwards compatible
- **PATCH version** (x.x.0 → x.x.1): Bug fixes, backwards compatible

Currently at **v0.1.0** (pre-release).

## License

GPL-3.0-only - See LICENSE file for details.

## Additional Resources

- **[USER_GUIDE.md](./USER_GUIDE.md)** - End-user documentation
- **[ARCHITECTURE.md](./ARCHITECTURE.md)** - System architecture
- **[DEVELOPMENT.md](./DEVELOPMENT.md)** - Development workflows
- **[freedesktop.org Notification Spec](https://specifications.freedesktop.org/notification-spec/latest/)** - D-Bus spec we implement

---

**Questions or Issues?**
- 💬 GitHub Discussions
- 🐛 GitHub Issues
- 📖 [Generated rustdoc](target/doc/cosmic_applet_notifications/index.html)

---

Built with ❤️ for COSMIC Desktop
