// Position settings widget
//
// Displays popup position configuration controls including mode selection,
// anchor point selection, offset sliders, and position preview.

use cosmic::Element;
use cosmic::iced::Length;
use cosmic::widget::{column, container, divider, row, slider, text, toggler};

// Import button constructors
use cosmic::widget::button::standard as button_standard;

use crate::config::{PanelAnchor, PopupPosition, PositionMode};

/// Create a position settings widget
///
/// Displays controls for:
/// - Position mode (Auto / Panel Relative)
/// - Anchor point selection
/// - X/Y offset sliders
/// - Snap-to-edge toggle
/// - Position preview button
pub fn position_settings<'a, Message>(
    position: &'a PopupPosition,
    on_mode_change: impl Fn(PositionMode) -> Message + 'a + Clone,
    on_anchor_change: impl Fn(PanelAnchor) -> Message + 'a + Clone,
    on_offset_x_change: impl Fn(i32) -> Message + 'a + Clone,
    on_offset_y_change: impl Fn(i32) -> Message + 'a + Clone,
    on_snap_toggle: Message,
    on_snap_threshold_change: impl Fn(u32) -> Message + 'a + Clone,
    on_preview: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a + 'static,
{
    let mut content = column().spacing(12.0).padding(16.0);

    // Section header
    content = content.push(text("Popup Position").size(16));

    content = content.push(divider::horizontal::default());

    // Position mode selector
    content = content.push(text("Position Mode").size(14));

    let mode_buttons = row()
        .push(mode_button(
            "Auto",
            position.mode == PositionMode::Auto,
            on_mode_change.clone(),
            PositionMode::Auto,
        ))
        .push(mode_button(
            "Panel Relative",
            position.mode == PositionMode::PanelRelative,
            on_mode_change.clone(),
            PositionMode::PanelRelative,
        ))
        .spacing(8.0);

    content = content.push(mode_buttons);

    // Only show detailed controls in PanelRelative mode
    if position.mode == PositionMode::PanelRelative {
        content = content.push(divider::horizontal::default());

        // Anchor point selector
        content = content.push(text("Anchor Point").size(14));

        let anchor_row1 = row()
            .push(anchor_button(
                "Start",
                position.anchor == PanelAnchor::Start,
                on_anchor_change.clone(),
                PanelAnchor::Start,
            ))
            .push(anchor_button(
                "Center",
                position.anchor == PanelAnchor::Center,
                on_anchor_change.clone(),
                PanelAnchor::Center,
            ))
            .spacing(8.0);

        let anchor_row2 = row()
            .push(anchor_button(
                "End",
                position.anchor == PanelAnchor::End,
                on_anchor_change.clone(),
                PanelAnchor::End,
            ))
            .push(anchor_button(
                "Applet Icon",
                position.anchor == PanelAnchor::AppletIcon,
                on_anchor_change.clone(),
                PanelAnchor::AppletIcon,
            ))
            .spacing(8.0);

        content = content.push(anchor_row1).push(anchor_row2);

        // X Offset slider
        content = content.push(text(format!("X Offset: {} px", position.offset_x)).size(14));

        let x_slider = slider(-500..=500, position.offset_x, on_offset_x_change.clone())
            .step(10)
            .width(Length::Fill);

        content = content.push(x_slider);

        // Y Offset slider
        content = content.push(text(format!("Y Offset: {} px", position.offset_y)).size(14));

        let y_slider = slider(-500..=500, position.offset_y, on_offset_y_change.clone())
            .step(10)
            .width(Length::Fill);

        content = content.push(y_slider);

        // Snap to edge toggle
        let snap_row = row()
            .push(text("Snap to Edge").size(14).width(Length::Fill))
            .push(toggler(position.snap_to_edge).on_toggle(move |_| on_snap_toggle.clone()))
            .spacing(8.0)
            .align_y(cosmic::iced::Alignment::Center);

        content = content.push(snap_row);

        // Snap threshold (only if snap enabled)
        if position.snap_to_edge {
            content = content
                .push(text(format!("Snap Threshold: {} px", position.snap_threshold)).size(14));

            let threshold_slider = slider(
                5..=100,
                position.snap_threshold,
                on_snap_threshold_change.clone(),
            )
            .step(5u32)
            .width(Length::Fill);

            content = content.push(threshold_slider);
        }
    }

    // Preview button
    content = content.push(divider::horizontal::default());

    let preview_button = button_standard("Preview Position")
        .on_press(on_preview.clone())
        .width(Length::Fill);

    content = content.push(preview_button);

    container(content).width(Length::Fill).into()
}

/// Create a position mode button
fn mode_button<'a, Message>(
    label: &'a str,
    _is_selected: bool,
    on_press: impl Fn(PositionMode) -> Message + 'a,
    mode: PositionMode,
) -> Element<'a, Message>
where
    Message: Clone + 'a + 'static,
{
    let btn = button_standard(label)
        .on_press(on_press(mode))
        .padding([6, 12]);

    btn.into()
}

/// Create an anchor point button
fn anchor_button<'a, Message>(
    label: &'a str,
    _is_selected: bool,
    on_press: impl Fn(PanelAnchor) -> Message + 'a,
    anchor: PanelAnchor,
) -> Element<'a, Message>
where
    Message: Clone + 'a + 'static,
{
    let btn = button_standard(label)
        .on_press(on_press(anchor))
        .padding([6, 12]);

    btn.into()
}
