// D-Bus signal sender
//
// Handles sending signals back to notification senders, particularly ActionInvoked.

use zbus::Connection;

/// Send an ActionInvoked signal to notify the sender that an action was clicked
///
/// According to the freedesktop.org notification spec:
/// - Signal: org.freedesktop.Notifications.ActionInvoked
/// - Parameters: (UINT32 id, STRING action_key)
///
/// Returns:
/// - Ok(()) if signal sent successfully
/// - Err if D-Bus communication failed
pub async fn send_action_invoked(
    notification_id: u32,
    action_key: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!(
        "Sending ActionInvoked signal for notification {}: action={}",
        notification_id,
        action_key
    );

    // Get session bus connection
    let connection = Connection::session().await?;

    // Send ActionInvoked signal
    connection
        .emit_signal(
            None::<()>, // destination (None = broadcast)
            "/org/freedesktop/Notifications",
            "org.freedesktop.Notifications",
            "ActionInvoked",
            &(notification_id, action_key),
        )
        .await?;

    tracing::debug!("ActionInvoked signal sent successfully");
    Ok(())
}

/// Send a NotificationClosed signal to notify the sender that a notification was closed
///
/// According to the freedesktop.org notification spec:
/// - Signal: org.freedesktop.Notifications.NotificationClosed
/// - Parameters: (UINT32 id, UINT32 reason)
///
/// Reason codes:
/// - 1: Expired
/// - 2: Dismissed by user
/// - 3: Closed by CloseNotification call
/// - 4: Undefined/reserved
///
/// Returns:
/// - Ok(()) if signal sent successfully
/// - Err if D-Bus communication failed
pub async fn send_notification_closed(
    notification_id: u32,
    reason: CloseReason,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::debug!(
        "Sending NotificationClosed signal for notification {}: reason={:?}",
        notification_id,
        reason
    );

    // Get session bus connection
    let connection = Connection::session().await?;

    // Send NotificationClosed signal
    connection
        .emit_signal(
            None::<()>,
            "/org/freedesktop/Notifications",
            "org.freedesktop.Notifications",
            "NotificationClosed",
            &(notification_id, reason as u32),
        )
        .await?;

    tracing::trace!("NotificationClosed signal sent successfully");
    Ok(())
}

/// Reason a notification was closed
///
/// Maps to the reason parameter in NotificationClosed signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CloseReason {
    /// Notification expired (timeout)
    Expired = 1,
    /// Dismissed by user (e.g., clicking dismiss button)
    Dismissed = 2,
    /// Closed via CloseNotification D-Bus method
    Closed = 3,
    /// Undefined/reserved reason
    Undefined = 4,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_close_reason_values() {
        assert_eq!(CloseReason::Expired as u32, 1);
        assert_eq!(CloseReason::Dismissed as u32, 2);
        assert_eq!(CloseReason::Closed as u32, 3);
        assert_eq!(CloseReason::Undefined as u32, 4);
    }
}
