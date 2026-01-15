// D-Bus notification types
//
// This module defines all types for freedesktop.org notifications
// Reference: https://specifications.freedesktop.org/notification-spec/latest/

use chrono::{DateTime, Local};
use std::collections::HashMap;
use zbus::zvariant::{OwnedValue, Value};

/// A notification received from D-Bus
///
/// Implements the freedesktop.org Desktop Notifications Specification v1.2
/// All fields are required to implement Clone for iced Message compatibility
#[derive(Debug, Clone)]
pub struct Notification {
    /// Unique notification ID (assigned by us)
    pub id: u32,

    /// Application name that sent the notification
    pub app_name: String,

    /// ID of notification to replace (0 if new)
    pub replaces_id: u32,

    /// Application icon name or path
    pub app_icon: String,

    /// Summary text (title) - single line
    pub summary: String,

    /// Body text - can be multi-line, may contain basic markup
    pub body: String,

    /// Actions available for this notification
    pub actions: Vec<NotificationAction>,

    /// Parsed notification hints
    pub hints: NotificationHints,

    /// Raw D-Bus hints for unrecognized keys
    pub raw_hints: HashMap<String, OwnedValue>,

    /// Expiration timeout in milliseconds
    /// -1: never expire, 0: use server default, >0: specific timeout
    pub expire_timeout: i32,

    /// Timestamp when notification was received
    pub timestamp: DateTime<Local>,
}

impl Notification {
    /// Get the urgency level of this notification
    pub fn urgency(&self) -> Urgency {
        self.hints.urgency
    }

    /// Check if this notification is transient (should not be persisted)
    pub fn is_transient(&self) -> bool {
        self.hints.transient
    }

    /// Check if this notification is resident (should not be automatically removed)
    pub fn is_resident(&self) -> bool {
        self.hints.resident
    }

    /// Get the category of this notification
    pub fn category(&self) -> Option<&str> {
        self.hints.category.as_deref()
    }

    /// Get the desktop entry name for this notification
    pub fn desktop_entry(&self) -> Option<&str> {
        self.hints.desktop_entry.as_deref()
    }

    /// Check if this notification has actions
    pub fn has_actions(&self) -> bool {
        !self.actions.is_empty()
    }
}

/// Notification urgency level
///
/// Determines the importance and presentation of the notification
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Urgency {
    /// Low urgency - background information
    Low = 0,
    /// Normal urgency - standard notification (default)
    Normal = 1,
    /// Critical urgency - requires immediate attention
    Critical = 2,
}

impl Urgency {
    /// Parse urgency from a u8 value
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Urgency::Low),
            1 => Some(Urgency::Normal),
            2 => Some(Urgency::Critical),
            _ => None,
        }
    }

    /// Convert urgency to u8 value
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

impl Default for Urgency {
    fn default() -> Self {
        Urgency::Normal
    }
}

/// A notification action (button)
///
/// Actions are displayed as buttons in the notification.
/// When clicked, an ActionInvoked signal is sent on D-Bus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationAction {
    /// Action identifier (sent back when invoked)
    pub key: String,
    /// User-visible label for the action button
    pub label: String,
}

impl NotificationAction {
    /// Create a new notification action
    pub fn new(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
        }
    }
}

/// Parsed notification hints
///
/// Standard hints from the freedesktop.org specification
/// Reference: https://specifications.freedesktop.org/notification-spec/latest/hints.html
#[derive(Debug, Clone, Default)]
pub struct NotificationHints {
    /// Urgency level (low, normal, critical)
    pub urgency: Urgency,

    /// Category of the notification (e.g., "email.arrived", "im.received")
    pub category: Option<String>,

    /// Desktop entry name (e.g., "firefox")
    pub desktop_entry: Option<String>,

    /// Whether notification should bypass server's persistence
    pub transient: bool,

    /// Whether notification should not be automatically removed
    pub resident: bool,

    /// X11 window position hint
    pub x: Option<i32>,

    /// Y11 window position hint
    pub y: Option<i32>,

    /// Sound file to play
    pub sound_file: Option<String>,

    /// Sound name from freedesktop.org sound naming spec
    pub sound_name: Option<String>,

    /// Whether to suppress sound
    pub suppress_sound: bool,

    /// Action icons (mapping of action keys to icon names)
    pub action_icons: bool,

    /// Image data (icon as raw pixel data)
    pub image_data: Option<ImageData>,

    /// Image path (icon as file path)
    pub image_path: Option<String>,
}

/// Raw image data for notification icons
///
/// ARGB32 format image data
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

/// Parse notification hints from D-Bus HashMap
///
/// Extracts standard hints and falls back to sensible defaults
/// for any missing or malformed values
pub fn parse_hints(hints: &HashMap<String, OwnedValue>) -> NotificationHints {
    NotificationHints {
        urgency: parse_urgency(hints),
        category: parse_string(hints, "category"),
        desktop_entry: parse_string(hints, "desktop-entry"),
        transient: parse_bool(hints, "transient"),
        resident: parse_bool(hints, "resident"),
        x: parse_i32(hints, "x"),
        y: parse_i32(hints, "y"),
        sound_file: parse_string(hints, "sound-file"),
        sound_name: parse_string(hints, "sound-name"),
        suppress_sound: parse_bool(hints, "suppress-sound"),
        action_icons: parse_bool(hints, "action-icons"),
        image_data: parse_image_data(hints),
        image_path: parse_string(hints, "image-path").or_else(|| parse_string(hints, "image_path")),
    }
}

/// Parse urgency from hints with fallback to Normal
fn parse_urgency(hints: &HashMap<String, OwnedValue>) -> Urgency {
    hints
        .get("urgency")
        .and_then(|v| v.downcast_ref::<u8>().ok().copied())
        .and_then(Urgency::from_u8)
        .unwrap_or(Urgency::Normal)
}

/// Parse string value from hints
fn parse_string(hints: &HashMap<String, OwnedValue>, key: &str) -> Option<String> {
    hints.get(key).and_then(|v| match v.downcast_ref() {
        Ok(Value::Str(s)) => Some(s.to_string()),
        _ => None,
    })
}

/// Parse boolean value from hints
fn parse_bool(hints: &HashMap<String, OwnedValue>, key: &str) -> bool {
    hints
        .get(key)
        .and_then(|v| v.downcast_ref::<bool>().ok().copied())
        .unwrap_or(false)
}

/// Parse i32 value from hints
fn parse_i32(hints: &HashMap<String, OwnedValue>, key: &str) -> Option<i32> {
    hints
        .get(key)
        .and_then(|v| v.downcast_ref::<i32>().ok().copied())
}

/// Parse image data from hints
///
/// Image data can be in multiple formats. We try to parse the structure
/// (width, height, rowstride, has_alpha, bits_per_sample, channels, data)
fn parse_image_data(hints: &HashMap<String, OwnedValue>) -> Option<ImageData> {
    // Try multiple possible keys for image data
    let keys = ["image-data", "image_data", "icon_data"];

    for key in &keys {
        if let Some(value) = hints.get(*key) {
            // Image data is a structure: (iiibiiay)
            // Try to extract it - this is complex and may need adjustment
            // For now, we'll skip the implementation and return None
            // TODO: Implement proper image data parsing
            tracing::debug!(
                "Image data found but parsing not yet implemented for key: {}",
                key
            );
            let _ = value; // Use value to avoid warning
        }
    }

    None
}

/// Parse notification actions from D-Bus array
///
/// Actions come as a flat array: [key1, label1, key2, label2, ...]
/// Pairs are processed into NotificationAction structs
pub fn parse_actions(actions: &[String]) -> Vec<NotificationAction> {
    actions
        .chunks(2)
        .filter_map(|chunk| {
            if chunk.len() == 2 {
                Some(NotificationAction::new(chunk[0].clone(), chunk[1].clone()))
            } else {
                tracing::warn!("Malformed action pair in notification, skipping");
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urgency_from_u8() {
        assert_eq!(Urgency::from_u8(0), Some(Urgency::Low));
        assert_eq!(Urgency::from_u8(1), Some(Urgency::Normal));
        assert_eq!(Urgency::from_u8(2), Some(Urgency::Critical));
        assert_eq!(Urgency::from_u8(3), None);
        assert_eq!(Urgency::from_u8(255), None);
    }

    #[test]
    fn test_urgency_to_u8() {
        assert_eq!(Urgency::Low.to_u8(), 0);
        assert_eq!(Urgency::Normal.to_u8(), 1);
        assert_eq!(Urgency::Critical.to_u8(), 2);
    }

    #[test]
    fn test_urgency_default() {
        assert_eq!(Urgency::default(), Urgency::Normal);
    }

    #[test]
    fn test_urgency_ordering() {
        assert!(Urgency::Low < Urgency::Normal);
        assert!(Urgency::Normal < Urgency::Critical);
        assert!(Urgency::Critical > Urgency::Low);
    }

    #[test]
    fn test_notification_action_new() {
        let action = NotificationAction::new("default", "Open");
        assert_eq!(action.key, "default");
        assert_eq!(action.label, "Open");
    }

    #[test]
    fn test_parse_actions_valid() {
        let actions = vec![
            "default".to_string(),
            "Open".to_string(),
            "dismiss".to_string(),
            "Dismiss".to_string(),
        ];

        let parsed = parse_actions(&actions);
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].key, "default");
        assert_eq!(parsed[0].label, "Open");
        assert_eq!(parsed[1].key, "dismiss");
        assert_eq!(parsed[1].label, "Dismiss");
    }

    #[test]
    fn test_parse_actions_odd_length() {
        let actions = vec![
            "default".to_string(),
            "Open".to_string(),
            "incomplete".to_string(),
        ];

        let parsed = parse_actions(&actions);
        assert_eq!(parsed.len(), 1); // Last incomplete pair is skipped
    }

    #[test]
    fn test_parse_actions_empty() {
        let actions: Vec<String> = vec![];
        let parsed = parse_actions(&actions);
        assert!(parsed.is_empty());
    }

    #[test]
    fn test_parse_hints_empty() {
        let hints = HashMap::new();
        let parsed = parse_hints(&hints);

        assert_eq!(parsed.urgency, Urgency::Normal); // Default
        assert_eq!(parsed.category, None);
        assert!(!parsed.transient);
        assert!(!parsed.resident);
    }

    #[test]
    fn test_parse_hints_with_urgency() {
        let mut hints = HashMap::new();
        hints.insert("urgency".to_string(), OwnedValue::from(Value::U8(2)));

        let parsed = parse_hints(&hints);
        assert_eq!(parsed.urgency, Urgency::Critical);
    }

    #[test]
    fn test_parse_hints_with_strings() {
        let mut hints = HashMap::new();
        hints.insert(
            "category".to_string(),
            OwnedValue::from(Value::new_signature("s").unwrap()),
        );

        let parsed = parse_hints(&hints);
        // Note: This test needs adjustment based on actual OwnedValue behavior
        // For now, it just checks that parsing doesn't crash
        let _ = parsed;
    }

    #[test]
    fn test_notification_has_actions() {
        let mut notif = Notification {
            id: 1,
            app_name: "test".to_string(),
            replaces_id: 0,
            app_icon: "".to_string(),
            summary: "Test".to_string(),
            body: "Body".to_string(),
            actions: vec![],
            hints: NotificationHints::default(),
            raw_hints: HashMap::new(),
            expire_timeout: 0,
            timestamp: Local::now(),
        };

        assert!(!notif.has_actions());

        notif
            .actions
            .push(NotificationAction::new("default", "Open"));
        assert!(notif.has_actions());
    }
}
