// Integration tests for configuration persistence
//
// Tests configuration loading, saving, validation, and sanitization.

use cosmic_applet_notifications::config::{
    AnimationConfig, AppletConfig, PanelAnchor, PopupPosition, PositionMode,
};
use std::collections::HashMap;

fn create_test_config() -> AppletConfig {
    AppletConfig {
        version: 1,
        max_visible_notifications: 10,
        show_timestamp: true,
        show_app_icon: true,
        popup_width: 400,
        popup_height: 600,
        popup_position: PopupPosition::default(),
        do_not_disturb: false,
        default_timeout: Some(5000),
        play_sound: false,
        show_preview: true,
        history_enabled: true,
        max_history_items: 100,
        history_retention_days: Some(7),
        app_filters: HashMap::new(),
        min_urgency_level: 0,
        animations: AnimationConfig::default(),
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
        config.app_filters.insert(format!("app{}", i), i % 2 == 0);
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

// ============================================================================
// Popup Position Configuration Tests
// ============================================================================

#[test]
fn test_popup_position_default_values() {
    let config = AppletConfig::default();
    let pos = &config.popup_position;

    // Check default values
    assert_eq!(pos.mode, PositionMode::Auto);
    assert_eq!(pos.anchor, PanelAnchor::AppletIcon);
    assert_eq!(pos.offset_x, 0);
    assert_eq!(pos.offset_y, 0);
    assert!(pos.snap_to_edge); // Default is true
    assert_eq!(pos.snap_threshold, 20);
}

#[test]
fn test_popup_position_validation_valid() {
    let mut config = create_test_config();

    // Test Auto mode (should always be valid)
    config.popup_position.mode = PositionMode::Auto;
    assert!(config.validate());

    // Test PanelRelative mode with valid values
    config.popup_position.mode = PositionMode::PanelRelative;
    config.popup_position.anchor = PanelAnchor::Start;
    config.popup_position.offset_x = 100;
    config.popup_position.offset_y = -50;
    config.popup_position.snap_to_edge = true;
    config.popup_position.snap_threshold = 50;
    assert!(config.validate());

    // Test all anchor points
    for anchor in [
        PanelAnchor::Start,
        PanelAnchor::Center,
        PanelAnchor::End,
        PanelAnchor::AppletIcon,
    ] {
        config.popup_position.anchor = anchor;
        assert!(config.validate());
    }
}

#[test]
fn test_popup_position_validation_invalid_offsets() {
    let mut config = create_test_config();
    config.popup_position.mode = PositionMode::PanelRelative;

    // Test offset_x too low
    config.popup_position.offset_x = -3001;
    assert!(!config.validate());

    // Test offset_x too high
    config.popup_position.offset_x = 3001;
    assert!(!config.validate());

    // Reset offset_x, test offset_y too low
    config.popup_position.offset_x = 0;
    config.popup_position.offset_y = -3001;
    assert!(!config.validate());

    // Test offset_y too high
    config.popup_position.offset_y = 3001;
    assert!(!config.validate());

    // Test edge cases (should be valid)
    config.popup_position.offset_x = -3000;
    config.popup_position.offset_y = 3000;
    assert!(config.validate());

    config.popup_position.offset_x = 3000;
    config.popup_position.offset_y = -3000;
    assert!(config.validate());
}

#[test]
fn test_popup_position_validation_invalid_snap_threshold() {
    let mut config = create_test_config();
    config.popup_position.snap_to_edge = true;

    // Test threshold too low
    config.popup_position.snap_threshold = 4;
    assert!(!config.validate());

    // Test threshold too high
    config.popup_position.snap_threshold = 101;
    assert!(!config.validate());

    // Test edge cases (should be valid)
    config.popup_position.snap_threshold = 5;
    assert!(config.validate());

    config.popup_position.snap_threshold = 100;
    assert!(config.validate());
}

#[test]
fn test_popup_position_sanitization() {
    let mut config = create_test_config();

    // Set invalid values
    config.popup_position.offset_x = 5000;
    config.popup_position.offset_y = -5000;
    config.popup_position.snap_threshold = 200;

    config.sanitize();

    // Check values are clamped to valid ranges
    assert!(config.popup_position.offset_x >= -3000);
    assert!(config.popup_position.offset_x <= 3000);
    assert!(config.popup_position.offset_y >= -3000);
    assert!(config.popup_position.offset_y <= 3000);
    assert!(config.popup_position.snap_threshold >= 5);
    assert!(config.popup_position.snap_threshold <= 100);

    // Verify specific clamped values
    assert_eq!(config.popup_position.offset_x, 3000);
    assert_eq!(config.popup_position.offset_y, -3000);
    assert_eq!(config.popup_position.snap_threshold, 100);
}

#[test]
fn test_popup_position_sanitization_preserves_valid() {
    let mut config = create_test_config();
    config.popup_position.offset_x = 250;
    config.popup_position.offset_y = -150;
    config.popup_position.snap_threshold = 30;

    let original_pos = config.popup_position.clone();

    config.sanitize();

    // Valid values should remain unchanged
    assert_eq!(config.popup_position.offset_x, original_pos.offset_x);
    assert_eq!(config.popup_position.offset_y, original_pos.offset_y);
    assert_eq!(
        config.popup_position.snap_threshold,
        original_pos.snap_threshold
    );
}

#[test]
fn test_popup_position_modes() {
    let mut config = create_test_config();

    // Test Auto mode
    config.popup_position.mode = PositionMode::Auto;
    assert!(config.validate());

    // Test PanelRelative mode
    config.popup_position.mode = PositionMode::PanelRelative;
    assert!(config.validate());
}

#[test]
fn test_popup_position_all_anchors() {
    let mut config = create_test_config();
    config.popup_position.mode = PositionMode::PanelRelative;

    // Test all anchor points are valid
    let anchors = [
        PanelAnchor::Start,
        PanelAnchor::Center,
        PanelAnchor::End,
        PanelAnchor::AppletIcon,
    ];

    for anchor in anchors {
        config.popup_position.anchor = anchor;
        assert!(config.validate(), "Anchor {:?} should be valid", anchor);
    }
}

#[test]
fn test_popup_position_clone() {
    let config = create_test_config();
    let cloned = config.clone();

    assert_eq!(config.popup_position.mode, cloned.popup_position.mode);
    assert_eq!(config.popup_position.anchor, cloned.popup_position.anchor);
    assert_eq!(
        config.popup_position.offset_x,
        cloned.popup_position.offset_x
    );
    assert_eq!(
        config.popup_position.offset_y,
        cloned.popup_position.offset_y
    );
    assert_eq!(
        config.popup_position.snap_to_edge,
        cloned.popup_position.snap_to_edge
    );
    assert_eq!(
        config.popup_position.snap_threshold,
        cloned.popup_position.snap_threshold
    );
}

#[test]
fn test_popup_position_equality() {
    let config1 = create_test_config();
    let config2 = create_test_config();

    assert_eq!(config1.popup_position, config2.popup_position);
}

#[test]
fn test_popup_position_inequality_after_modification() {
    let mut config1 = create_test_config();
    let config2 = create_test_config();

    // Modify mode
    config1.popup_position.mode = PositionMode::PanelRelative;
    assert_ne!(config1.popup_position, config2.popup_position);

    // Reset and modify anchor
    config1.popup_position.mode = config2.popup_position.mode;
    config1.popup_position.anchor = PanelAnchor::Start;
    assert_ne!(config1.popup_position, config2.popup_position);

    // Reset and modify offsets
    config1.popup_position.anchor = config2.popup_position.anchor;
    config1.popup_position.offset_x = 100;
    assert_ne!(config1.popup_position, config2.popup_position);
}

#[test]
fn test_popup_position_snap_behavior() {
    let mut config = create_test_config();

    // Snap disabled with valid threshold should be valid
    config.popup_position.snap_to_edge = false;
    config.popup_position.snap_threshold = 20; // Must still be valid range
    assert!(config.validate());

    // Snap disabled with invalid threshold should still fail
    // (threshold is always validated to ensure config integrity)
    config.popup_position.snap_threshold = 4;
    assert!(!config.validate());

    // Fix threshold and enable snap
    config.popup_position.snap_threshold = 20;
    config.popup_position.snap_to_edge = true;
    assert!(config.validate()); // Should pass
}

#[test]
fn test_popup_position_extreme_values() {
    let mut config = create_test_config();

    // Test maximum negative offsets
    config.popup_position.offset_x = -3000;
    config.popup_position.offset_y = -3000;
    assert!(config.validate());

    // Test maximum positive offsets
    config.popup_position.offset_x = 3000;
    config.popup_position.offset_y = 3000;
    assert!(config.validate());

    // Test minimum snap threshold
    config.popup_position.snap_to_edge = true;
    config.popup_position.snap_threshold = 5;
    assert!(config.validate());

    // Test maximum snap threshold
    config.popup_position.snap_threshold = 100;
    assert!(config.validate());
}

#[test]
fn test_config_with_custom_position() {
    let mut config = create_test_config();

    // Configure custom position
    config.popup_position.mode = PositionMode::PanelRelative;
    config.popup_position.anchor = PanelAnchor::End;
    config.popup_position.offset_x = -200;
    config.popup_position.offset_y = 50;
    config.popup_position.snap_to_edge = true;
    config.popup_position.snap_threshold = 15;

    // Should validate
    assert!(config.validate());

    // Clone should preserve all settings
    let cloned = config.clone();
    assert_eq!(config.popup_position, cloned.popup_position);
}
