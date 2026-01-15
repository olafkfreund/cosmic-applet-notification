// Integration tests for NotificationManager
//
// Tests the complete notification lifecycle including filtering,
// history management, and interaction with configuration.

use cosmic_applet_notifications::{
    dbus::{Notification, NotificationHints, Urgency},
    manager::{NotificationAction, NotificationManager},
};
use std::collections::HashMap;

fn create_notification(app_name: &str, summary: &str, urgency: Urgency) -> Notification {
    Notification {
        id: 0,
        app_name: app_name.to_string(),
        replaces_id: 0,
        app_icon: String::new(),
        summary: summary.to_string(),
        body: String::new(),
        actions: Vec::new(),
        hints: NotificationHints {
            urgency,
            ..Default::default()
        },
        raw_hints: HashMap::new(),
        expire_timeout: 0,
        timestamp: chrono::Local::now(),
    }
}

#[test]
fn test_manager_lifecycle() {
    let mut manager = NotificationManager::new();

    // Add notification
    let notification = create_notification("test", "Test 1", Urgency::Normal);
    let action = manager.add_notification(notification);
    assert_eq!(action, NotificationAction::Displayed);
    assert_eq!(manager.active_count(), 1);

    // Remove notification
    let id = manager
        .get_notification_at(0)
        .expect("notification should exist")
        .id;
    assert!(manager.remove_notification(id));
    assert_eq!(manager.active_count(), 0);
}

#[test]
fn test_manager_multiple_notifications() {
    let mut manager = NotificationManager::new();

    // Add multiple notifications
    for i in 0..5 {
        let notif = create_notification("test", &format!("Notification {}", i), Urgency::Normal);
        manager.add_notification(notif);
    }

    assert_eq!(manager.active_count(), 5);
}

#[test]
fn test_manager_filtering_cascade() {
    let mut manager = NotificationManager::new();

    // Configure filters
    manager.set_min_urgency_level(1); // Block low urgency
    manager.set_app_filter("blocked_app".to_string(), false);

    // Test 1: Low urgency blocked by urgency filter
    let low_notif = create_notification("test", "Low", Urgency::Low);
    let action = manager.add_notification(low_notif);
    assert_eq!(action, NotificationAction::AddedToHistoryOnly);

    // Test 2: Normal urgency from allowed app - should display
    let normal_notif = create_notification("test", "Normal", Urgency::Normal);
    let action = manager.add_notification(normal_notif);
    assert_eq!(action, NotificationAction::Displayed);

    // Test 3: Critical from blocked app - blocked by app filter
    let blocked_notif = create_notification("blocked_app", "Blocked", Urgency::Critical);
    let action = manager.add_notification(blocked_notif);
    assert_eq!(action, NotificationAction::AddedToHistoryOnly);

    assert_eq!(manager.active_count(), 1); // Only normal notification displayed
    // Only filtered notifications are in history immediately; active ones go to history when dismissed
    assert_eq!(manager.history().len(), 2); // 2 filtered notifications in history
}

#[test]
fn test_manager_dnd_with_critical_bypass() {
    let mut manager = NotificationManager::new();
    manager.set_do_not_disturb(true);

    // Normal notification blocked by DND
    let normal = create_notification("test", "Normal", Urgency::Normal);
    let action = manager.add_notification(normal);
    assert_eq!(action, NotificationAction::AddedToHistoryOnly);

    // Critical notification bypasses DND
    let critical = create_notification("test", "Critical", Urgency::Critical);
    let action = manager.add_notification(critical);
    assert_eq!(action, NotificationAction::Displayed);

    assert_eq!(manager.active_count(), 1);
}

#[test]
fn test_manager_notification_replacement() {
    let mut manager = NotificationManager::new();

    // Add original notification
    let notif1 = create_notification("test", "Original", Urgency::Normal);
    manager.add_notification(notif1.clone());
    let original_id = manager
        .get_notification_at(0)
        .expect("notification should exist")
        .id;

    assert_eq!(manager.active_count(), 1);

    // Add replacement notification
    let mut notif2 = create_notification("test", "Replacement", Urgency::Normal);
    notif2.replaces_id = original_id;
    manager.add_notification(notif2);

    // Should still have only 1 active notification
    assert_eq!(manager.active_count(), 1);
    assert_eq!(
        manager
            .get_notification_at(0)
            .expect("notification should exist")
            .summary,
        "Replacement"
    );
}

#[test]
fn test_manager_clear_all() {
    let mut manager = NotificationManager::new();

    // Add multiple notifications
    for i in 0..5 {
        let notif = create_notification("test", &format!("Notification {}", i), Urgency::Normal);
        manager.add_notification(notif);
    }

    assert_eq!(manager.active_count(), 5);

    manager.clear_all();

    assert_eq!(manager.active_count(), 0);
    assert_eq!(manager.history().len(), 5); // Moved to history
}

#[test]
fn test_manager_grouped_by_app() {
    let mut manager = NotificationManager::new();

    // Add notifications from different apps
    manager.add_notification(create_notification(
        "firefox",
        "Tab loaded",
        Urgency::Normal,
    ));
    manager.add_notification(create_notification(
        "firefox",
        "Download complete",
        Urgency::Normal,
    ));
    manager.add_notification(create_notification(
        "thunderbird",
        "New email",
        Urgency::Normal,
    ));

    let grouped = manager.get_notifications_by_app();

    assert_eq!(grouped.len(), 2);
    assert_eq!(grouped.get("firefox").unwrap().len(), 2);
    assert_eq!(grouped.get("thunderbird").unwrap().len(), 1);
}

#[test]
fn test_manager_filter_by_urgency() {
    let mut manager = NotificationManager::new();

    manager.add_notification(create_notification("test", "Low", Urgency::Low));
    manager.add_notification(create_notification("test", "Normal", Urgency::Normal));
    manager.add_notification(create_notification("test", "Critical", Urgency::Critical));

    let low = manager.get_by_urgency(Urgency::Low);
    let normal = manager.get_by_urgency(Urgency::Normal);
    let critical = manager.get_by_urgency(Urgency::Critical);

    assert_eq!(low.len(), 1);
    assert_eq!(normal.len(), 1);
    assert_eq!(critical.len(), 1);
}

#[test]
fn test_manager_expiration() {
    let mut manager = NotificationManager::new();

    // Add notification that expired 10 seconds ago
    let mut old_notif = create_notification("test", "Old", Urgency::Normal);
    old_notif.expire_timeout = 5000; // 5 seconds
    old_notif.timestamp = chrono::Local::now() - chrono::Duration::seconds(10);

    // Add recent notification
    let recent_notif = create_notification("test", "Recent", Urgency::Normal);

    manager.add_notification(old_notif);
    manager.add_notification(recent_notif);

    let expired = manager.get_expired_notifications();

    assert_eq!(expired.len(), 1);
}

#[test]
fn test_manager_never_expire_notifications() {
    let mut manager = NotificationManager::new();

    let mut notif = create_notification("test", "Never Expire", Urgency::Normal);
    notif.expire_timeout = -1;
    notif.timestamp = chrono::Local::now() - chrono::Duration::hours(24);

    manager.add_notification(notif);

    let expired = manager.get_expired_notifications();
    assert_eq!(expired.len(), 0);
}

#[test]
fn test_manager_transient_notifications() {
    let mut manager = NotificationManager::new();

    let mut notif = create_notification("test", "Transient", Urgency::Normal);
    notif.hints.transient = true;

    manager.add_notification(notif);

    assert_eq!(manager.active_count(), 1);

    let id = manager
        .get_notification_at(0)
        .expect("notification should exist")
        .id;
    manager.remove_notification(id);

    // Transient notifications don't go to history
    assert_eq!(manager.history().len(), 0);
}

#[test]
fn test_manager_load_app_filters() {
    let mut manager = NotificationManager::new();

    let mut filters = HashMap::new();
    filters.insert("app1".to_string(), true);
    filters.insert("app2".to_string(), false);
    filters.insert("app3".to_string(), true);

    manager.load_app_filters(filters);

    let retrieved_filters = manager.app_filters();
    assert_eq!(retrieved_filters.len(), 3);
    assert_eq!(retrieved_filters.get("app1"), Some(&true));
    assert_eq!(retrieved_filters.get("app2"), Some(&false));
}

#[test]
fn test_manager_history_integration() {
    // Test with history enabled
    let manager = NotificationManager::with_history(100, Some(7));

    assert_eq!(manager.active_count(), 0);
    // History might be loaded from disk, so just verify it's initialized
    assert!(manager.history().len() <= 100);
}

#[test]
fn test_manager_cleanup_history() {
    let mut manager = NotificationManager::new();

    // Add many notifications - first 10 stay active, rest (140) get evicted to history
    // But history is limited to MAX_HISTORY_SIZE (100), so only last 100 are kept
    for i in 0..150 {
        let notif = create_notification("test", &format!("Notification {}", i), Urgency::Normal);
        manager.add_notification(notif);
    }

    // Should have 10 active, 100 in history (capped at MAX_HISTORY_SIZE)
    assert_eq!(manager.active_count(), 10);
    assert_eq!(manager.history().len(), 100);

    // Cleanup with max 100 items - should remove 0 (already at limit)
    let removed = manager.cleanup_history(100, None);

    assert_eq!(removed, 0);
    assert_eq!(manager.history().len(), 100);
}

#[test]
fn test_manager_resident_notifications_dont_expire() {
    let mut manager = NotificationManager::new();

    let mut notif = create_notification("test", "Resident", Urgency::Normal);
    notif.hints.resident = true;
    notif.expire_timeout = 1000;
    notif.timestamp = chrono::Local::now() - chrono::Duration::seconds(10);

    manager.add_notification(notif);

    let expired = manager.get_expired_notifications();
    assert_eq!(expired.len(), 0);
}

#[test]
fn test_manager_concurrent_filter_changes() {
    let mut manager = NotificationManager::new();

    // Set initial filters
    manager.set_app_filter("app1".to_string(), false);
    manager.set_min_urgency_level(1);
    manager.set_do_not_disturb(true);

    // Add notification - should be blocked by multiple filters
    let notif = create_notification("app1", "Test", Urgency::Low);
    let action = manager.add_notification(notif);

    assert_eq!(action, NotificationAction::AddedToHistoryOnly);

    // Change filters
    manager.set_do_not_disturb(false);
    manager.set_min_urgency_level(0);

    // Same notification type should still be blocked by app filter
    let notif2 = create_notification("app1", "Test 2", Urgency::Low);
    let action2 = manager.add_notification(notif2);

    assert_eq!(action2, NotificationAction::AddedToHistoryOnly);
}
