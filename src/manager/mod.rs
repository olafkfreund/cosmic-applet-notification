// Notification manager module
//
// This module manages notification lifecycle, state, and history.
// It provides a simple, single-threaded state manager that integrates
// with the iced application without requiring Arc/Mutex.

use std::collections::{HashMap, VecDeque};

use chrono::{DateTime, Duration, Local};

use crate::dbus::{Notification, Urgency};

/// Maximum number of notifications to keep in history.
///
/// Chosen based on typical session notification volume (~100 notifications/day)
/// and memory constraints (~10KB per notification = ~1MB total).
/// When exceeded, oldest notifications are removed (FIFO).
const MAX_HISTORY_SIZE: usize = 100;

/// Maximum number of active notifications to display simultaneously.
///
/// Prevents UI overflow and maintains readability. When exceeded,
/// oldest notifications are moved to history automatically.
/// Value based on typical screen height accommodating 10 notification cards.
const MAX_ACTIVE_NOTIFICATIONS: usize = 10;

/// Default notification timeout in seconds for notifications with expire_timeout=0.
///
/// Matches freedesktop.org notification spec default behavior.
/// Users expect brief, non-critical notifications to auto-dismiss after 5 seconds.
const DEFAULT_TIMEOUT_SECONDS: i64 = 5;

/// Notification manager state
///
/// Manages active notifications, history, and filtering.
/// All operations are synchronous - no threading complexity.
#[derive(Debug, Clone)]
pub struct NotificationManager {
    /// Active (visible) notifications
    active_notifications: VecDeque<Notification>,

    /// Historical notifications (circular buffer)
    notification_history: VecDeque<Notification>,

    /// Next notification ID counter
    next_id: u32,

    /// Do Not Disturb mode
    do_not_disturb: bool,

    /// Application filters (app_name -> should_show)
    app_filters: HashMap<String, bool>,
}

impl Default for NotificationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl NotificationManager {
    /// Create a new notification manager
    pub fn new() -> Self {
        Self {
            active_notifications: VecDeque::new(),
            notification_history: VecDeque::with_capacity(MAX_HISTORY_SIZE),
            next_id: 1,
            do_not_disturb: false,
            app_filters: HashMap::new(),
        }
    }

    /// Add a new notification
    ///
    /// Handles:
    /// - ID assignment
    /// - Notification replacement (replaces_id)
    /// - Filtering (DND, app filters, urgency)
    /// - History management
    /// - Active notification limits
    pub fn add_notification(&mut self, mut notification: Notification) -> NotificationAction {
        // Assign unique ID if not already assigned
        if notification.id == 0 {
            notification.id = self.next_id;
            self.next_id = self.next_id.wrapping_add(1);
        }

        // Check if this replaces an existing notification
        if notification.replaces_id != 0 {
            self.remove_notification(notification.replaces_id);
        }

        // Apply filters
        if !self.should_display(&notification) {
            // Add to history only, don't show
            self.add_to_history(notification);
            return NotificationAction::AddedToHistoryOnly;
        }

        // Add to history first
        self.add_to_history(notification.clone());

        // Add to active notifications
        self.active_notifications.push_back(notification);

        // Enforce maximum active notifications (FIFO)
        // Evicted notifications are already in history, don't add again
        while self.active_notifications.len() > MAX_ACTIVE_NOTIFICATIONS {
            self.active_notifications.pop_front();
        }

        NotificationAction::Displayed
    }

    /// Remove a notification by ID
    ///
    /// Removes from active notifications and adds to history if not already there.
    pub fn remove_notification(&mut self, id: u32) -> bool {
        if let Some(pos) = self.active_notifications.iter().position(|n| n.id == id) {
            let notification = self.active_notifications.remove(pos);
            // Add to history if transient flag not set
            if !notification.is_transient() {
                self.add_to_history(notification);
            }
            true
        } else {
            false
        }
    }

    /// Clear all active notifications
    pub fn clear_all(&mut self) {
        // Move all active to history (unless transient)
        for notification in self.active_notifications.drain(..) {
            if !notification.is_transient() {
                self.add_to_history(notification);
            }
        }
    }

    /// Clear notification history
    pub fn clear_history(&mut self) {
        self.notification_history.clear();
    }

    /// Get active notifications
    pub fn active_notifications(&self) -> &[Notification] {
        &self.active_notifications
    }

    /// Get number of active notifications
    pub fn active_count(&self) -> usize {
        self.active_notifications.len()
    }

    /// Get notification by ID
    pub fn get_notification(&self, id: u32) -> Option<&Notification> {
        self.active_notifications.iter().find(|n| n.id == id)
    }

    /// Get notification history
    pub fn history(&self) -> &VecDeque<Notification> {
        &self.notification_history
    }

    /// Get notifications that should expire
    ///
    /// Returns list of notification IDs that have exceeded their timeout.
    /// Call this periodically (e.g., on Tick message) to enforce timeouts.
    pub fn get_expired_notifications(&self) -> Vec<u32> {
        let now = Local::now();
        self.active_notifications
            .iter()
            .filter(|n| self.is_expired(n, now))
            .map(|n| n.id)
            .collect()
    }

    /// Check if a notification has expired
    fn is_expired(&self, notification: &Notification, now: DateTime<Local>) -> bool {
        // Validate timeout value and handle special cases
        match notification.expire_timeout {
            // -1 means never expire
            -1 => return false,
            // Invalid negative values (treat as never expire)
            t if t < 0 => {
                tracing::warn!(
                    "Invalid expire_timeout {} for notification {}, treating as never expire",
                    t,
                    notification.id
                );
                return false;
            }
            _ => {}
        }

        // Resident notifications don't auto-expire
        if notification.is_resident() {
            return false;
        }

        let timeout_seconds = if notification.expire_timeout == 0 {
            DEFAULT_TIMEOUT_SECONDS
        } else {
            notification.expire_timeout as i64 / 1000 // Convert ms to seconds
        };

        let age = now.signed_duration_since(notification.timestamp);
        age > Duration::seconds(timeout_seconds)
    }

    /// Set Do Not Disturb mode
    pub fn set_do_not_disturb(&mut self, enabled: bool) {
        self.do_not_disturb = enabled;
    }

    /// Check if Do Not Disturb is enabled
    pub fn is_do_not_disturb(&self) -> bool {
        self.do_not_disturb
    }

    /// Set application filter
    ///
    /// If should_show is false, notifications from this app will only go to history.
    pub fn set_app_filter(&mut self, app_name: String, should_show: bool) {
        self.app_filters.insert(app_name, should_show);
    }

    /// Remove application filter
    pub fn remove_app_filter(&mut self, app_name: &str) {
        self.app_filters.remove(app_name);
    }

    /// Get all application filters
    pub fn app_filters(&self) -> &HashMap<String, bool> {
        &self.app_filters
    }

    /// Check if a notification should be displayed
    ///
    /// Applies filtering logic:
    /// - Critical urgency always shows (even in DND)
    /// - DND mode hides low/normal urgency
    /// - App-specific filters
    fn should_display(&self, notification: &Notification) -> bool {
        // Critical notifications always show
        if notification.urgency() == Urgency::Critical {
            return true;
        }

        // Check Do Not Disturb mode
        if self.do_not_disturb {
            return false;
        }

        // Check app-specific filter
        if let Some(&should_show) = self.app_filters.get(&notification.app_name) {
            return should_show;
        }

        // Default: show the notification
        true
    }

    /// Add notification to history
    ///
    /// Maintains circular buffer with MAX_HISTORY_SIZE limit.
    fn add_to_history(&mut self, notification: Notification) {
        // Don't add transient notifications to history
        if notification.is_transient() {
            return;
        }

        self.notification_history.push_back(notification);

        // Maintain size limit (FIFO)
        while self.notification_history.len() > MAX_HISTORY_SIZE {
            self.notification_history.pop_front();
        }
    }

    /// Get notifications grouped by application
    ///
    /// Returns a map of app_name -> list of notifications.
    /// Useful for displaying grouped notifications in UI.
    pub fn get_notifications_by_app(&self) -> HashMap<String, Vec<&Notification>> {
        let mut grouped: HashMap<String, Vec<&Notification>> = HashMap::new();

        for notification in &self.active_notifications {
            grouped
                .entry(notification.app_name.clone())
                .or_default()
                .push(notification);
        }

        grouped
    }

    /// Get notifications filtered by urgency
    pub fn get_by_urgency(&self, urgency: Urgency) -> Vec<&Notification> {
        self.active_notifications
            .iter()
            .filter(|n| n.urgency() == urgency)
            .collect()
    }
}

/// Action taken when adding a notification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationAction {
    /// Notification was displayed to the user
    Displayed,
    /// Notification was added to history only (filtered)
    AddedToHistoryOnly,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dbus::{NotificationHints, Urgency};

    fn create_test_notification(app_name: &str, summary: &str) -> Notification {
        Notification {
            id: 0,
            app_name: app_name.to_string(),
            replaces_id: 0,
            app_icon: String::new(),
            summary: summary.to_string(),
            body: String::new(),
            actions: Vec::new(),
            hints: NotificationHints {
                urgency: Urgency::Normal,
                ..Default::default()
            },
            raw_hints: HashMap::new(),
            expire_timeout: 0,
            timestamp: Local::now(),
        }
    }

    #[test]
    fn test_new_manager() {
        let manager = NotificationManager::new();
        assert_eq!(manager.active_count(), 0);
        assert_eq!(manager.history().len(), 0);
        assert!(!manager.is_do_not_disturb());
    }

    #[test]
    fn test_add_notification() {
        let mut manager = NotificationManager::new();
        let notification = create_test_notification("test", "Test notification");

        let action = manager.add_notification(notification);
        assert_eq!(action, NotificationAction::Displayed);
        assert_eq!(manager.active_count(), 1);
        assert_eq!(manager.history().len(), 1);
    }

    #[test]
    fn test_remove_notification() {
        let mut manager = NotificationManager::new();
        let notification = create_test_notification("test", "Test");

        manager.add_notification(notification);
        let id = manager.active_notifications()[0].id;

        assert!(manager.remove_notification(id));
        assert_eq!(manager.active_count(), 0);
    }

    #[test]
    fn test_notification_replacement() {
        let mut manager = NotificationManager::new();

        let mut notif1 = create_test_notification("test", "Original");
        manager.add_notification(notif1.clone());

        let id1 = manager.active_notifications()[0].id;

        // Create replacement notification
        let mut notif2 = create_test_notification("test", "Replacement");
        notif2.replaces_id = id1;

        manager.add_notification(notif2);

        // Should only have one notification (replacement)
        assert_eq!(manager.active_count(), 1);
        assert_eq!(manager.active_notifications()[0].summary, "Replacement");
    }

    #[test]
    fn test_max_active_notifications() {
        let mut manager = NotificationManager::new();

        // Add more than MAX_ACTIVE_NOTIFICATIONS
        for i in 0..MAX_ACTIVE_NOTIFICATIONS + 5 {
            let notif = create_test_notification("test", &format!("Notification {}", i));
            manager.add_notification(notif);
        }

        // Should not exceed maximum
        assert_eq!(manager.active_count(), MAX_ACTIVE_NOTIFICATIONS);
    }

    #[test]
    fn test_history_limit() {
        let mut manager = NotificationManager::new();

        // Add more than MAX_HISTORY_SIZE
        for i in 0..MAX_HISTORY_SIZE + 10 {
            let notif = create_test_notification("test", &format!("Notification {}", i));
            manager.add_notification(notif);
        }

        // History should not exceed maximum
        assert_eq!(manager.history().len(), MAX_HISTORY_SIZE);
    }

    #[test]
    fn test_do_not_disturb_normal() {
        let mut manager = NotificationManager::new();
        manager.set_do_not_disturb(true);

        let notification = create_test_notification("test", "Test");
        let action = manager.add_notification(notification);

        assert_eq!(action, NotificationAction::AddedToHistoryOnly);
        assert_eq!(manager.active_count(), 0);
        assert_eq!(manager.history().len(), 1);
    }

    #[test]
    fn test_do_not_disturb_critical() {
        let mut manager = NotificationManager::new();
        manager.set_do_not_disturb(true);

        let mut notification = create_test_notification("test", "Critical");
        notification.hints.urgency = Urgency::Critical;

        let action = manager.add_notification(notification);

        // Critical notifications bypass DND
        assert_eq!(action, NotificationAction::Displayed);
        assert_eq!(manager.active_count(), 1);
    }

    #[test]
    fn test_app_filter() {
        let mut manager = NotificationManager::new();
        manager.set_app_filter("blocked_app".to_string(), false);

        let notification = create_test_notification("blocked_app", "Test");
        let action = manager.add_notification(notification);

        assert_eq!(action, NotificationAction::AddedToHistoryOnly);
        assert_eq!(manager.active_count(), 0);
    }

    #[test]
    fn test_clear_all() {
        let mut manager = NotificationManager::new();

        for i in 0..5 {
            let notif = create_test_notification("test", &format!("Notification {}", i));
            manager.add_notification(notif);
        }

        assert_eq!(manager.active_count(), 5);

        manager.clear_all();
        assert_eq!(manager.active_count(), 0);
        // All should be in history now
        assert_eq!(manager.history().len(), 5);
    }

    #[test]
    fn test_get_notifications_by_app() {
        let mut manager = NotificationManager::new();

        manager.add_notification(create_test_notification("app1", "Test 1"));
        manager.add_notification(create_test_notification("app1", "Test 2"));
        manager.add_notification(create_test_notification("app2", "Test 3"));

        let grouped = manager.get_notifications_by_app();

        assert_eq!(grouped.len(), 2);
        assert_eq!(grouped.get("app1").unwrap().len(), 2);
        assert_eq!(grouped.get("app2").unwrap().len(), 1);
    }

    #[test]
    fn test_get_by_urgency() {
        let mut manager = NotificationManager::new();

        let mut notif_critical = create_test_notification("test", "Critical");
        notif_critical.hints.urgency = Urgency::Critical;

        manager.add_notification(notif_critical);
        manager.add_notification(create_test_notification("test", "Normal"));

        let critical = manager.get_by_urgency(Urgency::Critical);
        assert_eq!(critical.len(), 1);

        let normal = manager.get_by_urgency(Urgency::Normal);
        assert_eq!(normal.len(), 1);
    }

    #[test]
    fn test_transient_not_in_history() {
        let mut manager = NotificationManager::new();

        let mut notification = create_test_notification("test", "Transient");
        notification.hints.transient = true;

        manager.add_notification(notification);

        // Should be active but not in history
        assert_eq!(manager.active_count(), 1);

        let id = manager.active_notifications()[0].id;
        manager.remove_notification(id);

        // Still should not be in history after removal
        assert_eq!(manager.history().len(), 0);
    }

    #[test]
    fn test_expired_notifications() {
        let mut manager = NotificationManager::new();

        // Create notification that expired 10 seconds ago
        let mut old_notification = create_test_notification("test", "Old");
        old_notification.expire_timeout = 5000; // 5 seconds
        old_notification.timestamp = Local::now() - Duration::seconds(10);

        // Create recent notification
        let recent_notification = create_test_notification("test", "Recent");

        manager.add_notification(old_notification);
        manager.add_notification(recent_notification);

        let expired = manager.get_expired_notifications();

        // Only the old notification should be expired
        assert_eq!(expired.len(), 1);
    }

    #[test]
    fn test_never_expire_timeout() {
        let mut manager = NotificationManager::new();

        let mut notification = create_test_notification("test", "Never Expire");
        notification.expire_timeout = -1;
        notification.timestamp = Local::now() - Duration::seconds(1000);

        manager.add_notification(notification);

        let expired = manager.get_expired_notifications();

        // Should not expire even though very old
        assert_eq!(expired.len(), 0);
    }

    #[test]
    fn test_resident_not_expired() {
        let mut manager = NotificationManager::new();

        let mut notification = create_test_notification("test", "Resident");
        notification.hints.resident = true;
        notification.expire_timeout = 1000; // 1 second
        notification.timestamp = Local::now() - Duration::seconds(10);

        manager.add_notification(notification);

        let expired = manager.get_expired_notifications();

        // Resident notifications don't auto-expire
        assert_eq!(expired.len(), 0);
    }

    #[test]
    fn test_no_duplicate_history() {
        let mut manager = NotificationManager::new();

        let notification = create_test_notification("test", "Test");
        manager.add_notification(notification);

        // History should only have one entry
        assert_eq!(manager.history().len(), 1);
        assert_eq!(manager.active_count(), 1);
    }

    #[test]
    fn test_evicted_notifications_in_history() {
        let mut manager = NotificationManager::new();

        // Add more than MAX_ACTIVE_NOTIFICATIONS
        for i in 0..MAX_ACTIVE_NOTIFICATIONS + 2 {
            let notif = create_test_notification("test", &format!("Notification {}", i));
            manager.add_notification(notif);
        }

        // Should have exactly MAX_ACTIVE_NOTIFICATIONS active
        assert_eq!(manager.active_count(), MAX_ACTIVE_NOTIFICATIONS);

        // All notifications should be in history (including evicted ones)
        assert_eq!(manager.history().len(), MAX_ACTIVE_NOTIFICATIONS + 2);
    }
}
