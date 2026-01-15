// Popup positioning logic
//
// Calculates popup position based on configuration and panel location.
// Uses Wayland XDG-shell positioner (anchor/gravity/offset system).

use crate::config::{PanelAnchor, PopupPosition, PositionMode};

// Re-export Wayland XDG-shell positioning types
pub use wayland_protocols::xdg::shell::client::xdg_positioner::{Anchor, Gravity};

/// Panel edge location
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelEdge {
    Top,
    Bottom,
    Left,
    Right,
}

impl PanelEdge {
    /// Detect panel edge from COSMIC environment
    ///
    /// Reads the COSMIC_PANEL_ANCHOR environment variable to determine
    /// which edge of the screen the panel is attached to.
    ///
    /// Fallback: Bottom (most common panel position)
    pub fn detect() -> Self {
        if let Ok(anchor) = std::env::var("COSMIC_PANEL_ANCHOR") {
            match anchor.to_lowercase().as_str() {
                "top" => Self::Top,
                "bottom" => Self::Bottom,
                "left" => Self::Left,
                "right" => Self::Right,
                _ => {
                    tracing::warn!(
                        "Unknown COSMIC_PANEL_ANCHOR value: '{}', defaulting to Bottom",
                        anchor
                    );
                    Self::Bottom
                }
            }
        } else {
            tracing::debug!("COSMIC_PANEL_ANCHOR not set, defaulting to Bottom panel");
            Self::Bottom
        }
    }
}

/// Calculate popup positioner settings based on configuration
///
/// Returns (anchor, gravity, offset) tuple for XDG-shell positioner.
///
/// # Arguments
///
/// * `config` - Popup position configuration
/// * `panel_edge` - Which edge the COSMIC panel is on
///
/// # Returns
///
/// * `Anchor` - Which edge/corner of the anchor rectangle to attach to
/// * `Gravity` - Which direction the popup extends from the anchor point
/// * `(i32, i32)` - X and Y offset in pixels from the anchor point
pub fn calculate_popup_position(
    config: &PopupPosition,
    panel_edge: PanelEdge,
) -> (Anchor, Gravity, (i32, i32)) {
    match config.mode {
        PositionMode::Auto => calculate_auto_position(panel_edge),
        PositionMode::PanelRelative => calculate_panel_relative_position(config, panel_edge),
    }
}

/// Calculate automatic position based on panel edge
///
/// Uses COSMIC's default behavior: popup extends away from panel.
fn calculate_auto_position(panel_edge: PanelEdge) -> (Anchor, Gravity, (i32, i32)) {
    match panel_edge {
        PanelEdge::Top => (Anchor::Bottom, Gravity::Bottom, (0, 0)),
        PanelEdge::Bottom => (Anchor::Top, Gravity::Top, (0, 0)),
        PanelEdge::Left => (Anchor::Right, Gravity::Right, (0, 0)),
        PanelEdge::Right => (Anchor::Left, Gravity::Left, (0, 0)),
    }
}

/// Calculate panel-relative position with custom anchor and offsets
fn calculate_panel_relative_position(
    config: &PopupPosition,
    panel_edge: PanelEdge,
) -> (Anchor, Gravity, (i32, i32)) {
    // Step 1: Determine anchor point on panel based on panel edge and anchor config
    let anchor = determine_anchor(panel_edge, config.anchor);

    // Step 2: Determine gravity (direction popup extends)
    let gravity = determine_gravity(panel_edge);

    // Step 3: Calculate offset with snap-to-edge logic
    let offset = calculate_offset(config);

    (anchor, gravity, offset)
}

/// Determine anchor point based on panel edge and configured anchor
fn determine_anchor(panel_edge: PanelEdge, panel_anchor: PanelAnchor) -> Anchor {
    match (panel_edge, panel_anchor) {
        // Horizontal panels (Top/Bottom)
        (PanelEdge::Top | PanelEdge::Bottom, PanelAnchor::Start) => Anchor::BottomLeft,
        (PanelEdge::Top | PanelEdge::Bottom, PanelAnchor::Center) => Anchor::Bottom,
        (PanelEdge::Top | PanelEdge::Bottom, PanelAnchor::End) => Anchor::BottomRight,
        (PanelEdge::Top, PanelAnchor::AppletIcon) => Anchor::Bottom,
        (PanelEdge::Bottom, PanelAnchor::AppletIcon) => Anchor::Top,

        // Vertical panels (Left/Right)
        (PanelEdge::Left | PanelEdge::Right, PanelAnchor::Start) => Anchor::TopRight,
        (PanelEdge::Left | PanelEdge::Right, PanelAnchor::Center) => Anchor::Right,
        (PanelEdge::Left | PanelEdge::Right, PanelAnchor::End) => Anchor::BottomRight,
        (PanelEdge::Left, PanelAnchor::AppletIcon) => Anchor::Right,
        (PanelEdge::Right, PanelAnchor::AppletIcon) => Anchor::Left,
    }
}

/// Determine gravity based on panel edge
///
/// Gravity determines which direction the popup extends from the anchor point.
/// Always extends away from the panel.
fn determine_gravity(panel_edge: PanelEdge) -> Gravity {
    match panel_edge {
        PanelEdge::Top => Gravity::Bottom,
        PanelEdge::Bottom => Gravity::Top,
        PanelEdge::Left => Gravity::Right,
        PanelEdge::Right => Gravity::Left,
    }
}

/// Calculate offset with optional snap-to-edge behavior
fn calculate_offset(config: &PopupPosition) -> (i32, i32) {
    let mut offset_x = config.offset_x;
    let mut offset_y = config.offset_y;

    if config.snap_to_edge {
        let threshold = config.snap_threshold as i32;

        // Snap to zero if within threshold
        if offset_x.abs() < threshold {
            offset_x = 0;
        }
        if offset_y.abs() < threshold {
            offset_y = 0;
        }
    }

    (offset_x, offset_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_position_all_edges() {
        // Top panel: popup extends downward
        let (anchor, gravity, offset) = calculate_auto_position(PanelEdge::Top);
        assert_eq!(anchor, Anchor::Bottom);
        assert_eq!(gravity, Gravity::Bottom);
        assert_eq!(offset, (0, 0));

        // Bottom panel: popup extends upward
        let (anchor, gravity, offset) = calculate_auto_position(PanelEdge::Bottom);
        assert_eq!(anchor, Anchor::Top);
        assert_eq!(gravity, Gravity::Top);
        assert_eq!(offset, (0, 0));

        // Left panel: popup extends rightward
        let (anchor, gravity, offset) = calculate_auto_position(PanelEdge::Left);
        assert_eq!(anchor, Anchor::Right);
        assert_eq!(gravity, Gravity::Right);
        assert_eq!(offset, (0, 0));

        // Right panel: popup extends leftward
        let (anchor, gravity, offset) = calculate_auto_position(PanelEdge::Right);
        assert_eq!(anchor, Anchor::Left);
        assert_eq!(gravity, Gravity::Left);
        assert_eq!(offset, (0, 0));
    }

    #[test]
    fn test_panel_relative_applet_icon() {
        let mut config = PopupPosition::default();
        config.mode = PositionMode::PanelRelative;
        config.anchor = PanelAnchor::AppletIcon;

        // Bottom panel with applet icon anchor
        let (anchor, gravity, offset) = calculate_popup_position(&config, PanelEdge::Bottom);
        assert_eq!(anchor, Anchor::Top);
        assert_eq!(gravity, Gravity::Top);
        assert_eq!(offset, (0, 0));

        // Top panel with applet icon anchor
        let (anchor, gravity, offset) = calculate_popup_position(&config, PanelEdge::Top);
        assert_eq!(anchor, Anchor::Bottom);
        assert_eq!(gravity, Gravity::Bottom);
        assert_eq!(offset, (0, 0));
    }

    #[test]
    fn test_snap_to_edge() {
        let mut config = PopupPosition::default();
        config.mode = PositionMode::PanelRelative;
        config.snap_to_edge = true;
        config.snap_threshold = 20;
        config.offset_x = 15; // Within threshold
        config.offset_y = 100; // Outside threshold

        let (_, _, (x, y)) = calculate_popup_position(&config, PanelEdge::Bottom);
        assert_eq!(x, 0); // Snapped to 0
        assert_eq!(y, 100); // Not snapped
    }

    #[test]
    fn test_snap_disabled() {
        let mut config = PopupPosition::default();
        config.mode = PositionMode::PanelRelative;
        config.snap_to_edge = false;
        config.offset_x = 15;
        config.offset_y = 10;

        let (_, _, (x, y)) = calculate_popup_position(&config, PanelEdge::Bottom);
        assert_eq!(x, 15); // Not snapped
        assert_eq!(y, 10); // Not snapped
    }

    #[test]
    fn test_panel_anchor_start() {
        let mut config = PopupPosition::default();
        config.mode = PositionMode::PanelRelative;
        config.anchor = PanelAnchor::Start;

        // Bottom panel (horizontal) with Start anchor
        let (anchor, _, _) = calculate_popup_position(&config, PanelEdge::Bottom);
        assert_eq!(anchor, Anchor::BottomLeft); // Left side of horizontal panel

        // Left panel (vertical) with Start anchor
        let (anchor, _, _) = calculate_popup_position(&config, PanelEdge::Left);
        assert_eq!(anchor, Anchor::TopRight); // Top side of vertical panel
    }

    #[test]
    fn test_panel_anchor_center() {
        let mut config = PopupPosition::default();
        config.mode = PositionMode::PanelRelative;
        config.anchor = PanelAnchor::Center;

        // Bottom panel with Center anchor
        let (anchor, _, _) = calculate_popup_position(&config, PanelEdge::Bottom);
        assert_eq!(anchor, Anchor::Bottom); // Center of horizontal panel

        // Right panel with Center anchor
        let (anchor, _, _) = calculate_popup_position(&config, PanelEdge::Right);
        assert_eq!(anchor, Anchor::Right); // Center of vertical panel
    }

    #[test]
    fn test_custom_offsets() {
        let mut config = PopupPosition::default();
        config.mode = PositionMode::PanelRelative;
        config.offset_x = 50;
        config.offset_y = -20;
        config.snap_to_edge = false;

        let (_, _, (x, y)) = calculate_popup_position(&config, PanelEdge::Bottom);
        assert_eq!(x, 50);
        assert_eq!(y, -20);
    }

    #[test]
    fn test_negative_offsets_with_snap() {
        let mut config = PopupPosition::default();
        config.mode = PositionMode::PanelRelative;
        config.snap_to_edge = true;
        config.snap_threshold = 25;
        config.offset_x = -10; // Within threshold (absolute value)
        config.offset_y = -30; // Outside threshold

        let (_, _, (x, y)) = calculate_popup_position(&config, PanelEdge::Bottom);
        assert_eq!(x, 0); // Snapped
        assert_eq!(y, -30); // Not snapped
    }
}
