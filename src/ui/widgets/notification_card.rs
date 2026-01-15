// Notification card widget
//
// Displays a single notification with app icon, summary, body, timestamp, and dismiss button.

use cosmic::iced::Length;
use cosmic::widget::{button, column, container, row, text};
use cosmic::Element;

use crate::dbus::Notification;

/// Create a notification card widget
///
/// Displays notification information with a dismiss button.
/// Uses COSMIC design patterns for consistent appearance.
pub fn notification_card<'a, Message>(
    notification: &Notification,
    on_dismiss: impl Fn(u32) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let notification_id = notification.id;

    // Header row: app name and timestamp
    let header = row![
        text(&notification.app_name)
            .size(12)
            .style(cosmic::theme::Text::Accent),
        text(format_timestamp(&notification.timestamp))
            .size(12)
            .style(cosmic::theme::Text::Muted),
    ]
    .spacing(8)
    .align_items(cosmic::iced::Alignment::Center);

    // Summary text (bold)
    let summary = text(&notification.summary)
        .size(14)
        .style(cosmic::theme::Text::Default);

    // Dismiss button
    let dismiss_btn = button(text("✕").size(16))
        .on_press(on_dismiss(notification_id))
        .padding(4)
        .style(cosmic::theme::Button::Text);

    // Main content column - conditionally add body if present
    let mut content = column![
        row![header, dismiss_btn.width(Length::Shrink)]
            .spacing(8)
            .align_items(cosmic::iced::Alignment::Center)
            .width(Length::Fill),
        summary,
    ];

    // Add body text only if present (avoids empty text widget)
    if !notification.body.is_empty() {
        content = content.push(
            text(&notification.body)
                .size(12)
                .style(cosmic::theme::Text::Muted),
        );
    }

    let content = content.spacing(4).padding(12).width(Length::Fill);

    // Wrap in container with theme styling
    container(content)
        .style(cosmic::theme::Container::Card)
        .width(Length::Fill)
        .into()
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
