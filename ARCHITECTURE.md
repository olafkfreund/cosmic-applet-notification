# Architecture: COSMIC Notification Applet

## System Overview

The COSMIC Notification Applet is a Wayland-native panel applet that intercepts freedesktop.org desktop notifications via D-Bus and displays them through the COSMIC Desktop panel interface. The architecture prioritizes modularity, performance, and maintainability while adhering to COSMIC's design principles.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     COSMIC Desktop Panel                     │
│  ┌───────────────────────────────────────────────────────┐  │
│  │         Notification Applet (This Project)            │  │
│  │  ┌─────────────┐  ┌──────────────┐  ┌────────────┐  │  │
│  │  │   D-Bus     │→ │ Notification │→ │   UI       │  │  │
│  │  │  Listener   │  │   Manager    │  │ Renderer   │  │  │
│  │  └─────────────┘  └──────────────┘  └────────────┘  │  │
│  │         ↓                ↓                  ↓         │  │
│  │  ┌──────────────────────────────────────────────┐   │  │
│  │  │          State & Configuration               │   │  │
│  │  └──────────────────────────────────────────────┘   │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
          ↑                              ↑
          │ D-Bus Notifications          │ Wayland Popups
          │                              │
┌─────────────────────┐        ┌─────────────────────┐
│   Applications      │        │ COSMIC Compositor   │
│  (notify-send,      │        │   (cosmic-comp)     │
│   Firefox, etc)     │        │                     │
└─────────────────────┘        └─────────────────────┘
```

## Core Components

### 1. D-Bus Notification Listener

**Responsibility**: Subscribe to and receive notification signals from the D-Bus session bus.

**Technology**: `zbus` (async D-Bus library)

**Key Features**:
- Listens for `org.freedesktop.Notifications.Notify` signals
- Implements freedesktop.org notification specification
- Handles notification actions and closures
- Non-blocking async operation

**Interface**:
```rust
pub struct NotificationListener {
    connection: zbus::Connection,
    receiver: mpsc::Receiver<NotificationEvent>,
}

impl NotificationListener {
    pub async fn new() -> Result<Self>;
    pub async fn listen(&mut self) -> Result<()>;
    pub fn subscribe(&self) -> mpsc::Receiver<NotificationEvent>;
}
```

**D-Bus Methods Implemented**:
- `GetCapabilities` - Report supported notification features
- `Notify` - Receive notification data
- `CloseNotification` - Handle explicit close requests
- `GetServerInformation` - Provide server metadata

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

### Thread Architecture

```
┌─────────────────────────────────────────┐
│           Main Thread (tokio)           │
│  ┌────────────────────────────────────┐ │
│  │     libcosmic UI Loop (async)      │ │
│  └────────────────────────────────────┘ │
└─────────────────────────────────────────┘
              │         ↑
              │         │ mpsc channel
              ↓         │
┌─────────────────────────────────────────┐
│         D-Bus Listener Thread           │
│  ┌────────────────────────────────────┐ │
│  │      zbus Connection (async)       │ │
│  └────────────────────────────────────┘ │
└─────────────────────────────────────────┘
```

**Rationale**:
- Single-threaded UI (iced requirement)
- Async D-Bus listener for non-blocking I/O
- Channel-based communication between components
- Tokio runtime for async tasks

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

## Future Architecture Considerations

### Scalability
- Notification database for long-term history
- Network sync for multi-device
- Cloud backup for history

### Performance
- Hardware acceleration for rendering
- Notification caching
- Lazy loading of history

### Integration
- COSMIC Tasks integration
- Calendar event notifications
- System monitoring integration

---

**Document Version**: 1.0
**Last Updated**: 2025-01-13
**Status**: Design Complete, Implementation Pending
