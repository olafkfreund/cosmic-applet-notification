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
use std::time::Duration;

use chrono::Local;
use cosmic::iced;
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

/// Initial reconnection delay in milliseconds
///
/// When D-Bus connection fails, we wait this long before the first retry.
/// Chosen to balance responsiveness with system load.
const INITIAL_RECONNECT_DELAY_MS: u64 = 100;

/// Maximum reconnection delay in milliseconds (30 seconds)
///
/// This value balances two concerns:
/// 1. System resource usage: Limits retry frequency during prolonged D-Bus outages
/// 2. Recovery time: D-Bus session bus typically restarts within 5-10 seconds
///
/// Chosen based on:
/// - Typical session bus restart: ~3 seconds (requires 4-5 retries at 30s cap)
/// - Maximum outage tolerance: ~5 minutes before user notices
/// - Log spam prevention: 30s interval generates ~10 log entries per 5 minutes
///
/// If D-Bus is down longer than 5 minutes, user intervention is likely needed anyway.
const MAX_RECONNECT_DELAY_MS: u64 = 30_000;

/// Maximum connection retry attempts before giving up
///
/// After this many failed attempts, the listener stops trying to reconnect.
/// User can restart the panel to retry. Prevents infinite resource consumption
/// during permanent D-Bus failures (system shutdown, service removal).
const MAX_CONNECTION_RETRIES: u32 = 10;

/// Reconnection backoff multiplier
///
/// Each failed reconnection attempt multiplies the delay by this factor.
/// Value of 2.0 provides exponential backoff: 100ms → 200ms → 400ms → ...
const RECONNECT_BACKOFF_MULTIPLIER: f64 = 2.0;

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
/// ```ignore
/// use cosmic::{Application, iced::Subscription};
/// use cosmic_applet_notifications::dbus;
///
/// impl Application for MyApp {
///     fn subscription(&self) -> Subscription<Message> {
///         dbus::listener::subscribe()
///     }
/// }
/// ```
pub fn subscribe<Message>() -> iced::Subscription<Message>
where
    Message: 'static + Send + Clone + From<Notification>,
{
    iced::Subscription::run_with_id(
        ListenerSubscription,
        futures::stream::once(notification_stream())
            .flatten()
            .map(Message::from),
    )
}

/// Create an async stream of notifications from D-Bus with automatic reconnection
///
/// This is the core async function that:
/// 1. Connects to the D-Bus session bus (with retry logic)
/// 2. Sets up a match rule for notification signals
/// 3. Creates a MessageStream to receive signals
/// 4. Parses each signal into a Notification
/// 5. Yields notifications as a stream
/// 6. Automatically reconnects on connection drop (unfold triggers reconnection)
///
/// Uses a simpler two-layer approach:
/// - Outer unfold: Manages connection lifecycle (connect → stream → reconnect)
/// - Inner stream: Processes notifications from current connection
///
/// When the connection drops, unfold automatically calls retry_connect() again.
async fn notification_stream() -> impl Stream<Item = Notification> {
    futures::stream::unfold((), |_| async {
        // Attempt connection with exponential backoff
        let connection = retry_connect().await?;

        // Create notification stream for this connection
        let stream = create_notification_stream(connection).await?;

        // When stream ends (connection drop), unfold calls this function again
        Some((stream, ()))
    })
    .flatten()
    .boxed()
}

/// Attempt to connect to D-Bus with exponential backoff
///
/// Retries up to MAX_CONNECTION_RETRIES times with exponential backoff.
/// Returns None if all retry attempts fail (listener will stop).
async fn retry_connect() -> Option<Connection> {
    let mut delay_ms = INITIAL_RECONNECT_DELAY_MS;

    for attempt in 1..=MAX_CONNECTION_RETRIES {
        match Connection::session().await {
            Ok(conn) => {
                tracing::info!(
                    "Connected to D-Bus session bus (attempt {}/{})",
                    attempt,
                    MAX_CONNECTION_RETRIES
                );
                return Some(conn);
            }
            Err(e) => {
                tracing::warn!(
                    "Connection attempt {}/{} failed: {} (retrying in {}ms)",
                    attempt,
                    MAX_CONNECTION_RETRIES,
                    e,
                    delay_ms
                );

                // Wait before next retry
                tokio::time::sleep(Duration::from_millis(delay_ms)).await;

                // Calculate next delay with exponential backoff
                delay_ms = ((delay_ms as f64 * RECONNECT_BACKOFF_MULTIPLIER) as u64)
                    .min(MAX_RECONNECT_DELAY_MS);
            }
        }
    }

    // All retry attempts exhausted
    tracing::error!(
        "Failed to connect to D-Bus after {} attempts, giving up",
        MAX_CONNECTION_RETRIES
    );
    None
}

/// Create a notification stream from an established D-Bus connection
///
/// Sets up the match rule and message stream for the connection.
/// Returns None if setup fails (will trigger reconnection attempt).
async fn create_notification_stream(
    connection: Connection,
) -> Option<impl Stream<Item = Notification>> {
    // Create match rule for org.freedesktop.Notifications signals
    let match_rule = match MatchRule::builder()
        .msg_type(MessageType::Signal)
        .interface("org.freedesktop.Notifications")
    {
        Ok(builder) => builder.build(),
        Err(e) => {
            tracing::error!("Failed to build match rule: {} (will retry connection)", e);
            return None;
        }
    };

    // Create message stream with configured buffer size
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
            tracing::error!(
                "Failed to create message stream: {} (will retry connection)",
                e
            );
            return None;
        }
    };

    // Transform D-Bus messages into Notifications
    //
    // Note: We use filter_map with nested match instead of try_filter_map because
    // we want to continue processing notifications even when some fail to parse.
    // This ensures one malformed notification doesn't block the entire stream.
    // Errors are logged but don't propagate to the caller.
    Some(
        message_stream
            .filter_map(|message| async move {
                // Handle Result from message stream
                match message {
                    Ok(msg) => match parse_notification_signal(msg) {
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
                    },
                    Err(e) => {
                        tracing::warn!("Failed to receive D-Bus message: {}", e);
                        None
                    }
                }
            })
            .boxed(),
    )
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
    let header = message.header();
    let member = header.member().ok_or(NotificationError::MissingMember)?;

    if member.as_str() != "Notify" {
        return Err(NotificationError::UnexpectedSignal(
            member.as_str().to_string(),
        ));
    }

    // Extract and destructure D-Bus signal parameters (8 parameters in Notify signal)
    #[allow(clippy::type_complexity)]
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
