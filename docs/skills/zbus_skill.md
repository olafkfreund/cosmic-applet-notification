# zbus Skill: D-Bus Communication for COSMIC Notification Applet

## Overview

This skill provides comprehensive knowledge about using zbus for D-Bus communication in the COSMIC Notification Applet. zbus is a pure Rust D-Bus library that provides safe, async, and ergonomic D-Bus communication.

## Core Concepts

### D-Bus Session Bus
The session bus is a per-user message bus that applications use for inter-process communication within a user session.

```rust
use zbus::Connection;

// Connect to session bus
let connection = Connection::session().await?;
```

### freedesktop.org Notifications Interface

The notification specification defines:
- **Interface**: `org.freedesktop.Notifications`
- **Object Path**: `/org/freedesktop/Notifications`
- **Methods**: Notify, CloseNotification, GetCapabilities, GetServerInformation
- **Signals**: NotificationClosed, ActionInvoked

## Implementation Patterns

### Signal Subscription (Listener Mode)

For our applet, we want to **listen** to notification signals without claiming the service name:

```rust
use zbus::{Connection, MessageStream, MessageType};
use zbus::names::InterfaceName;
use futures::stream::StreamExt;

pub struct NotificationListener {
    connection: Connection,
}

impl NotificationListener {
    pub async fn new() -> zbus::Result<Self> {
        let connection = Connection::session().await?;
        Ok(Self { connection })
    }
    
    pub async fn subscribe(&self) -> zbus::Result<MessageStream> {
        // Create match rule for notification signals
        let match_rule = zbus::MatchRule::builder()
            .msg_type(MessageType::Signal)
            .interface("org.freedesktop.Notifications")?
            .build();
        
        // Subscribe to signals
        let stream = self.connection.add_match(match_rule).await?;
        Ok(stream)
    }
    
    pub async fn listen<F>(&self, handler: F) -> zbus::Result<()>
    where
        F: Fn(NotificationSignal) + Send + 'static,
    {
        let mut stream = self.subscribe().await?;
        
        while let Some(msg) = stream.next().await {
            if let Ok(signal) = msg.and_then(|m| {
                NotificationSignal::try_from_message(&m)
            }) {
                handler(signal);
            }
        }
        
        Ok(())
    }
}
```

### Notification Signal Parsing

```rust
use serde::{Deserialize, Serialize};
use zbus::zvariant::{Type, Value, OwnedValue};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct NotificationSignal {
    pub app_name: String,
    pub replaces_id: u32,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub actions: Vec<String>,
    pub hints: HashMap<String, OwnedValue>,
    pub expire_timeout: i32,
}

impl NotificationSignal {
    pub fn try_from_message(msg: &zbus::Message) -> zbus::Result<Self> {
        let header = msg.header();
        
        // Verify this is a Notify signal
        if header.interface()?.as_str() != "org.freedesktop.Notifications" {
            return Err(zbus::Error::InvalidField);
        }
        
        // Extract body
        let body = msg.body();
        let (app_name, replaces_id, app_icon, summary, body_text, actions, hints, expire_timeout) =
            body.deserialize::<(String, u32, String, String, String, Vec<String>, 
                               HashMap<String, OwnedValue>, i32)>()?;
        
        Ok(Self {
            app_name,
            replaces_id,
            app_icon,
            summary,
            body: body_text,
            actions,
            hints,
            expire_timeout,
        })
    }
}
```

### Hint Parsing

Notification hints provide additional metadata:

```rust
use zbus::zvariant::{OwnedValue, Type};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Urgency {
    Low = 0,
    Normal = 1,
    Critical = 2,
}

impl NotificationSignal {
    pub fn urgency(&self) -> Urgency {
        self.hints
            .get("urgency")
            .and_then(|v| {
                v.downcast_ref::<u8>()
                    .ok()
                    .copied()
            })
            .and_then(|u| match u {
                0 => Some(Urgency::Low),
                1 => Some(Urgency::Normal),
                2 => Some(Urgency::Critical),
                _ => None,
            })
            .unwrap_or(Urgency::Normal)
    }
    
    pub fn category(&self) -> Option<String> {
        self.hints
            .get("category")
            .and_then(|v| v.downcast_ref::<String>().ok())
            .cloned()
    }
    
    pub fn desktop_entry(&self) -> Option<String> {
        self.hints
            .get("desktop-entry")
            .and_then(|v| v.downcast_ref::<String>().ok())
            .cloned()
    }
    
    pub fn image_data(&self) -> Option<ImageData> {
        // Parse image-data hint
        self.hints
            .get("image-data")
            .and_then(|v| ImageData::try_from_value(v).ok())
    }
}

#[derive(Debug, Clone)]
pub struct ImageData {
    pub width: i32,
    pub height: i32,
    pub rowstride: i32,
    pub has_alpha: bool,
    pub bits_per_sample: i32,
    pub channels: i32,
    pub data: Vec<u8>,
}
```

### Action Invocation (Sending Signals Back)

When user clicks an action button, we need to send ActionInvoked signal:

```rust
use zbus::proxy;

#[proxy(
    interface = "org.freedesktop.Notifications",
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
trait Notifications {
    /// Invoke notification action
    #[zbus(signal)]
    fn action_invoked(&self, id: u32, action_key: &str) -> zbus::Result<()>;
    
    /// Close notification
    #[zbus(signal)]
    fn notification_closed(&self, id: u32, reason: u32) -> zbus::Result<()>;
}

pub async fn invoke_action(
    connection: &Connection,
    notification_id: u32,
    action_key: &str,
) -> zbus::Result<()> {
    let proxy = NotificationsProxy::new(connection).await?;
    proxy.action_invoked(notification_id, action_key).await
}

pub enum CloseReason {
    Expired = 1,
    DismissedByUser = 2,
    ClosedByCall = 3,
    Undefined = 4,
}

pub async fn close_notification(
    connection: &Connection,
    notification_id: u32,
    reason: CloseReason,
) -> zbus::Result<()> {
    let proxy = NotificationsProxy::new(connection).await?;
    proxy.notification_closed(notification_id, reason as u32).await
}
```

## Integration with tokio

zbus is async and works seamlessly with tokio:

```rust
use tokio::sync::mpsc;

pub async fn start_listener(
    tx: mpsc::UnboundedSender<NotificationSignal>,
) -> zbus::Result<()> {
    let listener = NotificationListener::new().await?;
    
    // Spawn listener task
    tokio::spawn(async move {
        listener
            .listen(move |signal| {
                let _ = tx.send(signal);
            })
            .await
            .expect("Listener failed");
    });
    
    Ok(())
}
```

## Error Handling

Common errors and how to handle them:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DBusError {
    #[error("Failed to connect to D-Bus: {0}")]
    ConnectionFailed(#[from] zbus::Error),
    
    #[error("Failed to parse notification: {0}")]
    ParseError(String),
    
    #[error("Invalid message format")]
    InvalidMessage,
    
    #[error("D-Bus connection lost")]
    ConnectionLost,
}

pub async fn robust_connect() -> Result<Connection, DBusError> {
    let mut attempts = 0;
    const MAX_ATTEMPTS: u32 = 5;
    
    loop {
        match Connection::session().await {
            Ok(conn) => return Ok(conn),
            Err(e) if attempts < MAX_ATTEMPTS => {
                attempts += 1;
                tracing::warn!("D-Bus connection attempt {} failed: {}", attempts, e);
                tokio::time::sleep(Duration::from_millis(100 * attempts as u64)).await;
            }
            Err(e) => return Err(DBusError::ConnectionFailed(e)),
        }
    }
}
```

## Testing

### Unit Testing with Mock

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_notification_parsing() {
        let signal = NotificationSignal {
            app_name: "test-app".to_string(),
            replaces_id: 0,
            app_icon: "test-icon".to_string(),
            summary: "Test Summary".to_string(),
            body: "Test body".to_string(),
            actions: vec!["default".to_string(), "Open".to_string()],
            hints: HashMap::new(),
            expire_timeout: 5000,
        };
        
        assert_eq!(signal.urgency(), Urgency::Normal);
        assert_eq!(signal.app_name, "test-app");
    }
}
```

### Integration Testing

```rust
#[tokio::test]
async fn test_receive_notification() {
    let (tx, mut rx) = mpsc::unbounded_channel();
    
    // Start listener
    start_listener(tx).await.unwrap();
    
    // Send test notification
    std::process::Command::new("notify-send")
        .args(&["Test", "Message"])
        .spawn()
        .unwrap();
    
    // Receive notification
    let notification = tokio::time::timeout(
        Duration::from_secs(5),
        rx.recv()
    ).await.unwrap().unwrap();
    
    assert_eq!(notification.summary, "Test");
}
```

## Best Practices

1. **Always handle disconnections**
   - Implement reconnection logic
   - Use exponential backoff

2. **Validate all input**
   - Check string lengths
   - Validate action keys
   - Sanitize HTML in body

3. **Use tracing for debugging**
   ```rust
   tracing::debug!(
       app_name = %signal.app_name,
       summary = %signal.summary,
       "Received notification"
   );
   ```

4. **Handle rapid notifications**
   - Use bounded channels
   - Implement rate limiting
   - Drop old notifications if necessary

5. **Don't block the D-Bus thread**
   - Process notifications in separate task
   - Use channels for communication
   - Keep handlers fast

## Common Pitfalls

❌ **Don't claim the service name**
```rust
// Wrong - this would conflict with other notification daemons
connection.request_name("org.freedesktop.Notifications").await?;
```

✅ **Do subscribe to signals**
```rust
// Right - just listen to signals
let stream = connection.add_match(match_rule).await?;
```

❌ **Don't block in signal handlers**
```rust
// Wrong - this blocks the D-Bus thread
listener.listen(|signal| {
    thread::sleep(Duration::from_secs(1)); // BAD!
}).await?;
```

✅ **Do use channels**
```rust
// Right - send to channel, process elsewhere
let (tx, rx) = mpsc::unbounded_channel();
listener.listen(move |signal| {
    let _ = tx.send(signal); // Fast, non-blocking
}).await?;
```

## Reference

- [zbus Documentation](https://docs.rs/zbus/)
- [freedesktop.org Notifications Spec](https://specifications.freedesktop.org/notification/latest/)
- [D-Bus Specification](https://dbus.freedesktop.org/doc/dbus-specification.html)

---

**Last Updated**: 2025-01-13
