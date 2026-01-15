// COSMIC Notification Applet Library
//
// This library provides the core functionality for the notification applet
// including D-Bus communication, notification management, and UI components.

pub mod accessibility;
pub mod config;
pub mod dbus;
pub mod manager;
pub mod ui;

// Re-export commonly used types
pub use dbus::{Notification, NotificationAction, NotificationHints, Urgency};
