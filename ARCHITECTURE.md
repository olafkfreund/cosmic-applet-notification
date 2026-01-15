# Architecture: COSMIC Notification Applet

## System Overview

The COSMIC Notification Applet is a Wayland-native panel applet that intercepts freedesktop.org desktop notifications via D-Bus and displays them through the COSMIC Desktop panel interface. The architecture prioritizes modularity, performance, and maintainability while adhering to COSMIC's design principles.

**Document Status**: Updated with research from COSMIC ecosystem, zbus patterns, and notification daemon best practices (2025-01-15)

**Research Sources**:
- [COSMIC Panel Applets Documentation](https://pop-os.github.io/libcosmic-book/panel-applets.html)
- [cosmic-applets Repository](https://github.com/pop-os/cosmic-applets) - Official COSMIC applets
- [freedesktop.org Notification Specification](https://specifications.freedesktop.org/notification-spec/latest/)
- [zbus Documentation](https://docs.rs/zbus/latest/zbus/) - Async D-Bus for Rust
- [iced Subscriptions](https://docs.iced.rs/iced_futures/subscription/struct.Subscription.html)
- [runst Architecture](https://blog.orhun.dev/introducing-runst/) - Rust notification daemon patterns

## COSMIC Applet Integration

### Panel Architecture Context

COSMIC panel applets are **self-contained application processes** that integrate with the COSMIC panel through a compositor-based architecture. Understanding this context is crucial for proper implementation:

**Key Principles**:
1. **Separate Process**: Each applet runs as its own process, not as a library in the panel
2. **Wayland Integration**: The panel is itself a Wayland compositor that manages applet windows
3. **Desktop Entry Launch**: Applets are launched by their desktop entry file names
4. **Position Configuration**: Panel reads config to determine applet positioning
5. **Transparent Windows**: Applets use headerless, transparent windows for seamless integration
6. **Popup Forwarding**: Popup windows are forwarded to the host compositor for display outside the panel

**Security Benefits**: This architecture provides process isolation, crash resilience, and easier sandboxing compared to in-process plugins.

### Applet Lifecycle

```
1. Panel Startup
   └─→ Reads panel configuration file
       └─→ Launches applet by desktop entry name
           └─→ Sets environment variables for configuration
               └─→ Applet initializes via cosmic::applet::run()
                   └─→ Panel positions main window in panel
                       └─→ Applet can create popup windows on demand
```

### Environment Variables

The panel communicates with applets through environment variables that are read at startup by `Context::default()`:
- **Position**: Where in the panel the applet is placed
- **Panel Configuration**: Theme, spacing, size preferences
- **Desktop Entry**: Applet identifier and metadata

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│         COSMIC Panel (Wayland Compositor)                    │
│  ┌───────────────────────────────────────────────────────┐  │
│  │    Notification Applet Process (This Project)         │  │
│  │  ┌─────────────────────────────────────────────────┐  │  │
│  │  │  libcosmic Application (iced runtime)           │  │  │
│  │  │                                                  │  │  │
│  │  │  ┌────────────────┐     ┌──────────────────┐   │  │  │
│  │  │  │ Main Window    │     │ Popup Window(s)  │   │  │  │
│  │  │  │ (Panel Icon)   │     │ (Notification    │   │  │  │
│  │  │  └────────────────┘     │  List)           │   │  │  │
│  │  │                         └──────────────────┘   │  │  │
│  │  │                                                  │  │  │
│  │  │  Application State:                             │  │  │
│  │  │  ┌─────────────────────────────────────────┐   │  │  │
│  │  │  │ • Notification Manager (state)          │   │  │  │
│  │  │  │ • Configuration                         │   │  │  │
│  │  │  │ • UI State (popup IDs, etc)            │   │  │  │
│  │  │  └─────────────────────────────────────────┘   │  │  │
│  │  │                    ↑                            │  │  │
│  │  │                    │ Messages                   │  │  │
│  │  │                    │                            │  │  │
│  │  │  ┌─────────────────┴──────────────────┐        │  │  │
│  │  │  │    iced::Subscription              │        │  │  │
│  │  │  │  (D-Bus Signal Stream)             │        │  │  │
│  │  │  └────────────────────────────────────┘        │  │  │
│  │  └─────────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
          ↑                              ↑
          │ D-Bus Notifications          │ Wayland Popups
          │ (org.freedesktop.           │ (Forwarded to
          │  Notifications)             │  host compositor)
          │                              │
┌─────────────────────┐        ┌─────────────────────┐
│   Applications      │        │ COSMIC Compositor   │
│  (notify-send,      │        │   (cosmic-comp)     │
│   Firefox, etc)     │        │                     │
└─────────────────────┘        └─────────────────────┘
```

## Core Components

### 1. D-Bus Notification Listener (Subscription-Based)

**Responsibility**: Subscribe to and receive notification signals from the D-Bus session bus through an iced Subscription.

**Technology**: `zbus` v5.x with tokio runtime, integrated via iced `Subscription`

**Architecture Pattern**: Instead of a separate thread with channels, we use iced's built-in Subscription system:

```rust
// In Application implementation
fn subscription(&self) -> Subscription<Message> {
    notification_listener::subscribe()
}
```

**Key Features**:
- Listens for `org.freedesktop.Notifications` signals on session bus
- Implements freedesktop.org notification specification v1.2
- Yields notification events as `Message` variants directly to the application
- Automatic lifecycle management (starts with app, stops when subscription dropped)
- Non-blocking async operation integrated with iced runtime

**Implementation Pattern** (Based on [iced Subscription patterns](https://docs.iced.rs/iced_futures/subscription/struct.Subscription.html)):

```rust
pub fn subscribe() -> Subscription<Message> {
    Subscription::run_with_id(
        "dbus-notification-listener",
        notification_stream()
    )
}

async fn notification_stream() -> impl Stream<Item = Message> {
    // Connect to session bus
    let connection = Connection::session().await
        .expect("Failed to connect to D-Bus session bus");

    // Create match rule for org.freedesktop.Notifications signals
    let rule = MatchRule::builder()
        .msg_type(MessageType::Signal)
        .interface("org.freedesktop.Notifications")
        .expect("Invalid interface");

    // Subscribe to message stream
    let stream = MessageStream::for_match_rule(
        rule,
        &connection,
        Some(128), // Queue up to 128 notifications
    )
    .await
    .expect("Failed to create message stream");

    // Transform D-Bus messages into application Messages
    stream.filter_map(|msg| async move {
        match parse_notification(msg) {
            Ok(notification) => Some(Message::NotificationReceived(notification)),
            Err(e) => {
                tracing::warn!("Failed to parse notification: {}", e);
                None
            }
        }
    })
}
```

**Advantages of Subscription Pattern**:
1. **Lifecycle Management**: Subscription automatically starts/stops with application
2. **No Manual Channels**: Direct integration with iced message loop
3. **Backpressure**: Built-in queue management (max_queued parameter)
4. **Error Recovery**: Subscription can be restarted if stream ends
5. **Simplicity**: No need to manage separate tasks or spawning

**D-Bus Signals Monitored**:
- `Notify` - New notification received
- `NotificationClosed` - Notification explicitly closed by daemon
- `ActionInvoked` - User clicked notification action button

**Message Queue Management**:
- Default queue size: 128 notifications
- Overflow strategy: Drop oldest notifications (FIFO)
- Rate limiting: Consider per-application limits to prevent spam

### 2. Notification Manager

**Responsibility**: Manage notification lifecycle, state, and history.

**Key Features**:
- Notification queue management
- History persistence
- Filtering and prioritization
- Notification grouping by application
- Timeout handling

**Data Model**:
```rust
pub struct Notification {
    pub id: u32,
    pub app_name: String,
    pub app_icon: Option<String>,
    pub summary: String,
    pub body: String,
    pub actions: Vec<NotificationAction>,
    pub hints: HashMap<String, Variant>,
    pub urgency: Urgency,
    pub timeout: i32,
    pub timestamp: DateTime<Local>,
}

pub enum Urgency {
    Low,
    Normal,
    Critical,
}

pub struct NotificationAction {
    pub key: String,
    pub label: String,
}
```

**State Management**:
```rust
pub struct NotificationState {
    active_notifications: Vec<Notification>,
    notification_history: VecDeque<Notification>,
    max_history_size: usize,
    grouped_by_app: HashMap<String, Vec<u32>>,
}
```

### 3. UI Renderer (libcosmic)

**Responsibility**: Display notifications through COSMIC panel applet interface.

**Technology**: `libcosmic`, `iced`

**Key Features**:
- Panel icon with notification count badge
- Popup window for notification list
- Individual notification cards
- Click handling for actions and URLs
- Theme integration
- Animations and transitions

**Widget Hierarchy**:
```
AppletIcon
  └─ NotificationBadge (count)
       └─ PopupWindow
            └─ NotificationList
                 └─ NotificationCard
                      ├─ AppIcon
                      ├─ Summary
                      ├─ Body
                      ├─ Actions
                      └─ Timestamp
```

**libcosmic Integration**:
```rust
impl Application for NotificationApplet {
    type Message = Message;
    type Executor = cosmic::executor::Default;
    type Flags = ();
    
    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>);
    fn update(&mut self, message: Self::Message) -> Command<Self::Message>;
    fn view(&self) -> Element<Self::Message>;
    fn view_window(&self, id: window::Id) -> Element<Self::Message>;
}
```

### 4. Configuration System

**Responsibility**: Manage user preferences and settings.

**Technology**: `cosmic-config` (COSMIC's config system)

**Configuration Schema**:
```rust
pub struct AppletConfig {
    // Display settings
    pub notification_position: NotificationPosition,
    pub notification_size: NotificationSize,
    pub max_visible_notifications: usize,
    pub show_timestamp: bool,
    pub show_app_icon: bool,
    
    // Behavior settings
    pub do_not_disturb: bool,
    pub notification_timeout: Option<u32>,
    pub play_sound: bool,
    pub sound_file: Option<PathBuf>,
    
    // History settings
    pub history_enabled: bool,
    pub max_history_items: usize,
    pub history_retention_days: u32,
    
    // Filtering
    pub app_filters: HashMap<String, AppFilterConfig>,
    pub urgency_filters: UrgencyFilterConfig,
    
    // Appearance
    pub theme_override: Option<String>,
    pub custom_colors: Option<ColorScheme>,
}

pub enum NotificationPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Custom { x: i32, y: i32 },
}
```

**Storage**: Configuration stored in `~/.config/cosmic/com.system76.CosmicAppletNotifications/`

## Data Flow

### Notification Reception Flow

```
Application
    │
    ├─→ D-Bus Signal (Notify)
    │        │
    ↓        ↓
zbus Listener
    │
    ├─→ Parse & Validate
    │        │
    ↓        ↓
Notification Manager
    │
    ├─→ Apply Filters
    ├─→ Check Urgency
    ├─→ Group if needed
    │        │
    ↓        ↓
UI Update Channel
    │
    ├─→ State Update
    │        │
    ↓        ↓
libcosmic Renderer
    │
    └─→ Display Notification
```

### User Interaction Flow

```
User Click/Key Event
    │
    ↓
libcosmic Event Handler
    │
    ├─→ Action Button? ─→ Send D-Bus ActionInvoked
    │
    ├─→ URL Click? ─→ Open with xdg-open
    │
    ├─→ Dismiss? ─→ Remove from active list
    │
    └─→ View History? ─→ Show history popup
```

## Threading and Async Model

### Unified Async Architecture (Subscription-Based)

**Research Finding**: Based on [iced's Subscription documentation](https://docs.iced.rs/iced_futures/subscription/struct.Subscription.html) and [tokio integration patterns](https://tokio.rs/tokio/topics/bridging), the recommended pattern is a single-threaded event loop with integrated async streams, NOT separate threads with channels.

```
┌───────────────────────────────────────────────────────────────┐
│                  Single Application Process                    │
│                                                                │
│  ┌──────────────────────────────────────────────────────────┐ │
│  │         iced Runtime (cosmic::executor::Default)         │ │
│  │                    (tokio-based)                         │ │
│  │                                                          │ │
│  │  ┌────────────────────┐      ┌──────────────────────┐  │ │
│  │  │  Application       │◄─────│  Subscription        │  │ │
│  │  │  (update/view)     │      │  (D-Bus Stream)      │  │ │
│  │  │                    │      │                      │  │ │
│  │  │  • State           │      │  • zbus Connection   │  │ │
│  │  │  • UI Rendering    │      │  • Message Stream    │  │ │
│  │  │  • Message Handler │      │  • Signal Parsing    │  │ │
│  │  └────────────────────┘      └──────────────────────┘  │ │
│  │           │                            │                │ │
│  │           └────────►Message◄───────────┘                │ │
│  │                                                          │ │
│  │  All running in the same async runtime, coordinated     │ │
│  │  by iced's event loop - no manual thread spawning!      │ │
│  └──────────────────────────────────────────────────────────┘ │
└───────────────────────────────────────────────────────────────┘
```

**Key Architectural Decisions**:

1. **No Separate Threads**: Everything runs in iced's event loop
2. **Subscription-Driven**: D-Bus signals delivered via `subscription()` method
3. **Single Executor**: cosmic::executor::Default (tokio-based) handles all async
4. **Message-Centric**: All events (UI, D-Bus, timers) flow through the Message enum

**Why This Pattern?**

Based on [iced FAQ](https://book.iced.rs/faq.html) and cosmic-applets repository patterns:
- ✅ **Simpler**: No manual task/thread management
- ✅ **Safer**: No shared state or locks needed
- ✅ **Automatic**: Subscription lifecycle tied to Application
- ✅ **Performant**: No context switching overhead
- ✅ **Debuggable**: Single-threaded execution is easier to trace

**Real-World Example**: The [cosmic-applet-audio](https://github.com/pop-os/cosmic-applets/tree/master/cosmic-applet-audio) uses Subscriptions for PulseAudio events, not separate threads.

### Async Runtime Configuration

```rust
impl Application for NotificationApplet {
    type Executor = cosmic::executor::Default;  // Uses tokio
    type Message = Message;
    type Flags = ();

    // Subscription provides async D-Bus stream
    fn subscription(&self) -> Subscription<Message> {
        notification_listener::subscribe()
    }

    // All message handling in one place
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NotificationReceived(notif) => {
                self.manager.add_notification(notif);
                Command::none()
            }
            // ... other messages
        }
    }
}
```

**Benefits of Single-Runtime Approach**:
- No race conditions
- No mutexes or atomics needed
- Simplified error handling
- Easier testing
- Natural backpressure from message queue

## Performance Considerations

### Memory Management
- **Notification History**: Circular buffer with max size (default: 100)
- **Active Notifications**: Limited by screen space (max 5 visible)
- **String Interning**: Common app names interned to reduce duplication

### CPU Optimization
- **Lazy Rendering**: Only render visible notifications
- **Event Debouncing**: Batch rapid notification updates
- **Async I/O**: Non-blocking D-Bus communication

### Resource Limits
```rust
const MAX_ACTIVE_NOTIFICATIONS: usize = 10;
const MAX_HISTORY_SIZE: usize = 100;
const MAX_NOTIFICATION_BODY_LENGTH: usize = 500;
const NOTIFICATION_RENDER_TIMEOUT_MS: u64 = 100;
```

## Security Considerations

### Input Validation
- **Summary/Body**: Sanitize HTML tags, limit length
- **App Icons**: Validate paths, check file types
- **Actions**: Validate action keys, prevent command injection
- **URLs**: Validate schemes (http, https, mailto only)

### D-Bus Security
- Use session bus (not system bus)
- Validate sender credentials
- Rate limiting per application
- Sandboxing for action execution

### Configuration Security
- Validate config file permissions (0600)
- Sanitize file paths
- Limit configuration file size

## Error Handling Strategy

### Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum AppletError {
    #[error("D-Bus connection failed: {0}")]
    DBusError(#[from] zbus::Error),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Invalid notification: {0}")]
    InvalidNotification(String),
    
    #[error("Rendering error: {0}")]
    RenderError(String),
}
```

### Error Recovery
- **D-Bus disconnection**: Automatic reconnection with exponential backoff
- **Invalid notifications**: Log and skip, don't crash
- **Configuration errors**: Fall back to defaults
- **Rendering errors**: Show error notification

## Testing Strategy

### Unit Tests
- Notification parsing and validation
- Filter logic
- State management
- Configuration loading

### Integration Tests
- D-Bus communication
- Configuration persistence
- History management

### Manual Testing
- Send test notifications with `notify-send`
- Test with real applications (Firefox, Telegram, etc.)
- Test all urgency levels
- Test action buttons
- Test with rapid notification bursts

## Deployment Architecture

### NixOS Package Structure
```nix
{
  pname = "cosmic-notification-applet";
  version = "0.1.0";
  
  nativeBuildInputs = [
    rustPlatform.rust.cargo
    rustPlatform.rust.rustc
    pkg-config
    just
  ];
  
  buildInputs = [
    libxkbcommon
    wayland
    # Other COSMIC dependencies
  ];
  
  installPhase = ''
    just install prefix=$out
  '';
}
```

### Installation Layout
```
/usr/local/
├── bin/
│   └── cosmic-applet-notifications
├── share/
│   ├── applications/
│   │   └── com.system76.CosmicAppletNotifications.desktop
│   ├── icons/
│   │   └── hicolor/scalable/apps/
│   │       └── com.system76.CosmicAppletNotifications.svg
│   └── cosmic/
│       └── com.system76.CosmicAppletNotifications/
│           └── v1/
│               └── config.ron
```

## Extensibility Points

### Plugin System (Future)
- Custom notification parsers
- Theme extensions
- Action handlers
- Filter plugins

### API Surface
```rust
pub trait NotificationFilter {
    fn should_display(&self, notification: &Notification) -> bool;
}

pub trait NotificationRenderer {
    fn render(&self, notification: &Notification) -> Element<Message>;
}
```

## Monitoring and Observability

### Logging
- Use `tracing` crate for structured logging
- Log levels: ERROR, WARN, INFO, DEBUG, TRACE
- Key events to log:
  - Notification reception
  - Filter application
  - Configuration changes
  - D-Bus errors

### Metrics (Future)
- Notification rate
- Filter hit rate
- Average notification lifetime
- Memory usage
- D-Bus latency

## Dependencies

### Core Dependencies
```toml
[dependencies]
libcosmic = { git = "https://github.com/pop-os/libcosmic" }
zbus = { version = "4", features = ["tokio"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
cosmic-config = { git = "https://github.com/pop-os/libcosmic" }
chrono = "0.4"
thiserror = "1"
tracing = "0.1"
tracing-subscriber = "0.3"

[dev-dependencies]
tokio-test = "0.4"
tempfile = "3"
```

### Build Dependencies
- Rust stable toolchain (1.75+)
- pkg-config
- just (build system)
- Wayland development libraries
- libxkbcommon development libraries

## Migration and Compatibility

### Coexistence with cosmic-notifications
- Both can run simultaneously
- This applet doesn't claim D-Bus service name
- Simply listens to signals
- Users can gradually transition

### Configuration Migration
- Detect existing cosmic-notifications config
- Offer to import settings
- Provide mapping guide

## Implementation Roadmap & Best Practices

### Phase 1: Foundation (Issues #2-5)

Based on research of [cosmic-applets patterns](https://github.com/pop-os/cosmic-applets) and [notification daemon implementations](https://blog.orhun.dev/introducing-runst/):

**Priority Order**:
1. **D-Bus types and data model** (Issue #3) - Foundation for everything
2. **Subscription-based listener** (Issue #2) - Core async pattern
3. **Basic notification manager** (Issue #4) - State management
4. **Message flow integration** (Issue #5) - Wire components together

**Key Lessons from Research**:

✅ **Start with Data Model**: Define `Notification`, `NotificationHint`, `Urgency` types first
- Use `#[derive(Debug, Clone)]` for all types (required for iced Messages)
- Parse D-Bus Variant hints carefully (urgency, category, image-data, etc.)
- Reference: [freedesktop.org notification spec](https://specifications.freedesktop.org/notification-spec/latest/)

✅ **Use Subscription Pattern**: Don't create separate threads
- Implement `notification_listener::subscribe()` returning `Subscription<Message>`
- Use `zbus::MessageStream::for_match_rule()` for signal subscription
- Let iced manage the lifecycle - no manual spawning

✅ **Keep State Simple**: Notification manager is just a struct with Vec/VecDeque
- `active_notifications: Vec<Notification>` - Currently displayed
- `history: VecDeque<Notification>` - Past notifications with max size
- No Arc/Mutex needed (single-threaded!)

✅ **Minimal UI First**: Panel icon with badge, basic popup window
- Use `core.applet.icon_button()` for panel icon
- Use `core.applet.popup_container()` for popup
- Add notification list rendering later

### Phase 2: Enhanced Features (Issues #6-12)

**UI Development Pattern** (from libcosmic book):
```rust
// Main window (panel icon)
fn view(&self) -> Element<Message> {
    let count = self.manager.active_count();
    self.core.applet
        .icon_button("notification-symbolic")
        .on_press_down(Message::TogglePopup)
        // TODO: Add badge showing count
        .into()
}

// Popup window (notification list)
fn view_window(&self, id: window::Id) -> Element<Message> {
    if Some(id) == self.popup_id {
        let notifications = self.manager.notifications()
            .map(|n| notification_card::view(n))
            .collect();

        self.core.applet
            .popup_container(column(notifications))
            .into()
    } else {
        text("Unknown window").into()
    }
}
```

**Configuration Integration** (cosmic-config pattern):
- Use `cosmic_config::Config::new()` with app ID
- Define config schema with `#[derive(Serialize, Deserialize)]`
- Watch for changes with config watcher subscription
- Store in `~/.config/cosmic/com.system76.CosmicAppletNotifications/`

### Notification Daemon Best Practices

Based on analysis of [runst](https://blog.orhun.dev/introducing-runst/), [wired-notify](https://github.com/Toqozz/wired-notify), and other implementations:

**1. Intelligent Timeout Handling**:
```rust
// Respect application-specified timeout
// -1 = no timeout (user dismisses)
// 0 = default timeout (server decides)
// > 0 = specific timeout in milliseconds

match notification.timeout {
    -1 => None,  // No auto-dismiss
    0 => Some(Duration::from_secs(5)),  // Default
    ms if ms > 0 => Some(Duration::from_millis(ms as u64)),
    _ => Some(Duration::from_secs(5)),  // Fallback
}
```

**2. Rate Limiting Per Application**:
```rust
// Track last notification time per app
// Prevent notification spam
struct RateLimiter {
    last_notification: HashMap<String, Instant>,
    min_interval: Duration,
}

impl RateLimiter {
    fn should_allow(&mut self, app_name: &str) -> bool {
        let now = Instant::now();
        if let Some(last) = self.last_notification.get(app_name) {
            if now.duration_since(*last) < self.min_interval {
                return false;  // Too soon, reject
            }
        }
        self.last_notification.insert(app_name.to_string(), now);
        true
    }
}
```

**3. Notification Grouping**:
```rust
// Group multiple notifications from same app
// Show count instead of spam
struct NotificationGroup {
    app_name: String,
    notifications: Vec<Notification>,
    first_summary: String,
    count: usize,
}
```

**4. Urgency-Based Filtering**:
```rust
match notification.urgency {
    Urgency::Low if config.do_not_disturb => {
        // Add to history but don't display
        manager.add_to_history_only(notification);
    }
    Urgency::Critical => {
        // Always show, even in DND mode
        manager.add_notification(notification);
    }
    _ => {
        // Normal handling
        manager.add_notification(notification);
    }
}
```

### Critical Implementation Notes

**🚨 Common Pitfalls to Avoid**:

❌ **DON'T**: Claim the `org.freedesktop.Notifications` D-Bus name
- This project is a **notification viewer**, not a daemon
- We only **listen** to signals, we don't provide the service
- Let existing notification daemon (mako, dunst, etc.) handle the protocol

❌ **DON'T**: Use separate threads with Arc<Mutex<State>>
- Use iced's Subscription pattern instead
- Keep all state in Application struct
- Let iced handle synchronization

❌ **DON'T**: Parse HTML in notification body
- freedesktop spec allows basic HTML, but it's security risk
- Strip HTML tags or use safe subset
- Sanitize all user-provided content

✅ **DO**: Handle malformed D-Bus messages gracefully
```rust
match parse_notification(msg) {
    Ok(notif) => Some(Message::NotificationReceived(notif)),
    Err(e) => {
        tracing::warn!("Malformed notification: {}", e);
        None  // Skip this notification, don't crash
    }
}
```

✅ **DO**: Respect XDG Base Directory specification
```rust
// Use standard XDG paths
let config_dir = dirs::config_dir()
    .map(|p| p.join("cosmic"))
    .unwrap_or_else(|| PathBuf::from("~/.config/cosmic"));
```

✅ **DO**: Implement proper D-Bus hint parsing
```rust
// Many applications send custom hints
// Be prepared for unknown/invalid hints
fn parse_hints(hints: &HashMap<String, Variant>) -> NotificationHints {
    NotificationHints {
        urgency: hints.get("urgency")
            .and_then(|v| v.downcast_ref::<u8>().ok())
            .and_then(|&u| Urgency::from_u8(u))
            .unwrap_or(Urgency::Normal),

        category: hints.get("category")
            .and_then(|v| v.str().ok())
            .map(String::from),

        // ... parse other hints with fallbacks
    }
}
```

### Testing Strategy

**Unit Tests** (Priority: High):
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_notification_parsing() {
        // Test valid notification
        // Test malformed data
        // Test edge cases
    }

    #[test]
    fn test_urgency_filtering() {
        // Test each urgency level
        // Test DND mode
    }

    #[test]
    fn test_rate_limiting() {
        // Test spam prevention
    }
}
```

**Integration Tests** (Priority: Medium):
```rust
// Test with actual D-Bus
#[tokio::test]
async fn test_dbus_connection() {
    let conn = Connection::session().await.unwrap();
    // Test signal reception
}
```

**Manual Testing** (Priority: High):
```bash
# Test basic notification
notify-send "Test" "Message"

# Test urgency levels
notify-send -u low "Low"
notify-send -u normal "Normal"
notify-send -u critical "Critical"

# Test with actions
notify-send -A "action1=Click Me" "Test"

# Test notification spam (rate limiting)
for i in {1..20}; do notify-send "Spam $i"; done

# Test long messages
notify-send "Test" "$(head -c 1000 < /dev/urandom | base64)"
```

## Future Architecture Considerations

### Scalability
- Notification database for long-term history (SQLite)
- Network sync for multi-device (optional)
- Cloud backup for history (optional, privacy concerns)

### Performance
- Hardware acceleration for rendering (iced supports it)
- Notification caching (in-memory LRU cache)
- Lazy loading of history (virtualized list)

### Integration
- COSMIC Tasks integration (task notifications)
- Calendar event notifications (cosmic-calendar integration)
- System monitoring integration (system alerts)

### Advanced Features (Future)
- **Smart Grouping**: ML-based notification classification
- **Priority Inbox**: Learn important vs unimportant notifications
- **Action Suggestions**: Suggest actions based on content
- **Custom Notification Sounds**: Per-app sound configuration
- **Notification Templates**: Custom rendering per app

---

**Document Version**: 2.0
**Last Updated**: 2025-01-15
**Status**: Research Complete, Ready for Implementation
**Research-Backed**: Updated with findings from COSMIC applets, zbus docs, notification daemon best practices
