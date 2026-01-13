// Notification manager module
//
// This module manages notification lifecycle, state, and history.

use crate::dbus::Notification;
use std::collections::VecDeque;

/// Maximum number of notifications to keep in history
const MAX_HISTORY_SIZE: usize = 100;

/// Notification manager state
pub struct NotificationManager {
    /// Active (visible) notifications
    active_notifications: Vec<Notification>,

    /// Historical notifications
    notification_history: VecDeque<Notification>,

    /// Next notification ID
    next_id: u32,
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
            active_notifications: Vec::new(),
            notification_history: VecDeque::with_capacity(MAX_HISTORY_SIZE),
            next_id: 1,
        }
    }

    /// Add a new notification
    pub fn add_notification(&mut self, mut notification: Notification) {
        // Assign ID if needed
        if notification.id == 0 {
            notification.id = self.next_id;
            self.next_id += 1;
        }

        // Add to active notifications
        self.active_notifications.push(notification.clone());

        // Add to history
        self.notification_history.push_back(notification);

        // Trim history if too large
        while self.notification_history.len() > MAX_HISTORY_SIZE {
            self.notification_history.pop_front();
        }
    }

    /// Remove a notification by ID
    pub fn remove_notification(&mut self, id: u32) {
        self.active_notifications.retain(|n| n.id != id);
    }

    /// Get active notifications
    pub fn active_notifications(&self) -> &[Notification] {
        &self.active_notifications
    }

    /// Get notification history
    pub fn history(&self) -> &VecDeque<Notification> {
        &self.notification_history
    }
}

// TODO: Implement filtering
// TODO: Implement grouping by application
// TODO: Implement timeout handling
