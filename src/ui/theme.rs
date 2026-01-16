// COSMIC Design System Theme Module
//
// This module provides standardized design tokens and styling helpers
// following COSMIC desktop design patterns and best practices.

use cosmic::iced::Color;
use cosmic::theme;

/// Design tokens for consistent spacing throughout the UI
pub struct Spacing;

impl Spacing {
    /// Extra extra small spacing (4px) - tight grouping
    pub fn xxs() -> u16 {
        theme::active().cosmic().spacing.space_xxs
    }

    /// Extra small spacing (8px) - related items
    pub fn xs() -> u16 {
        theme::active().cosmic().spacing.space_xs
    }

    /// Small spacing (12px) - standard element spacing
    pub fn s() -> u16 {
        theme::active().cosmic().spacing.space_s
    }

    /// Medium spacing (16px) - section spacing
    pub fn m() -> u16 {
        theme::active().cosmic().spacing.space_m
    }

    /// Large spacing (24px) - major sections
    pub fn l() -> u16 {
        theme::active().cosmic().spacing.space_l
    }

    /// Extra large spacing (32px) - page sections
    pub fn xl() -> u16 {
        theme::active().cosmic().spacing.space_xl
    }
}

/// Design tokens for corner radius
pub struct Radius;

impl Radius {
    /// Small radius for subtle rounding
    pub fn s() -> [f32; 4] {
        theme::active().cosmic().corner_radii.radius_s
    }

    /// Medium radius for standard components (most common)
    pub fn m() -> [f32; 4] {
        theme::active().cosmic().corner_radii.radius_m
    }

    /// Large radius for prominent components
    pub fn l() -> [f32; 4] {
        theme::active().cosmic().corner_radii.radius_l
    }
}

/// Semantic colors for UI states
pub struct SemanticColors;

impl SemanticColors {
    /// Accent color for primary actions and selection
    pub fn accent() -> Color {
        theme::active().cosmic().accent_color().into()
    }

    /// Destructive color for dangerous/delete actions
    pub fn destructive() -> Color {
        theme::active().cosmic().destructive_color().into()
    }

    /// Success color for positive states
    pub fn success() -> Color {
        theme::active().cosmic().success_color().into()
    }

    /// Warning color for attention-needed states
    pub fn warning() -> Color {
        theme::active().cosmic().warning_color().into()
    }

    /// Text color on background
    pub fn on_bg() -> Color {
        theme::active().cosmic().on_bg_color().into()
    }

    /// Background color
    pub fn bg() -> Color {
        theme::active().cosmic().background.base.into()
    }

    /// Accent color with custom alpha for subtle highlights
    ///
    /// Alpha value is clamped to 0.0..=1.0 range for safety.
    pub fn accent_alpha(alpha: f32) -> Color {
        debug_assert!(
            (0.0..=1.0).contains(&alpha),
            "Alpha must be in range 0.0..=1.0, got {}",
            alpha
        );
        let alpha = alpha.clamp(0.0, 1.0);
        let accent = Self::accent();
        Color::from_rgba(accent.r, accent.g, accent.b, alpha)
    }

    /// Destructive color with custom alpha
    ///
    /// Alpha value is clamped to 0.0..=1.0 range for safety.
    pub fn destructive_alpha(alpha: f32) -> Color {
        debug_assert!(
            (0.0..=1.0).contains(&alpha),
            "Alpha must be in range 0.0..=1.0, got {}",
            alpha
        );
        let alpha = alpha.clamp(0.0, 1.0);
        let destructive = Self::destructive();
        Color::from_rgba(destructive.r, destructive.g, destructive.b, alpha)
    }

    /// Warning color with custom alpha
    ///
    /// Alpha value is clamped to 0.0..=1.0 range for safety.
    pub fn warning_alpha(alpha: f32) -> Color {
        debug_assert!(
            (0.0..=1.0).contains(&alpha),
            "Alpha must be in range 0.0..=1.0, got {}",
            alpha
        );
        let alpha = alpha.clamp(0.0, 1.0);
        let warning = Self::warning();
        Color::from_rgba(warning.r, warning.g, warning.b, alpha)
    }
}

/// Standard component sizes
pub struct ComponentSize;

impl ComponentSize {
    /// Icon size for applet panel button (24px)
    pub const APPLET_ICON: u16 = 24;

    /// Icon size for notification app icon (32px)
    pub const NOTIFICATION_ICON: u16 = 32;

    /// Icon size for action buttons (20px)
    pub const ACTION_ICON: u16 = 20;

    /// Icon size for status indicators (16px)
    pub const STATUS_ICON: u16 = 16;

    /// Standard notification card width
    pub const NOTIFICATION_WIDTH: f32 = 400.0;

    /// Maximum popup height before scrolling
    pub const MAX_POPUP_HEIGHT: f32 = 600.0;

    /// Border width for selected notifications
    pub const SELECTION_BORDER_WIDTH: f32 = 2.0;

    /// Border width for urgency indicator (left border)
    pub const URGENCY_BORDER_WIDTH: f32 = 3.0;
}

/// Urgency level visual styling
pub struct UrgencyStyle;

impl UrgencyStyle {
    /// Get border color for urgency level
    pub fn border_color(urgency: crate::dbus::Urgency) -> Color {
        use crate::dbus::Urgency;
        match urgency {
            Urgency::Low => {
                // Neutral/subtle border
                let on_bg: Color = theme::active().cosmic().on_bg_color().into();
                Color::from_rgba(on_bg.r, on_bg.g, on_bg.b, 0.1)
            }
            Urgency::Normal => {
                // Accent color for normal priority
                SemanticColors::accent_alpha(0.3)
            }
            Urgency::Critical => {
                // Warning/destructive color for critical
                SemanticColors::destructive_alpha(0.6)
            }
        }
    }

    /// Get icon name for urgency level
    pub fn icon_name(urgency: crate::dbus::Urgency) -> &'static str {
        use crate::dbus::Urgency;
        match urgency {
            Urgency::Low => "notification-symbolic",
            Urgency::Normal => "notification-symbolic",
            Urgency::Critical => "dialog-warning-symbolic",
        }
    }
}
