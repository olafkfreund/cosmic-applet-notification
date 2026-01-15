// Notification card widget
//
// Displays a single notification with app icon, summary, body, timestamp, and dismiss button.

use cosmic::iced::Length;
use cosmic::widget::{button, column, container, row, text};
use cosmic::Element;

use crate::dbus::Notification;
use crate::ui::url_parser::{parse_text, TextSegment};

/// Create a notification card widget
///
/// Displays notification information with a dismiss button, clickable URLs, and action buttons.
/// Uses COSMIC design patterns for consistent appearance.
pub fn notification_card<'a, Message>(
    notification: &Notification,
    on_dismiss: impl Fn(u32) -> Message + 'a,
    on_url: impl Fn(String) -> Message + 'a + Clone,
    on_action: impl Fn(u32, String) -> Message + 'a + Clone,
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

    // Add body text with clickable URLs if present
    if !notification.body.is_empty() {
        let body_content = render_text_with_links(&notification.body, on_url);
        content = content.push(body_content);
    }

    // Add action buttons if present
    if !notification.actions.is_empty() {
        let action_row = render_action_buttons(&notification.actions, notification_id, on_action);
        content = content.push(action_row);
    }

    let content = content.spacing(4).padding(12).width(Length::Fill);

    // Wrap in container with theme styling
    container(content)
        .style(cosmic::theme::Container::Card)
        .width(Length::Fill)
        .into()
}

/// Render text with clickable URL links
///
/// Parses the text for URLs and creates a row with text segments and link buttons.
fn render_text_with_links<'a, Message>(
    text_content: &str,
    url_message: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let segments = parse_text(text_content);

    // Create a wrapping row for text segments and links
    let mut content_row = row![]
        .spacing(4)
        .align_items(cosmic::iced::Alignment::Center);

    for segment in segments {
        match segment {
            TextSegment::Text(txt) => {
                // Add plain text
                content_row =
                    content_row.push(text(txt).size(12).style(cosmic::theme::Text::Muted));
            }
            TextSegment::Link {
                text: link_text,
                url,
            } => {
                // Add clickable link button
                let link_button =
                    button(text(&link_text).size(12).style(cosmic::theme::Text::Accent))
                        .on_press(url_message(url))
                        .padding([0, 4])
                        .style(cosmic::theme::Button::Link);

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
fn render_action_buttons<'a, Message>(
    actions: &[crate::dbus::NotificationAction],
    notification_id: u32,
    on_action: impl Fn(u32, String) -> Message + 'a + Clone,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let mut action_row = row![].spacing(8).padding([8, 0, 0, 0]);

    for action in actions {
        let action_key = action.key.clone();
        let action_button = button(text(&action.label).size(12))
            .on_press(on_action(notification_id, action_key))
            .padding([4, 12])
            .style(cosmic::theme::Button::Standard);

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
