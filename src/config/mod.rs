// Configuration module
//
// This module handles loading and saving applet configuration using cosmic-config.
//
// Configuration is stored at: ~/.config/cosmic/com.system76.CosmicAppletNotifications/v1/

pub mod helper;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Re-export helper
pub use helper::ConfigHelper;

/// Configuration version for migration support
pub const CONFIG_VERSION: u64 = 1;

/// Popup positioning mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PositionMode {
    /// Auto-position based on panel location (current default)
    Auto,
    /// Panel-relative positioning with custom anchor and offsets
    PanelRelative,
}

impl Default for PositionMode {
    fn default() -> Self {
        Self::Auto
    }
}

/// Panel anchor point for popup positioning
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PanelAnchor {
    /// Start of panel (left for horizontal, top for vertical)
    Start,
    /// Center of panel
    Center,
    /// End of panel (right for horizontal, bottom for vertical)
    End,
    /// At applet icon location (current behavior)
    AppletIcon,
}

impl Default for PanelAnchor {
    fn default() -> Self {
        Self::AppletIcon
    }
}

/// Popup positioning configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PopupPosition {
    /// Positioning mode
    #[serde(default)]
    pub mode: PositionMode,

    /// Anchor point on panel (for panel-relative mode)
    #[serde(default)]
    pub anchor: PanelAnchor,

    /// X offset from anchor (pixels, panel-relative)
    #[serde(default)]
    pub offset_x: i32,

    /// Y offset from anchor (pixels, panel-relative)
    #[serde(default)]
    pub offset_y: i32,

    /// Snap to edge when within threshold
    #[serde(default = "default_true")]
    pub snap_to_edge: bool,

    /// Snap threshold distance (pixels)
    #[serde(default = "default_snap_threshold")]
    pub snap_threshold: u32,
}

impl Default for PopupPosition {
    fn default() -> Self {
        Self {
            mode: PositionMode::Auto,
            anchor: PanelAnchor::AppletIcon,
            offset_x: 0,
            offset_y: 0,
            snap_to_edge: true,
            snap_threshold: 20,
        }
    }
}

/// Animation configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnimationConfig {
    /// Enable animations globally
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Enable notification appearance animations
    #[serde(default = "default_true")]
    pub notification_appear: bool,

    /// Enable notification dismissal animations
    #[serde(default = "default_true")]
    pub notification_dismiss: bool,

    /// Enable popup open/close animations
    #[serde(default = "default_true")]
    pub popup_transitions: bool,

    /// Enable hover effects on notification cards
    #[serde(default = "default_true")]
    pub hover_effects: bool,

    /// Show progress indicators for timed notifications
    #[serde(default = "default_true")]
    pub show_progress: bool,

    /// Animation duration multiplier (0.5 = half speed, 2.0 = double speed)
    /// Range: 0.1 to 3.0
    #[serde(default = "default_animation_speed")]
    pub speed_multiplier: f32,

    /// Respect system accessibility settings (prefers-reduced-motion)
    #[serde(default = "default_true")]
    pub respect_accessibility: bool,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            notification_appear: true,
            notification_dismiss: true,
            popup_transitions: true,
            hover_effects: true,
            show_progress: true,
            speed_multiplier: 1.0,
            respect_accessibility: true,
        }
    }
}

/// Applet configuration
///
/// All settings are persisted using cosmic-config.
/// Changes are automatically saved and trigger UI updates.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppletConfig {
    /// Configuration version for migration
    #[serde(default = "default_version")]
    pub version: u64,

    // Display Settings
    /// Maximum number of visible notifications in popup
    #[serde(default = "default_max_visible")]
    pub max_visible_notifications: usize,

    /// Show notification timestamps
    #[serde(default = "default_true")]
    pub show_timestamp: bool,

    /// Show application icons
    #[serde(default = "default_true")]
    pub show_app_icon: bool,

    /// Popup window width (pixels)
    #[serde(default = "default_popup_width")]
    pub popup_width: u32,

    /// Popup window height (pixels)
    #[serde(default = "default_popup_height")]
    pub popup_height: u32,

    /// Popup positioning configuration
    #[serde(default)]
    pub popup_position: PopupPosition,

    // Behavior Settings
    /// Do Not Disturb mode enabled
    #[serde(default)]
    pub do_not_disturb: bool,

    /// Default notification timeout (milliseconds, None = use notification's timeout)
    #[serde(default)]
    pub default_timeout: Option<u32>,

    /// Play sound on notification (if supported)
    #[serde(default)]
    pub play_sound: bool,

    /// Show notification preview (brief on-screen display)
    #[serde(default = "default_true")]
    pub show_preview: bool,

    // History Settings
    /// Enable notification history
    #[serde(default = "default_true")]
    pub history_enabled: bool,

    /// Maximum number of items in history
    #[serde(default = "default_max_history")]
    pub max_history_items: usize,

    /// History retention period (days, None = indefinite)
    #[serde(default)]
    pub history_retention_days: Option<u32>,

    // Filtering Configuration
    /// Per-application filters (app_name -> blocked)
    /// true = blocked, false = allowed
    #[serde(default)]
    pub app_filters: HashMap<String, bool>,

    /// Minimum urgency level to display (0=Low, 1=Normal, 2=Critical)
    #[serde(default = "default_urgency_level")]
    pub min_urgency_level: u8,

    // Animation Settings
    /// Animation configuration
    #[serde(default)]
    pub animations: AnimationConfig,
}

impl Default for AppletConfig {
    fn default() -> Self {
        Self {
            version: CONFIG_VERSION,
            max_visible_notifications: default_max_visible(),
            show_timestamp: true,
            show_app_icon: true,
            popup_width: default_popup_width(),
            popup_height: default_popup_height(),
            popup_position: PopupPosition::default(),
            do_not_disturb: false,
            default_timeout: None,
            play_sound: false,
            show_preview: true,
            history_enabled: true,
            max_history_items: default_max_history(),
            history_retention_days: None,
            app_filters: HashMap::new(),
            min_urgency_level: 0, // Show all (Low, Normal, Critical)
            animations: AnimationConfig::default(),
        }
    }
}

impl AppletConfig {
    /// Validate configuration values
    ///
    /// Ensures all values are within acceptable ranges.
    /// Returns true if config is valid, false otherwise.
    pub fn validate(&self) -> bool {
        // Validate max_visible_notifications (1-50)
        if !(1..=50).contains(&self.max_visible_notifications) {
            tracing::warn!(
                "Invalid max_visible_notifications: {}, must be 1-50",
                self.max_visible_notifications
            );
            return false;
        }

        // Validate popup dimensions (200-2000 pixels)
        if !(200..=2000).contains(&self.popup_width) {
            tracing::warn!(
                "Invalid popup_width: {}, must be 200-2000",
                self.popup_width
            );
            return false;
        }

        if !(200..=2000).contains(&self.popup_height) {
            tracing::warn!(
                "Invalid popup_height: {}, must be 200-2000",
                self.popup_height
            );
            return false;
        }

        // Validate popup area and aspect ratio
        if self.popup_width * self.popup_height < 100_000 {
            tracing::warn!(
                "Popup area too small: {}x{}",
                self.popup_width,
                self.popup_height
            );
            return false;
        }

        // Validate max_history_items (10-1000)
        if !(10..=1000).contains(&self.max_history_items) {
            tracing::warn!(
                "Invalid max_history_items: {}, must be 10-1000",
                self.max_history_items
            );
            return false;
        }

        // Validate min_urgency_level (0-2)
        if !(0..=2).contains(&self.min_urgency_level) {
            tracing::warn!(
                "Invalid min_urgency_level: {}, must be 0-2",
                self.min_urgency_level
            );
            return false;
        }

        // Validate default_timeout (max 5 minutes = 300,000ms)
        if let Some(timeout) = self.default_timeout {
            if timeout > 300_000 {
                tracing::warn!("Invalid default_timeout: {}ms, must be ≤300000ms", timeout);
                return false;
            }
        }

        // Validate history_retention_days (max 365 days = 1 year)
        if let Some(days) = self.history_retention_days {
            if days > 365 {
                tracing::warn!("Invalid history_retention_days: {}, must be ≤365", days);
                return false;
            }
        }

        // Validate app_filters (max 1000 entries, max 256 chars per name)
        if self.app_filters.len() > 1000 {
            tracing::warn!("Too many app filters: {}", self.app_filters.len());
            return false;
        }

        for app_name in self.app_filters.keys() {
            if app_name.len() > 256 {
                tracing::warn!("App filter name too long: {} bytes", app_name.len());
                return false;
            }
        }

        // Validate popup position offsets (±3000 pixels)
        if !(-3000..=3000).contains(&self.popup_position.offset_x) {
            tracing::warn!(
                "Invalid popup offset_x: {}, must be -3000 to 3000",
                self.popup_position.offset_x
            );
            return false;
        }

        if !(-3000..=3000).contains(&self.popup_position.offset_y) {
            tracing::warn!(
                "Invalid popup offset_y: {}, must be -3000 to 3000",
                self.popup_position.offset_y
            );
            return false;
        }

        // Validate snap threshold (5-100 pixels)
        if !(5..=100).contains(&self.popup_position.snap_threshold) {
            tracing::warn!(
                "Invalid snap_threshold: {}, must be 5-100",
                self.popup_position.snap_threshold
            );
            return false;
        }

        // Validate animation speed multiplier (0.1-3.0)
        if !(0.1..=3.0).contains(&self.animations.speed_multiplier) {
            tracing::warn!(
                "Invalid animation speed_multiplier: {}, must be 0.1-3.0",
                self.animations.speed_multiplier
            );
            return false;
        }

        true
    }

    /// Sanitize configuration values
    ///
    /// Clamps all values to acceptable ranges.
    /// Use this to fix invalid configs loaded from disk.
    pub fn sanitize(&mut self) {
        self.max_visible_notifications = self.max_visible_notifications.clamp(1, 50);
        self.popup_width = self.popup_width.clamp(200, 2000);
        self.popup_height = self.popup_height.clamp(200, 2000);
        self.max_history_items = self.max_history_items.clamp(10, 1000);
        self.min_urgency_level = self.min_urgency_level.min(2);

        // Sanitize optional timeout (max 5 minutes)
        if let Some(timeout) = self.default_timeout {
            if timeout > 300_000 {
                self.default_timeout = Some(300_000);
            }
        }

        // Sanitize optional retention period (max 1 year)
        if let Some(days) = self.history_retention_days {
            if days > 365 {
                self.history_retention_days = Some(365);
            }
        }

        // Sanitize app filters (limit count and name length)
        if self.app_filters.len() > 1000 {
            // Keep only first 1000 entries (arbitrary, but deterministic)
            let keys_to_remove: Vec<_> = self.app_filters.keys().skip(1000).cloned().collect();
            for key in keys_to_remove {
                self.app_filters.remove(&key);
            }
        }

        // Truncate long app names
        let long_names: Vec<_> = self
            .app_filters
            .keys()
            .filter(|name| name.len() > 256)
            .cloned()
            .collect();

        for long_name in long_names {
            if let Some(value) = self.app_filters.remove(&long_name) {
                let truncated = long_name.chars().take(256).collect::<String>();
                self.app_filters.insert(truncated, value);
            }
        }

        // Sanitize popup position
        self.popup_position.offset_x = self.popup_position.offset_x.clamp(-3000, 3000);
        self.popup_position.offset_y = self.popup_position.offset_y.clamp(-3000, 3000);
        self.popup_position.snap_threshold = self.popup_position.snap_threshold.clamp(5, 100);

        // Sanitize animation speed
        self.animations.speed_multiplier = self.animations.speed_multiplier.clamp(0.1, 3.0);
    }

    /// Migrate configuration from older version
    ///
    /// Handles version upgrades and sets new fields to defaults.
    pub fn migrate(&mut self, from_version: u64) {
        if from_version < CONFIG_VERSION {
            tracing::info!(
                "Migrating config from v{} to v{}",
                from_version,
                CONFIG_VERSION
            );

            // Future migration logic goes here
            // For now, just update the version
            self.version = CONFIG_VERSION;
        }
    }
}

// Default value functions for serde
fn default_version() -> u64 {
    CONFIG_VERSION
}

fn default_max_visible() -> usize {
    10
}

fn default_true() -> bool {
    true
}

fn default_popup_width() -> u32 {
    400
}

fn default_popup_height() -> u32 {
    600
}

fn default_max_history() -> usize {
    100
}

fn default_urgency_level() -> u8 {
    0 // Show all urgency levels
}

fn default_snap_threshold() -> u32 {
    20 // pixels
}

fn default_animation_speed() -> f32 {
    1.0 // Normal speed (1x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_is_valid() {
        let config = AppletConfig::default();
        assert!(config.validate());
    }

    #[test]
    fn test_invalid_max_visible() {
        let mut config = AppletConfig::default();
        config.max_visible_notifications = 0;
        assert!(!config.validate());

        config.max_visible_notifications = 100;
        assert!(!config.validate());
    }

    #[test]
    fn test_invalid_popup_dimensions() {
        let mut config = AppletConfig::default();
        config.popup_width = 100;
        assert!(!config.validate());

        config.popup_width = 3000;
        assert!(!config.validate());
    }

    #[test]
    fn test_sanitize() {
        let mut config = AppletConfig::default();
        config.max_visible_notifications = 0;
        config.popup_width = 100;
        config.max_history_items = 5000;
        config.min_urgency_level = 10;
        config.default_timeout = Some(500_000); // Exceeds max
        config.history_retention_days = Some(500); // Exceeds max

        config.sanitize();

        assert_eq!(config.max_visible_notifications, 1);
        assert_eq!(config.popup_width, 200);
        assert_eq!(config.max_history_items, 1000);
        assert_eq!(config.min_urgency_level, 2);
        assert_eq!(config.default_timeout, Some(300_000));
        assert_eq!(config.history_retention_days, Some(365));
    }

    #[test]
    fn test_config_serialization() {
        let config = AppletConfig::default();
        let serialized = ron::to_string(&config).unwrap();
        let deserialized: AppletConfig = ron::from_str(&serialized).unwrap();
        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_app_filters() {
        let mut config = AppletConfig::default();
        config.app_filters.insert("spam_app".to_string(), true);
        config
            .app_filters
            .insert("important_app".to_string(), false);

        assert!(config.validate());
        assert_eq!(config.app_filters.len(), 2);
    }

    #[test]
    fn test_invalid_timeout() {
        let mut config = AppletConfig::default();
        config.default_timeout = Some(400_000); // Exceeds 300,000ms max
        assert!(!config.validate());

        config.default_timeout = Some(300_000); // Exactly at max
        assert!(config.validate());

        config.default_timeout = Some(100_000); // Well within bounds
        assert!(config.validate());

        config.default_timeout = None; // None is valid
        assert!(config.validate());
    }

    #[test]
    fn test_invalid_retention_days() {
        let mut config = AppletConfig::default();
        config.history_retention_days = Some(400); // Exceeds 365 days
        assert!(!config.validate());

        config.history_retention_days = Some(365); // Exactly at max
        assert!(config.validate());

        config.history_retention_days = Some(30); // Well within bounds
        assert!(config.validate());

        config.history_retention_days = None; // None is valid
        assert!(config.validate());
    }

    #[test]
    fn test_too_many_app_filters() {
        let mut config = AppletConfig::default();

        // Add 1001 filters (exceeds max of 1000)
        for i in 0..1001 {
            config.app_filters.insert(format!("app_{}", i), i % 2 == 0);
        }

        assert!(!config.validate());
        assert_eq!(config.app_filters.len(), 1001);

        // Sanitize should reduce to 1000
        config.sanitize();
        assert_eq!(config.app_filters.len(), 1000);
        assert!(config.validate());
    }

    #[test]
    fn test_app_filter_name_too_long() {
        let mut config = AppletConfig::default();

        // Create a name with 300 characters (exceeds 256 max)
        let long_name = "a".repeat(300);
        config.app_filters.insert(long_name.clone(), true);

        assert!(!config.validate());

        // Sanitize should truncate to 256 chars
        config.sanitize();
        assert!(config.validate());

        // Should have one entry with 256 char name
        assert_eq!(config.app_filters.len(), 1);
        let truncated_name = config.app_filters.keys().next().unwrap();
        assert_eq!(truncated_name.len(), 256);
    }

    #[test]
    fn test_popup_area_validation() {
        let mut config = AppletConfig::default();

        // Too small area (200 x 200 = 40,000 < 100,000)
        config.popup_width = 200;
        config.popup_height = 200;
        assert!(!config.validate());

        // Just above minimum area (316 x 317 = 100,172)
        config.popup_width = 316;
        config.popup_height = 317;
        assert!(config.validate());

        // Default dimensions should be valid
        config.popup_width = default_popup_width();
        config.popup_height = default_popup_height();
        assert!(config.validate());
    }
}
