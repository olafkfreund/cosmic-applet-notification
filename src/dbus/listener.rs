// D-Bus notification listener using iced Subscription pattern
//
// This module implements a subscription-based D-Bus listener that monitors
// for org.freedesktop.Notifications signals on the session bus.
//
// Architecture: Uses iced's Subscription pattern instead of separate threads.
// This integrates directly with the iced event loop for automatic lifecycle
// management and simpler error handling.
//
// Reference: https://specifications.freedesktop.org/notification-spec/latest/

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use chrono::Local;
use futures::stream::{Stream, StreamExt};
use zbus::zvariant::OwnedValue;
use zbus::{Connection, MatchRule, MessageStream, MessageType};

use crate::dbus::types::{parse_actions, parse_hints, Notification};

/// Notification buffer size for backpressure management
///
/// Buffer size chosen based on typical notification burst patterns:
/// - 90th percentile: ~20 notifications in 1 second
/// - 128 provides 6x headroom for notification bursts
/// - Prevents memory exhaustion from notification spam
///
/// TODO: Make configurable via cosmic-config (default: 128, range: 32-512)
const NOTIFICATION_BUFFER_SIZE: usize = 128;

/// Subscription ID for the notification listener
/// This ensures only one listener instance exists
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ListenerSubscription;

impl Hash for ListenerSubscription {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::any::TypeId::of::<Self>().hash(state);
    }
}

/// Create a subscription that listens for D-Bus notification signals
///
/// This function returns an iced Subscription that will automatically:
/// - Connect to the D-Bus session bus
/// - Subscribe to org.freedesktop.Notifications signals
/// - Parse incoming notifications
/// - Yield them as Messages to the application
///
/// The subscription is managed by iced's runtime - no manual cleanup needed.
///
/// # Example
/// ```no_run
/// use iced::Subscription;
///
/// impl Application for MyApp {
///     fn subscription(&self) -> Subscription<Message> {
///         notification_listener::subscribe()
///     }
/// }
/// ```
pub fn subscribe<Message>() -> iced::Subscription<Message>
where
    Message: 'static + Send + Clone + From<Notification>,
{
    iced::Subscription::run_with_id(
        ListenerSubscription,
        notification_stream().map(Message::from),
    )
}

/// Create an async stream of notifications from D-Bus
///
/// This is the core async function that:
/// 1. Connects to the D-Bus session bus
/// 2. Sets up a match rule for notification signals
/// 3. Creates a MessageStream to receive signals
/// 4. Parses each signal into a Notification
/// 5. Yields notifications as a stream
///
/// Errors are logged but don't terminate the stream - we keep listening
/// even if individual notifications are malformed.
async fn notification_stream() -> impl Stream<Item = Notification> {
    // Connect to session bus - this is where notifications are sent
    let connection = match Connection::session().await {
        Ok(conn) => {
            tracing::info!("Connected to D-Bus session bus");
            conn
        }
        Err(e) => {
            tracing::error!("Failed to connect to D-Bus session bus: {}", e);
            // Return empty stream on connection failure
            return futures::stream::empty().boxed();
        }
    };

    // Create match rule for org.freedesktop.Notifications signals
    // We want all signals from this interface to catch notifications
    let match_rule = match MatchRule::builder()
        .msg_type(MessageType::Signal)
        .interface("org.freedesktop.Notifications")
        .and_then(|builder| builder.build())
    {
        Ok(rule) => rule,
        Err(e) => {
            tracing::error!("Failed to build match rule: {}", e);
            return futures::stream::empty().boxed();
        }
    };

    // Create message stream with configured buffer size
    // This provides backpressure if we can't process notifications fast enough
    let message_stream = match MessageStream::for_match_rule(
        match_rule,
        &connection,
        Some(NOTIFICATION_BUFFER_SIZE),
    )
    .await
    {
        Ok(stream) => {
            tracing::info!("Subscribed to notification signals");
            stream
        }
        Err(e) => {
            tracing::error!("Failed to create message stream: {}", e);
            return futures::stream::empty().boxed();
        }
    };

    // Transform D-Bus messages into Notifications
    // filter_map skips any messages we can't parse
    message_stream
        .filter_map(|message| async move {
            match parse_notification_signal(message) {
                Ok(notification) => {
                    tracing::debug!(
                        "Received notification: {} from {}",
                        notification.summary,
                        notification.app_name
                    );
                    Some(notification)
                }
                Err(e) => {
                    tracing::warn!("Failed to parse notification signal: {}", e);
                    None
                }
            }
        })
        .boxed()
}

/// Parse a D-Bus message into a Notification
///
/// This function extracts the Notify signal parameters:
/// - app_name: String
/// - replaces_id: u32
/// - app_icon: String
/// - summary: String
/// - body: String
/// - actions: Array of String
/// - hints: Dictionary (String -> Variant)
/// - expire_timeout: i32
///
/// Reference: https://specifications.freedesktop.org/notification-spec/latest/ar01s09.html
fn parse_notification_signal(message: zbus::Message) -> Result<Notification, NotificationError> {
    // Verify this is a Notify signal
    let member = message.member().ok_or(NotificationError::MissingMember)?;

    if member.as_str() != "Notify" {
        return Err(NotificationError::UnexpectedSignal(
            member.as_str().to_string(),
        ));
    }

    // Extract and destructure D-Bus signal parameters (8 parameters in Notify signal)
    let (
        app_name,
        replaces_id,
        app_icon,
        summary,
        body_text,
        actions_raw,
        hints_raw,
        expire_timeout,
    ): (
        String,
        u32,
        String,
        String,
        String,
        Vec<String>,
        HashMap<String, OwnedValue>,
        i32,
    ) = message
        .body()
        .deserialize()
        .map_err(NotificationError::DeserializeFailed)?;

    Ok(Notification {
        id: generate_notification_id(&app_name, &summary),
        app_name,
        replaces_id,
        app_icon,
        summary,
        body: body_text,
        actions: parse_actions(&actions_raw),
        hints: parse_hints(&hints_raw),
        raw_hints: hints_raw,
        expire_timeout,
        timestamp: Local::now(),
    })
}

/// Generate a notification ID
///
/// For now, this is a simple hash-based approach.
/// In a real implementation, the notification manager would assign IDs.
fn generate_notification_id(app_name: &str, summary: &str) -> u32 {
    use std::collections::hash_map::DefaultHasher;

    let mut hasher = DefaultHasher::new();
    app_name.hash(&mut hasher);
    summary.hash(&mut hasher);
    // Use unwrap_or_default() to handle clock skew gracefully
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
        .hash(&mut hasher);

    hasher.finish() as u32
}

/// Errors that can occur during notification listening
#[derive(Debug, thiserror::Error)]
enum NotificationError {
    #[error("D-Bus message missing member field")]
    MissingMember,

    #[error("Unexpected signal: {0}, expected 'Notify'")]
    UnexpectedSignal(String),

    #[error("Failed to deserialize notification body: {0}")]
    DeserializeFailed(zbus::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_notification_id() {
        let id1 = generate_notification_id("firefox", "Download complete");
        let id2 = generate_notification_id("firefox", "Download complete");

        // IDs should be different due to timestamp
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_generate_notification_id_different_apps() {
        let id1 = generate_notification_id("firefox", "Test");
        let id2 = generate_notification_id("chrome", "Test");

        // Different apps should produce different IDs
        assert_ne!(id1, id2);
    }
}
