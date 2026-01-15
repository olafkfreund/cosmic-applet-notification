// Notification list widget
//
// Displays a scrollable list of notifications with empty state handling.

use cosmic::iced::Length;
use cosmic::widget::{column, container, scrollable, text};
use cosmic::Element;

use crate::dbus::Notification;
use crate::ui::widgets::notification_card;

/// Create a notification list widget
///
/// Displays notifications in a scrollable column with clickable URLs.
/// Shows empty state message when no notifications are present.
pub fn notification_list<'a, Message>(
    notifications: &[Notification],
    on_dismiss: impl Fn(u32) -> Message + 'a + Clone,
    on_url: impl Fn(String) -> Message + 'a + Clone,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    if notifications.is_empty() {
        // Empty state
        return container(
            column![
                text("No Notifications")
                    .size(16)
                    .style(cosmic::theme::Text::Muted),
                text("You're all caught up!")
                    .size(12)
                    .style(cosmic::theme::Text::Muted),
            ]
            .spacing(8)
            .align_items(cosmic::iced::Alignment::Center)
            .padding(32),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into();
    }

    // Build list of notification cards using functional approach
    // More efficient than repeated push() calls
    let cards = notifications
        .iter()
        .fold(column![].spacing(8).padding(8), |col, notification| {
            col.push(notification_card::notification_card(
                notification,
                on_dismiss.clone(),
                on_url.clone(),
            ))
        });

    // Wrap in scrollable container
    scrollable(cards)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
