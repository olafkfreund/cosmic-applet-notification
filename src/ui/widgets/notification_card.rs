// Notification card widget
//
// Displays a single notification with app icon, summary, body, timestamp, and dismiss button.
// Follows COSMIC design patterns for consistent appearance and behavior.

use cosmic::iced::Length;
use cosmic::widget::{button, column, container, icon, row, text};
use cosmic::Element;

use crate::dbus::{Notification, Urgency};
use crate::ui::animation::NotificationAnimation;
use crate::ui::theme::{ComponentSize, SemanticColors, Spacing, UrgencyStyle};
use crate::ui::url_parser::{parse_text, TextSegment};

/// Create a notification card widget
///
/// Displays notification information with:
/// - Urgency indicator (colored left border)
/// - App icon (if available)
/// - App name and timestamp
/// - Summary and body text with clickable URLs
/// - Action buttons
/// - Dismiss button
///
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

    // Header row: app icon, app name, timestamp, dismiss button
    let mut header_row = row()
        .spacing(Spacing::xs())
        .align_y(cosmic::iced::Alignment::Center);

    // Resolve app icon with fallback to urgency icon
    let app_icon: cosmic::widget::Icon = resolve_notification_icon(notification);
    header_row = header_row.push(app_icon);

    // App name
    header_row = header_row.push(text::body(&notification.app_name));

    // Spacer to push timestamp and dismiss to the right
    header_row = header_row.push(cosmic::iced::widget::horizontal_space());

    // Timestamp
    header_row = header_row.push(text::caption(format_timestamp(&notification.timestamp)));

    // Dismiss button (icon button for better UX)
    let dismiss_btn = button::icon(icon::from_name("window-close-symbolic").size(16))
        .on_press(on_dismiss(notification_id))
        .padding(Spacing::xxs());

    header_row = header_row.push(dismiss_btn);

    // Summary text (prominent)
    let summary = text::title4(&notification.summary);

    // Main content column
    let mut content = column()
        .push(header_row)
        .push(summary)
        .spacing(Spacing::xs());

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

    // Apply padding to content
    let content = content
        .padding([Spacing::s(), Spacing::m()])
        .width(Length::Fill);

    // Add urgency indicator bar (left border) for non-selected notifications
    let content_with_urgency: Element<'a, Message> = if is_selected {
        // Selected: use full border in style
        content.into()
    } else {
        // Not selected: add colored left border indicator
        let urgency_bar = container(cosmic::iced::widget::vertical_space())
            .width(Length::Fixed(ComponentSize::URGENCY_BORDER_WIDTH))
            .height(Length::Fill)
            .style(urgency_bar_style(notification.urgency()));

        row()
            .push(urgency_bar)
            .push(content)
            .width(Length::Fill)
            .into()
    };

    // Wrap in container with selection styling
    let container = container(content_with_urgency)
        .width(Length::Fill)
        .style(notification_style(notification.urgency(), is_selected));

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

/// Create a container style for urgency indicator bar (left border)
///
/// Creates a colored vertical bar that indicates notification urgency level.
fn urgency_bar_style(
    urgency: Urgency,
) -> impl Fn(&cosmic::Theme) -> cosmic::iced::widget::container::Style {
    move |_theme: &cosmic::Theme| {
        let border_color = UrgencyStyle::border_color(urgency);

        cosmic::iced::widget::container::Style {
            text_color: None,
            background: Some(border_color.into()),
            border: cosmic::iced::Border::default(),
            shadow: cosmic::iced::Shadow::default(),
            icon_color: None,
        }
    }
}

/// Create a container style for notifications with selection state
///
/// Applies:
/// - Selection highlight with accent background tint (15% opacity) and border
/// - Rounded corners using COSMIC theme
/// - Urgency is indicated via separate left border bar (see urgency_bar_style)
fn notification_style(
    _urgency: Urgency,
    is_selected: bool,
) -> impl Fn(&cosmic::Theme) -> cosmic::iced::widget::container::Style {
    move |theme: &cosmic::Theme| {
        let cosmic = theme.cosmic();

        // Selection background tint and border
        let (background, border) = if is_selected {
            (
                Some(SemanticColors::accent_alpha(0.15).into()),
                cosmic::iced::Border {
                    color: cosmic.accent.base.into(),
                    width: ComponentSize::SELECTION_BORDER_WIDTH,
                    radius: cosmic.corner_radii.radius_m.into(),
                },
            )
        } else {
            (
                None,
                cosmic::iced::Border {
                    color: cosmic::iced::Color::TRANSPARENT,
                    width: 0.0,
                    radius: cosmic.corner_radii.radius_m.into(),
                },
            )
        };

        cosmic::iced::widget::container::Style {
            text_color: None,
            background,
            border,
            shadow: cosmic::iced::Shadow::default(),
            icon_color: None,
        }
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
    let mut content_row = row()
        .spacing(Spacing::xxs())
        .align_y(cosmic::iced::Alignment::Center);

    for segment in segments {
        match segment {
            TextSegment::Text(txt) => {
                // Add plain text using COSMIC body style
                content_row = content_row.push(text::body(txt));
            }
            TextSegment::Link {
                text: link_text,
                url,
            } => {
                // Add clickable link button with link styling
                let link_button = button::link(link_text.clone())
                    .on_press(url_message(url))
                    .padding([0, Spacing::xxs()]);

                content_row = content_row.push(link_button);
            }
        }
    }

    content_row.into()
}

/// Render action buttons for notification actions
///
/// Creates a row of buttons for each notification action.
/// The first action is styled as suggested (primary), others as standard.
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
    let mut action_row = row()
        .spacing(Spacing::xs())
        .padding([Spacing::s(), 0, 0, 0]);

    for (index, action) in actions.iter().enumerate() {
        let action_key = action.key.clone();
        let action_label = action.label.clone();
        let is_selected = selected_action_index == Some(index);
        let is_first = index == 0;

        // Style button based on selection and position
        let action_button = if is_selected {
            // Selected action: use suggested style
            button::suggested(action_label)
                .on_press(on_action(notification_id, action_key))
                .padding([Spacing::xxs(), Spacing::s()])
        } else if is_first {
            // First action: use suggested style (primary action)
            button::suggested(action_label)
                .on_press(on_action(notification_id, action_key))
                .padding([Spacing::xxs(), Spacing::s()])
        } else {
            // Other actions: use standard style
            button::standard(action_label)
                .on_press(on_action(notification_id, action_key))
                .padding([Spacing::xxs(), Spacing::s()])
        };

        action_row = action_row.push(action_button);
    }

    action_row.into()
}

/// Resolve notification icon with fallback
///
/// Attempts to load the application icon, falling back to urgency indicator if unavailable.
///
/// # Priority
/// 1. Application icon from `app_icon` field (if valid name/path)
/// 2. Urgency icon based on notification level (fallback)
fn resolve_notification_icon(notification: &Notification) -> cosmic::widget::Icon {
    // Determine icon name with fallback chain
    let icon_name = if !notification.app_icon.is_empty() {
        // Use provided app icon
        notification.app_icon.as_str()
    } else {
        // Fallback to urgency-based icon
        UrgencyStyle::icon_name(notification.urgency())
    };

    // Create icon - libcosmic will handle missing icons gracefully
    icon::from_name(icon_name)
        .size(ComponentSize::NOTIFICATION_ICON)
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
