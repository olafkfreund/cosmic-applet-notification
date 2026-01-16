// Notification card widget
//
// Displays a single notification with app icon, summary, body, timestamp, and dismiss button.

use cosmic::iced::Length;
use cosmic::widget::{button, column, container, row, text};
use cosmic::Element;

// Import button constructors
use cosmic::widget::button::link as button_link;
use cosmic::widget::button::standard as button_standard;
use cosmic::widget::button::suggested as button_suggested;

use crate::dbus::Notification;
use crate::ui::animation::NotificationAnimation;
use crate::ui::url_parser::{parse_text, TextSegment};

/// Create a notification card widget
///
/// Displays notification information with a dismiss button, clickable URLs, and action buttons.
/// Uses COSMIC design patterns for consistent appearance.
///
/// Performance: Accepts a reference to avoid cloning notification data on every frame.
pub fn notification_card<'a, Message>(
    notification: &'a Notification,
    animation: Option<&'a NotificationAnimation>,
    is_selected: bool,
    selected_action_index: Option<usize>,
    on_dismiss: impl Fn(u32) -> Message + 'a,
    on_url: impl Fn(String) -> Message + 'a + Clone,
    on_action: impl Fn(u32, String) -> Message + 'a + Clone,
) -> Element<'a, Message>
where
    Message: Clone + 'a + 'static,
{
    let notification_id = notification.id;

    // Header row: app name and timestamp
    let header = row()
        .push(text(&notification.app_name).size(12))
        .push(text(format_timestamp(&notification.timestamp)).size(12))
        .spacing(8.0)
        .align_y(cosmic::iced::Alignment::Center);

    // Summary text (bold)
    let summary = text(&notification.summary).size(14);

    // Dismiss button
    let dismiss_btn = button::text("✕")
        .on_press(on_dismiss(notification_id))
        .padding(4.0);

    // Main content column - conditionally add body if present
    let mut content = column()
        .push(
            row()
                .push(header)
                .push(dismiss_btn.width(Length::Shrink))
                .spacing(8.0)
                .align_y(cosmic::iced::Alignment::Center)
                .width(Length::Fill),
        )
        .push(summary);

    // Add body text with clickable URLs if present
    if !notification.body.is_empty() {
        let body_content = render_text_with_links(&notification.body, on_url);
        content = content.push(body_content);
    }

    // Add action buttons if present
    if !notification.actions.is_empty() {
        // Validate action index is within bounds
        let selected_action = if is_selected {
            selected_action_index.filter(|&idx| idx < notification.actions.len())
        } else {
            None
        };
        let action_row = render_action_buttons(
            &notification.actions,
            notification_id,
            selected_action,
            on_action,
        );
        content = content.push(action_row);
    }

    let content = content.spacing(4.0).padding(12.0).width(Length::Fill);

    // Wrap in container with selection styling
    let container = container(content).width(Length::Fill);

    // Apply selection styling
    let container = if is_selected {
        container.style(selected_notification_style)
    } else {
        container
    };

    // Log animation state if present
    // TODO: Apply visual transformations (opacity, translation, scale) when iced supports it
    // Currently iced/libcosmic has limited transform/opacity support for Elements
    // Animation state is tracked correctly and can be applied via custom rendering in the future
    if let Some(anim) = animation {
        let opacity = anim.opacity.value();
        let translation_y = anim.translation_y.value();
        let scale = anim.scale.value();

        tracing::trace!(
            "Animation state for notification {}: opacity={:.2}, translation_y={:.2}, scale={:.2}",
            notification.id,
            opacity,
            translation_y,
            scale
        );
    }

    container.into()
}

/// Create a container style for selected notifications
///
/// Applies accent-colored border (2px) and subtle background tint (15% opacity)
/// to visually indicate the currently selected notification for keyboard navigation.
fn selected_notification_style(theme: &cosmic::Theme) -> cosmic::iced::widget::container::Style {
    let cosmic = theme.cosmic();
    let accent = cosmic.accent_color();

    cosmic::iced::widget::container::Style {
        text_color: None,
        // Subtle background tint with alpha
        background: Some(
            cosmic::iced::Color::from_rgba(accent.red, accent.green, accent.blue, 0.15).into(),
        ),
        // Accent border to show selection
        border: cosmic::iced::Border {
            color: cosmic.accent.base.into(),
            width: 2.0,
            radius: 8.0.into(),
        },
        shadow: cosmic::iced::Shadow::default(),
        icon_color: None,
    }
}

/// Render text with clickable URL links
///
/// Parses the text for URLs and creates a row with text segments and link buttons.
fn render_text_with_links<'a, Message>(
    text_content: &str,
    url_message: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a + 'static,
{
    let segments = parse_text(text_content);

    // Create a wrapping row for text segments and links
    let mut content_row = row().spacing(4.0).align_y(cosmic::iced::Alignment::Center);

    for segment in segments {
        match segment {
            TextSegment::Text(txt) => {
                // Add plain text
                content_row = content_row.push(text(txt).size(12));
            }
            TextSegment::Link {
                text: link_text,
                url,
            } => {
                // Add clickable link button
                let link_button = button_link(link_text.clone())
                    .on_press(url_message(url))
                    .padding([0, 4]);

                content_row = content_row.push(link_button);
            }
        }
    }

    content_row.into()
}

/// Render action buttons for notification actions
///
/// Creates a row of buttons for each notification action.
/// Action buttons are styled with the standard button theme.
/// The selected action (if any) is highlighted with accent styling.
fn render_action_buttons<'a, Message>(
    actions: &[crate::dbus::NotificationAction],
    notification_id: u32,
    selected_action_index: Option<usize>,
    on_action: impl Fn(u32, String) -> Message + 'a + Clone,
) -> Element<'a, Message>
where
    Message: Clone + 'a + 'static,
{
    let mut action_row = row().spacing(8.0).padding([8, 0, 0, 0]);

    for (index, action) in actions.iter().enumerate() {
        let action_key = action.key.clone();
        let action_label = action.label.clone();
        let is_selected = selected_action_index == Some(index);

        // Use suggested button styling for selected action
        let action_button = if is_selected {
            button_suggested(action_label)
                .on_press(on_action(notification_id, action_key))
                .padding([4, 12])
        } else {
            button_standard(action_label)
                .on_press(on_action(notification_id, action_key))
                .padding([4, 12])
        };

        action_row = action_row.push(action_button);
    }

    action_row.into()
}

/// Format timestamp for display
///
/// Shows relative time (e.g., "2m ago", "1h ago", "3d ago")
fn format_timestamp(timestamp: &chrono::DateTime<chrono::Local>) -> String {
    let now = chrono::Local::now();
    let duration = now.signed_duration_since(*timestamp);

    if duration.num_seconds() < 60 {
        "just now".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{}m ago", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{}h ago", duration.num_hours())
    } else {
        format!("{}d ago", duration.num_days())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Local};

    #[test]
    fn test_format_timestamp_just_now() {
        let timestamp = Local::now();
        assert_eq!(format_timestamp(&timestamp), "just now");
    }

    #[test]
    fn test_format_timestamp_minutes() {
        let timestamp = Local::now() - Duration::minutes(5);
        assert_eq!(format_timestamp(&timestamp), "5m ago");
    }

    #[test]
    fn test_format_timestamp_hours() {
        let timestamp = Local::now() - Duration::hours(2);
        assert_eq!(format_timestamp(&timestamp), "2h ago");
    }

    #[test]
    fn test_format_timestamp_days() {
        let timestamp = Local::now() - Duration::days(3);
        assert_eq!(format_timestamp(&timestamp), "3d ago");
    }
}
