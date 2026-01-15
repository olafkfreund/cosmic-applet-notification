// Integration tests for configuration persistence
//
// Tests configuration loading, saving, validation, and sanitization.

use cosmic_applet_notifications::config::{AppletConfig, ConfigHelper};
use std::collections::HashMap;
use tempfile::TempDir;

fn create_test_config() -> AppletConfig {
    AppletConfig {
        version: 1,
        max_visible_notifications: 10,
        show_timestamp: true,
        show_app_icon: true,
        popup_width: 400,
        popup_height: 600,
        do_not_disturb: false,
        default_timeout: Some(5000),
        play_sound: false,
        show_preview: true,
        history_enabled: true,
        max_history_items: 100,
        history_retention_days: Some(7),
        app_filters: HashMap::new(),
        min_urgency_level: 0,
    }
}

#[test]
fn test_config_default_values() {
    let config = AppletConfig::default();

    assert_eq!(config.version, 1);
    assert_eq!(config.max_visible_notifications, 10);
    assert!(config.show_timestamp);
    assert!(config.show_app_icon);
    assert_eq!(config.popup_width, 400);
    assert_eq!(config.popup_height, 600);
    assert!(!config.do_not_disturb);
    assert_eq!(config.default_timeout, None);
    assert!(!config.play_sound);
    assert!(config.show_preview);
    assert!(config.history_enabled);
    assert_eq!(config.max_history_items, 100);
    assert_eq!(config.history_retention_days, None);
    assert_eq!(config.min_urgency_level, 0);
}

#[test]
fn test_config_validation_valid() {
    let config = create_test_config();
    assert!(config.validate());
}

#[test]
fn test_config_validation_invalid_max_visible() {
    let mut config = create_test_config();
    config.max_visible_notifications = 0;
    assert!(!config.validate());

    config.max_visible_notifications = 100;
    assert!(!config.validate());
}

#[test]
fn test_config_validation_invalid_popup_size() {
    let mut config = create_test_config();
    config.popup_width = 100;
    assert!(!config.validate());

    config.popup_width = 400;
    config.popup_height = 100;
    assert!(!config.validate());
}

#[test]
fn test_config_validation_invalid_history() {
    let mut config = create_test_config();
    config.max_history_items = 5;
    assert!(!config.validate());

    config.max_history_items = 2000;
    assert!(!config.validate());
}

#[test]
fn test_config_sanitization() {
    let mut config = create_test_config();

    // Set invalid values
    config.max_visible_notifications = 0;
    config.popup_width = 100;
    config.popup_height = 3000;
    config.max_history_items = 5000;
    config.default_timeout = Some(1000000);
    config.history_retention_days = Some(500);
    config.min_urgency_level = 10;

    config.sanitize();

    // Check all values are clamped to valid ranges
    assert!(config.max_visible_notifications >= 1);
    assert!(config.max_visible_notifications <= 50);
    assert!(config.popup_width >= 200);
    assert!(config.popup_width <= 2000);
    assert!(config.popup_height >= 200);
    assert!(config.popup_height <= 2000);
    assert!(config.max_history_items >= 10);
    assert!(config.max_history_items <= 1000);
    assert!(config.default_timeout.unwrap() <= 300000);
    assert!(config.history_retention_days.unwrap() <= 365);
    assert!(config.min_urgency_level <= 2);
}

#[test]
fn test_config_with_app_filters() {
    let mut config = create_test_config();

    config.app_filters.insert("firefox".to_string(), false);
    config.app_filters.insert("thunderbird".to_string(), true);

    assert!(config.validate());
    assert_eq!(config.app_filters.len(), 2);
    assert_eq!(config.app_filters.get("firefox"), Some(&false));
    assert_eq!(config.app_filters.get("thunderbird"), Some(&true));
}

#[test]
fn test_config_urgency_levels() {
    let mut config = create_test_config();

    // Test all valid urgency levels
    config.min_urgency_level = 0;
    assert!(config.validate());

    config.min_urgency_level = 1;
    assert!(config.validate());

    config.min_urgency_level = 2;
    assert!(config.validate());

    // Invalid urgency level
    config.min_urgency_level = 3;
    assert!(!config.validate());
}

#[test]
fn test_config_timeout_values() {
    let mut config = create_test_config();

    // Valid timeout
    config.default_timeout = Some(5000);
    assert!(config.validate());

    // No timeout (use notification's own timeout)
    config.default_timeout = None;
    assert!(config.validate());

    // Maximum timeout
    config.default_timeout = Some(300000);
    assert!(config.validate());

    // Exceeds maximum
    config.default_timeout = Some(300001);
    assert!(!config.validate());
}

#[test]
fn test_config_history_retention() {
    let mut config = create_test_config();

    // No retention limit
    config.history_retention_days = None;
    assert!(config.validate());

    // Valid retention
    config.history_retention_days = Some(30);
    assert!(config.validate());

    // Maximum retention
    config.history_retention_days = Some(365);
    assert!(config.validate());

    // Exceeds maximum
    config.history_retention_days = Some(366);
    assert!(!config.validate());
}

#[test]
fn test_config_clone() {
    let config = create_test_config();
    let cloned = config.clone();

    assert_eq!(config.version, cloned.version);
    assert_eq!(
        config.max_visible_notifications,
        cloned.max_visible_notifications
    );
    assert_eq!(config.popup_width, cloned.popup_width);
    assert_eq!(config.popup_height, cloned.popup_height);
}

#[test]
fn test_config_equality() {
    let config1 = create_test_config();
    let config2 = create_test_config();

    assert_eq!(config1, config2);
}

#[test]
fn test_config_inequality_after_modification() {
    let mut config1 = create_test_config();
    let config2 = create_test_config();

    config1.do_not_disturb = true;
    assert_ne!(config1, config2);
}

#[test]
fn test_config_large_app_filter_list() {
    let mut config = create_test_config();

    // Add many app filters
    for i in 0..100 {
        config
            .app_filters
            .insert(format!("app{}", i), i % 2 == 0);
    }

    assert_eq!(config.app_filters.len(), 100);
    assert!(config.validate());
}

#[test]
fn test_config_sanitize_preserves_valid_values() {
    let mut config = create_test_config();
    let original = config.clone();

    config.sanitize();

    assert_eq!(config, original);
}
