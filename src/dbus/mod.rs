// D-Bus notification listener module
//
// This module handles D-Bus communication for receiving notifications
// according to the freedesktop.org notification specification.

use std::collections::HashMap;
use zbus::zvariant::OwnedValue;

/// Notification received from D-Bus
#[derive(Debug, Clone)]
pub struct Notification {
    pub id: u32,
    pub app_name: String,
    pub replaces_id: u32,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub actions: Vec<String>,
    pub hints: HashMap<String, OwnedValue>,
    pub expire_timeout: i32,
}

/// Notification urgency level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Urgency {
    Low = 0,
    Normal = 1,
    Critical = 2,
}

impl Notification {
    /// Extract urgency from hints
    pub fn urgency(&self) -> Urgency {
        self.hints
            .get("urgency")
            .and_then(|v| v.downcast_ref::<u8>().ok().copied())
            .and_then(|u| match u {
                0 => Some(Urgency::Low),
                1 => Some(Urgency::Normal),
                2 => Some(Urgency::Critical),
                _ => None,
            })
            .unwrap_or(Urgency::Normal)
    }
}

// TODO: Implement NotificationListener
// pub struct NotificationListener { ... }
//
// TODO: Implement signal subscription
// pub async fn subscribe_notifications() -> Result<...> { ... }
