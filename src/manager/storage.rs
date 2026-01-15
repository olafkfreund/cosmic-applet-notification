// History storage module
//
// Handles persistent storage of notification history to disk.
// Uses RON format for human-readable storage compatible with config system.

use std::collections::VecDeque;
use std::path::PathBuf;

use crate::dbus::Notification;

/// History storage helper
///
/// Manages persistence of notification history using file-based storage.
/// Storage location: ~/.config/cosmic/com.system76.CosmicAppletNotifications/v1/history.ron
pub struct HistoryStorage {
    storage_path: PathBuf,
}

impl HistoryStorage {
    /// Create a new history storage helper
    ///
    /// Determines storage path based on XDG base directory spec.
    /// Matches config directory structure for consistency.
    pub fn new() -> Self {
        let storage_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("cosmic")
            .join("com.system76.CosmicAppletNotifications")
            .join("v1");

        // Attempt to create directory hierarchy
        if let Err(e) = std::fs::create_dir_all(&storage_dir) {
            tracing::warn!(
                "Failed to create history storage directory {:?}: {}. History save will fail.",
                storage_dir,
                e
            );
        }

        let storage_path = storage_dir.join("history.ron");

        Self { storage_path }
    }

    /// Load notification history from disk
    ///
    /// Returns empty VecDeque if file doesn't exist or is corrupted.
    /// Automatically handles:
    /// - Missing file (returns empty history)
    /// - Corrupted data (logs warning, returns empty)
    /// - Permission errors (logs error, returns empty)
    pub fn load(&self) -> VecDeque<Notification> {
        match std::fs::read_to_string(&self.storage_path) {
            Ok(content) => match ron::from_str::<Vec<Notification>>(&content) {
                Ok(notifications) => {
                    tracing::info!(
                        "Loaded {} notifications from history: {:?}",
                        notifications.len(),
                        self.storage_path
                    );
                    notifications.into_iter().collect()
                }
                Err(e) => {
                    tracing::warn!(
                        "Failed to parse history file {:?}: {}. Starting with empty history.",
                        self.storage_path,
                        e
                    );
                    VecDeque::new()
                }
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                tracing::debug!(
                    "No history file found at {:?}. Starting with empty history.",
                    self.storage_path
                );
                VecDeque::new()
            }
            Err(e) => {
                tracing::error!(
                    "Failed to read history file {:?}: {}. Starting with empty history.",
                    self.storage_path,
                    e
                );
                VecDeque::new()
            }
        }
    }

    /// Save notification history to disk
    ///
    /// Serializes notifications to RON format.
    /// Sets restrictive file permissions (0600) for privacy.
    ///
    /// Returns:
    /// - Ok(()) on success
    /// - Err on I/O or serialization failure
    pub fn save(&self, history: &VecDeque<Notification>) -> Result<(), std::io::Error> {
        // Ensure parent directory exists
        if let Some(parent) = self.storage_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Convert VecDeque to Vec for serialization
        let history_vec: Vec<_> = history.iter().cloned().collect();

        // Serialize to RON format with pretty printing
        let serialized = ron::ser::to_string_pretty(&history_vec, Default::default())
            .map_err(|e| std::io::Error::other(e))?;

        // Write to file
        std::fs::write(&self.storage_path, serialized)?;

        // Set restrictive permissions (owner read/write only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&self.storage_path)?.permissions();
            perms.set_mode(0o600);
            std::fs::set_permissions(&self.storage_path, perms)?;
        }

        tracing::debug!(
            "Saved {} notifications to history: {:?}",
            history.len(),
            self.storage_path
        );

        Ok(())
    }

    /// Clean up old notifications based on retention policy
    ///
    /// Removes notifications older than the specified number of days.
    /// Returns the number of notifications removed.
    ///
    /// Parameters:
    /// - history: Mutable reference to notification history
    /// - retention_days: Maximum age in days (None = keep all)
    pub fn cleanup_old_notifications(
        history: &mut VecDeque<Notification>,
        retention_days: Option<u32>,
    ) -> usize {
        let Some(days) = retention_days else {
            // No retention policy, keep everything
            return 0;
        };

        let now = chrono::Local::now();
        let retention_duration = chrono::Duration::days(days as i64);
        let cutoff_time = now - retention_duration;

        let original_len = history.len();

        // Remove notifications older than cutoff
        history.retain(|notification| notification.timestamp >= cutoff_time);

        let removed = original_len - history.len();

        if removed > 0 {
            tracing::info!(
                "Cleaned up {} notifications older than {} days",
                removed,
                days
            );
        }

        removed
    }

    /// Enforce maximum history size
    ///
    /// Removes oldest notifications if history exceeds max_items.
    /// Returns the number of notifications removed.
    ///
    /// Parameters:
    /// - history: Mutable reference to notification history
    /// - max_items: Maximum number of items to keep
    pub fn enforce_size_limit(history: &mut VecDeque<Notification>, max_items: usize) -> usize {
        if history.len() <= max_items {
            return 0;
        }

        let to_remove = history.len() - max_items;

        // Remove oldest notifications (from front)
        for _ in 0..to_remove {
            history.pop_front();
        }

        tracing::debug!(
            "Enforced history size limit: removed {} old notifications",
            to_remove
        );

        to_remove
    }

    /// Get storage file path
    pub fn path(&self) -> &PathBuf {
        &self.storage_path
    }
}

impl Default for HistoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dbus::NotificationHints;
    use chrono::Local;
    use std::collections::HashMap;
    use tempfile::TempDir;

    fn create_test_notification(summary: &str, age_days: i64) -> Notification {
        Notification {
            id: 1,
            app_name: "test".to_string(),
            replaces_id: 0,
            app_icon: String::new(),
            summary: summary.to_string(),
            body: String::new(),
            actions: Vec::new(),
            hints: NotificationHints::default(),
            raw_hints: HashMap::new(),
            expire_timeout: 0,
            timestamp: Local::now() - chrono::Duration::days(age_days),
        }
    }

    fn create_test_storage() -> (HistoryStorage, TempDir) {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage_path = temp_dir.path().join("history.ron");

        let storage = HistoryStorage {
            storage_path: storage_path.clone(),
        };

        (storage, temp_dir)
    }

    #[test]
    fn test_load_nonexistent_returns_empty() {
        let (storage, _temp) = create_test_storage();

        let history = storage.load();
        assert_eq!(history.len(), 0);
    }

    #[test]
    fn test_save_and_load() {
        let (storage, _temp) = create_test_storage();

        let mut history = VecDeque::new();
        history.push_back(create_test_notification("Test 1", 0));
        history.push_back(create_test_notification("Test 2", 0));

        // Save
        storage.save(&history).unwrap();

        // Load
        let loaded = storage.load();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].summary, "Test 1");
        assert_eq!(loaded[1].summary, "Test 2");
    }

    #[test]
    fn test_load_corrupted_returns_empty() {
        let (storage, _temp) = create_test_storage();

        // Write invalid RON data
        std::fs::write(&storage.storage_path, "invalid ron content").unwrap();

        let history = storage.load();
        assert_eq!(history.len(), 0);
    }

    #[test]
    fn test_cleanup_old_notifications() {
        let mut history = VecDeque::new();

        // Add notifications of various ages
        history.push_back(create_test_notification("1 day old", 1));
        history.push_back(create_test_notification("10 days old", 10));
        history.push_back(create_test_notification("50 days old", 50));
        history.push_back(create_test_notification("100 days old", 100));

        // Clean up notifications older than 30 days
        let removed = HistoryStorage::cleanup_old_notifications(&mut history, Some(30));

        assert_eq!(removed, 2); // 50 and 100 day old removed
        assert_eq!(history.len(), 2); // 1 and 10 day old remain
    }

    #[test]
    fn test_cleanup_no_retention() {
        let mut history = VecDeque::new();
        history.push_back(create_test_notification("Old", 100));

        // No retention policy (None)
        let removed = HistoryStorage::cleanup_old_notifications(&mut history, None);

        assert_eq!(removed, 0);
        assert_eq!(history.len(), 1); // Nothing removed
    }

    #[test]
    fn test_enforce_size_limit() {
        let mut history = VecDeque::new();

        // Add 10 notifications
        for i in 0..10 {
            history.push_back(create_test_notification(&format!("Notification {}", i), 0));
        }

        // Enforce limit of 5
        let removed = HistoryStorage::enforce_size_limit(&mut history, 5);

        assert_eq!(removed, 5);
        assert_eq!(history.len(), 5);

        // Should keep the newest 5 (5-9)
        assert_eq!(history[0].summary, "Notification 5");
        assert_eq!(history[4].summary, "Notification 9");
    }

    #[test]
    fn test_enforce_size_limit_no_change() {
        let mut history = VecDeque::new();
        history.push_back(create_test_notification("Test", 0));

        // Limit is larger than current size
        let removed = HistoryStorage::enforce_size_limit(&mut history, 10);

        assert_eq!(removed, 0);
        assert_eq!(history.len(), 1);
    }

    #[test]
    fn test_save_creates_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let nested_path = temp_dir
            .path()
            .join("nested")
            .join("directory")
            .join("history.ron");

        let storage = HistoryStorage {
            storage_path: nested_path.clone(),
        };

        let mut history = VecDeque::new();
        history.push_back(create_test_notification("Test", 0));

        // Should create directory structure
        storage.save(&history).unwrap();

        assert!(nested_path.exists());
    }

    #[test]
    #[cfg(unix)]
    fn test_file_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let (storage, _temp) = create_test_storage();

        let mut history = VecDeque::new();
        history.push_back(create_test_notification("Test", 0));

        storage.save(&history).unwrap();

        // Check file permissions are 0600
        let metadata = std::fs::metadata(&storage.storage_path).unwrap();
        let permissions = metadata.permissions();
        assert_eq!(permissions.mode() & 0o777, 0o600);
    }
}
