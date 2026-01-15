// D-Bus notification listener module
//
// This module handles D-Bus communication for receiving notifications
// according to the freedesktop.org notification specification.
//
// Reference: https://specifications.freedesktop.org/notification-spec/latest/

pub mod types;

// Re-export commonly used types
pub use types::{
    parse_actions, parse_hints, ImageData, Notification, NotificationAction, NotificationHints,
    Urgency,
};

// TODO: Implement NotificationListener with iced Subscription
// pub fn subscribe() -> iced::Subscription<crate::Message> { ... }
