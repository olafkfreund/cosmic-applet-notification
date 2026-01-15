// D-Bus notification listener module
//
// This module handles D-Bus communication for receiving notifications
// according to the freedesktop.org notification specification.
//
// Reference: https://specifications.freedesktop.org/notification-spec/latest/

pub mod listener;
pub mod sender;
pub mod types;

// Re-export commonly used types
pub use types::{
    parse_actions, parse_hints, ImageData, Notification, NotificationAction, NotificationHints,
    Urgency,
};

// Re-export listener subscription function
pub use listener::subscribe;

// Re-export sender functions
pub use sender::{send_action_invoked, send_notification_closed, CloseReason};
