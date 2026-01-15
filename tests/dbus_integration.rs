// Integration tests for D-Bus types and parsing
//
// Tests the notification data structures and hint parsing
// without requiring actual D-Bus connection.

use cosmic_applet_notifications::dbus::{
    parse_actions, parse_hints, Notification, NotificationAction, NotificationHints, Urgency,
};
use std::collections::HashMap;

#[test]
fn test_notification_default() {
    let notification = Notification {
        id: 1,
        app_name: "test".to_string(),
        replaces_id: 0,
        app_icon: "".to_string(),
        summary: "Test".to_string(),
        body: "".to_string(),
        actions: vec![],
        hints: NotificationHints::default(),
        raw_hints: HashMap::new(),
        expire_timeout: 0,
        timestamp: chrono::Local::now(),
    };

    assert_eq!(notification.id, 1);
    assert_eq!(notification.app_name, "test");
    assert!(!notification.is_transient());
    assert!(!notification.is_resident());
    assert_eq!(notification.urgency(), Urgency::Normal);
}

#[test]
fn test_parse_actions_empty() {
    let actions = vec![];
    let result = parse_actions(&actions);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_parse_actions_valid() {
    let actions = vec![
        "default".to_string(),
        "Open".to_string(),
        "reply".to_string(),
        "Reply".to_string(),
    ];
    let result = parse_actions(&actions);

    assert_eq!(result.len(), 2);
    assert_eq!(result[0].key, "default");
    assert_eq!(result[0].label, "Open");
    assert_eq!(result[1].key, "reply");
    assert_eq!(result[1].label, "Reply");
}

#[test]
fn test_parse_actions_odd_length() {
    // Odd number of elements should be handled gracefully
    let actions = vec!["default".to_string(), "Open".to_string(), "orphan".to_string()];
    let result = parse_actions(&actions);

    // Should only parse the complete pairs
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].key, "default");
}

#[test]
fn test_notification_hints_urgency() {
    let hints = NotificationHints {
        urgency: Urgency::Critical,
        ..Default::default()
    };

    assert_eq!(hints.urgency, Urgency::Critical);
}

#[test]
fn test_notification_hints_transient() {
    let hints = NotificationHints {
        transient: true,
        ..Default::default()
    };

    assert!(hints.transient);
}

#[test]
fn test_notification_hints_resident() {
    let hints = NotificationHints {
        resident: true,
        ..Default::default()
    };

    assert!(hints.resident);
}

#[test]
fn test_notification_action_clone() {
    let action = NotificationAction {
        key: "test".to_string(),
        label: "Test Action".to_string(),
    };

    let cloned = action.clone();
    assert_eq!(cloned.key, "test");
    assert_eq!(cloned.label, "Test Action");
}

#[test]
fn test_urgency_ordering() {
    assert!(Urgency::Low < Urgency::Normal);
    assert!(Urgency::Normal < Urgency::Critical);
    assert_eq!(Urgency::Normal, Urgency::Normal);
}

#[test]
fn test_notification_with_actions() {
    let actions = vec![
        NotificationAction {
            key: "view".to_string(),
            label: "View".to_string(),
        },
        NotificationAction {
            key: "dismiss".to_string(),
            label: "Dismiss".to_string(),
        },
    ];

    let notification = Notification {
        id: 1,
        app_name: "test".to_string(),
        replaces_id: 0,
        app_icon: "".to_string(),
        summary: "Test".to_string(),
        body: "".to_string(),
        actions: actions.clone(),
        hints: NotificationHints::default(),
        raw_hints: HashMap::new(),
        expire_timeout: 0,
        timestamp: chrono::Local::now(),
    };

    assert_eq!(notification.actions.len(), 2);
    assert_eq!(notification.actions[0].key, "view");
    assert_eq!(notification.actions[1].key, "dismiss");
}

#[test]
fn test_notification_with_body() {
    let notification = Notification {
        id: 1,
        app_name: "test".to_string(),
        replaces_id: 0,
        app_icon: "".to_string(),
        summary: "Test Summary".to_string(),
        body: "This is a test notification with a body".to_string(),
        actions: vec![],
        hints: NotificationHints::default(),
        raw_hints: HashMap::new(),
        expire_timeout: 5000,
        timestamp: chrono::Local::now(),
    };

    assert!(!notification.body.is_empty());
    assert_eq!(notification.body, "This is a test notification with a body");
    assert_eq!(notification.expire_timeout, 5000);
}

#[test]
fn test_notification_replacement() {
    let notification = Notification {
        id: 2,
        app_name: "test".to_string(),
        replaces_id: 1,
        app_icon: "".to_string(),
        summary: "Replacement".to_string(),
        body: "".to_string(),
        actions: vec![],
        hints: NotificationHints::default(),
        raw_hints: HashMap::new(),
        expire_timeout: 0,
        timestamp: chrono::Local::now(),
    };

    assert_eq!(notification.replaces_id, 1);
    assert_eq!(notification.id, 2);
}

#[test]
fn test_notification_with_icon() {
    let notification = Notification {
        id: 1,
        app_name: "test".to_string(),
        replaces_id: 0,
        app_icon: "dialog-information".to_string(),
        summary: "Test".to_string(),
        body: "".to_string(),
        actions: vec![],
        hints: NotificationHints::default(),
        raw_hints: HashMap::new(),
        expire_timeout: 0,
        timestamp: chrono::Local::now(),
    };

    assert_eq!(notification.app_icon, "dialog-information");
}
