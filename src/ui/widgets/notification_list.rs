// Notification list widget
//
// Displays a scrollable list of notifications with empty state handling.

use std::collections::VecDeque;

use cosmic::Element;
use cosmic::iced::Length;
use cosmic::widget::{column, container, scrollable, text};

use crate::dbus::Notification;
use crate::ui::widgets::notification_card;

/// Create a notification list widget
///
/// Displays notifications in a scrollable column with clickable URLs and action buttons.
/// Shows empty state message when no notifications are present.
///
/// Performance: Accepts a reference to avoid copying notification data on every frame.
pub fn notification_list<'a, Message>(
    notifications: &'a VecDeque<Notification>,
    selected_index: Option<usize>,
    selected_action_index: Option<usize>,
    on_dismiss: impl Fn(u32) -> Message + 'a + Clone,
    on_url: impl Fn(String) -> Message + 'a + Clone,
    on_action: impl Fn(u32, String) -> Message + 'a + Clone,
) -> Element<'a, Message>
where
    Message: Clone + 'a + 'static,
{
    if notifications.is_empty() {
        // Empty state
        return container(
            column()
                .push(text("No Notifications").size(16))
                .push(text("You're all caught up!").size(12))
                .spacing(8.0)
                .align_x(cosmic::iced::Alignment::Center)
                .padding(32.0),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into();
    }

    // Build list of notification cards using functional approach
    // More efficient than repeated push() calls
    let cards = notifications.iter().enumerate().fold(
        column().spacing(8.0).padding(8.0),
        |col, (index, notification)| {
            let is_selected = selected_index == Some(index);
            // Only pass action index if this notification is selected
            let action_index = if is_selected {
                selected_action_index
            } else {
                None
            };
            col.push(notification_card::notification_card(
                notification,
                is_selected,
                action_index,
                on_dismiss.clone(),
                on_url.clone(),
                on_action.clone(),
            ))
        },
    );

    // Wrap in scrollable container
    scrollable(cards)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
