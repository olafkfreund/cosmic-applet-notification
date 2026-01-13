// Configuration module
//
// This module handles loading and saving applet configuration.

use serde::{Deserialize, Serialize};

/// Applet configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppletConfig {
    /// Maximum number of visible notifications
    pub max_visible_notifications: usize,

    /// Show notification timestamps
    pub show_timestamp: bool,

    /// Show application icons
    pub show_app_icon: bool,

    /// Do Not Disturb mode enabled
    pub do_not_disturb: bool,

    /// Default notification timeout (milliseconds, None = use notification's timeout)
    pub default_timeout: Option<u32>,

    /// Maximum history size
    pub max_history_items: usize,
}

impl Default for AppletConfig {
    fn default() -> Self {
        Self {
            max_visible_notifications: 5,
            show_timestamp: true,
            show_app_icon: true,
            do_not_disturb: false,
            default_timeout: None,
            max_history_items: 100,
        }
    }
}

// TODO: Implement configuration loading/saving
// TODO: Use cosmic-config when available
// TODO: Implement configuration watching for live updates
