# freedesktop.org Notification Specification Skill

## Overview

This skill covers the freedesktop.org Desktop Notifications Specification, which defines how applications send and display notifications on Linux desktop environments.

## Specification Details

**Current Version**: 1.2
**Interface**: `org.freedesktop.Notifications`
**Object Path**: `/org/freedesktop/Notifications`
**Bus**: Session bus

## Methods

### Notify

Sends a notification to the notification server.

```rust
fn Notify(
    app_name: String,        // Name of application sending notification
    replaces_id: u32,        // ID of notification to replace (0 for new)
    app_icon: String,        // Icon name or path
    summary: String,         // Summary text (required, single line)
    body: String,           // Body text (optional, may contain markup)
    actions: Vec<String>,   // List of actions (pairs: key, label)
    hints: HashMap,         // Additional data
    expire_timeout: i32,    // Timeout in milliseconds (-1 = default, 0 = never)
) -> u32;                  // Returns notification ID
```

**Example**:
```rust
let id = proxy.notify(
    "My App",                          // app_name
    0,                                 // replaces_id (new notification)
    "dialog-information",              // app_icon
    "Hello!",                          // summary
    "This is a test notification",    // body
    vec![                             // actions
        "default".to_string(),
        "Open".to_string(),
        "dismiss".to_string(),
        "Dismiss".to_string(),
    ],
    hints,                            // hints
    5000,                             // expire_timeout (5 seconds)
).await?;
```

### CloseNotification

Closes a notification.

```rust
fn CloseNotification(id: u32);
```

### GetCapabilities

Returns server capabilities.

```rust
fn GetCapabilities() -> Vec<String>;
```

**Standard Capabilities**:
- `actions` - Server supports actions
- `action-icons` - Icons in actions
- `body` - Body text support
- `body-hyperlinks` - Hyperlinks in body
- `body-images` - Images in body
- `body-markup` - Markup in body
- `icon-multi` - Multiple icons
- `icon-static` - Static icons
- `persistence` - Notifications persist
- `sound` - Sound support

### GetServerInformation

Returns information about the notification server.

```rust
fn GetServerInformation() -> (String, String, String, String);
// Returns: (name, vendor, version, spec_version)
```

## Signals

### NotificationClosed

Emitted when a notification is closed.

```rust
signal NotificationClosed(id: u32, reason: u32);
```

**Close Reasons**:
- `1` - Expired
- `2` - Dismissed by user
- `3` - Closed by CloseNotification call
- `4` - Undefined/reserved

### ActionInvoked

Emitted when a notification action is invoked.

```rust
signal ActionInvoked(id: u32, action_key: String);
```

## Hints

Hints provide additional notification metadata:

### Standard Hints

| Hint Key | Type | Description |
|----------|------|-------------|
| `action-icons` | bool | Use icons for actions |
| `category` | string | Notification category |
| `desktop-entry` | string | Desktop entry name |
| `image-data` | (iiibiiay) | Image data structure |
| `image-path` | string | Path to image file |
| `resident` | bool | Don't remove on action |
| `sound-file` | string | Sound file path |
| `sound-name` | string | Sound theme name |
| `suppress-sound` | bool | Suppress sound |
| `transient` | bool | Don't save in history |
| `x` | int | X position hint |
| `y` | int | Y position hint |
| `urgency` | byte | Urgency level (0-2) |

### Urgency Levels

```rust
pub enum Urgency {
    Low = 0,      // Low priority
    Normal = 1,   // Normal priority (default)
    Critical = 2, // Critical priority
}
```

### Image Data Structure

```rust
pub struct ImageData {
    width: i32,
    height: i32,
    rowstride: i32,
    has_alpha: bool,
    bits_per_sample: i32,
    channels: i32,
    data: Vec<u8>,
}
```

## Categories

Standard notification categories:

- `device` - Device-related
  - `device.added` - Device added
  - `device.removed` - Device removed
  
- `email` - Email notifications
  - `email.arrived` - New email
  - `email.bounced` - Bounced email

- `im` - Instant messaging
  - `im.received` - Message received
  - `im.error` - IM error

- `network` - Network events
  - `network.connected` - Connected
  - `network.disconnected` - Disconnected

- `presence` - Presence changes
  - `presence.online` - User online
  - `presence.offline` - User offline

- `transfer` - File transfers
  - `transfer.complete` - Transfer complete
  - `transfer.error` - Transfer error

## Body Markup

If `body-markup` capability is supported, body text can contain:

```xml
<b>Bold text</b>
<i>Italic text</i>
<u>Underlined text</u>
<a href="url">Link text</a>
```

**Example**:
```rust
let body = "Download <b>complete</b>!\nClick <a href=\"file:///path/to/file\">here</a> to open.";
```

## Actions

Actions are specified as pairs of strings:

```rust
vec![
    "action-key-1", "Display Label 1",
    "action-key-2", "Display Label 2",
]
```

**Special Actions**:
- `default` - Default action (invoked by clicking notification)

**Example**:
```rust
let actions = vec![
    "default".to_string(), "Open".to_string(),
    "save".to_string(), "Save".to_string(),
    "cancel".to_string(), "Cancel".to_string(),
];
```

## Best Practices

### For Notification Listeners (Our Use Case)

1. **Handle all hint types**
   - Parse urgency to prioritize
   - Extract desktop-entry for app identification
   - Handle image-data for custom icons

2. **Respect urgency levels**
   - Critical: Show immediately, don't timeout
   - Normal: Standard display
   - Low: Can be less prominent

3. **Support standard capabilities**
   - Report accurate capabilities
   - Implement actions if supported
   - Handle body markup safely

4. **Handle rapid notifications**
   - Queue notifications
   - Group by application
   - Implement rate limiting

5. **Preserve notification context**
   - Store complete notification data
   - Keep action handlers active
   - Track notification lifecycle

### Security Considerations

1. **Sanitize HTML markup**
   ```rust
   fn sanitize_markup(text: &str) -> String {
       // Only allow: <b>, <i>, <u>, <a>
       // Strip: <script>, <iframe>, etc.
   }
   ```

2. **Validate URLs**
   ```rust
   fn is_safe_url(url: &str) -> bool {
       matches!(
           Url::parse(url).map(|u| u.scheme()),
           Ok("http" | "https" | "file" | "mailto")
       )
   }
   ```

3. **Limit resource usage**
   - Maximum body length: 500 characters
   - Maximum actions: 10
   - Maximum image size: 256x256px

## Implementation Checklist

For our applet:

- [ ] Subscribe to NotificationClosed signals
- [ ] Subscribe to ActionInvoked signals
- [ ] Parse all standard hints
- [ ] Handle urgency levels
- [ ] Support notification actions
- [ ] Render body markup safely
- [ ] Handle image-data hints
- [ ] Respect transient hint
- [ ] Implement notification timeout
- [ ] Group notifications by app
- [ ] Persist notification history
- [ ] Handle notification replacement

## Testing Commands

```bash
# Simple notification
notify-send "Summary" "Body"

# With urgency
notify-send -u critical "Critical" "Important!"

# With icon
notify-send -i firefox "Firefox" "Download complete"

# With action
notify-send -A "action=Label" "Title" "Body"

# With hints
notify-send -h string:category:email.arrived "Email" "New message"

# With timeout
notify-send -t 10000 "Title" "10 second timeout"

# Replace notification
ID=$(notify-send -p "Original" "Message")
notify-send -r $ID "Updated" "Message"
```

## Reference

- [Specification](https://specifications.freedesktop.org/notification/latest/)
- [D-Bus Specification](https://dbus.freedesktop.org/doc/dbus-specification.html)

---

**Last Updated**: 2025-01-13
