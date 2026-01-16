// Notification list widget
//
// Displays a scrollable list of notifications with empty state handling.
// Follows COSMIC design patterns for consistent appearance.

use std::collections::{HashMap, VecDeque};

use cosmic::iced::Length;
use cosmic::widget::{column, container, icon, scrollable, text};
use cosmic::Element;

use crate::dbus::Notification;
use crate::ui::animation::NotificationAnimation;
use crate::ui::theme::{ComponentSize, Spacing};
use crate::ui::widgets::notification_card;

/// Create a notification list widget
///
/// Displays notifications in a scrollable column with clickable URLs and action buttons.
/// Shows empty state message when no notifications are present.
///
/// Performance: Accepts a reference to avoid copying notification data on every frame.
pub fn notification_list<'a, Message>(
    notifications: &'a VecDeque<Notification>,
    notification_animations: &'a HashMap<u32, NotificationAnimation>,
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
        // Empty state with COSMIC styling
        let empty_icon: cosmic::widget::Icon = icon::from_name("notification-symbolic")
            .size(ComponentSize::NOTIFICATION_ICON)
            .into();

        return container(
            column()
                .push(empty_icon)
                .push(text::title3("No Notifications"))
                .push(text::body("You're all caught up!"))
                .spacing(Spacing::s())
                .align_x(cosmic::iced::Alignment::Center)
                .padding(Spacing::xl()),
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
        column().spacing(Spacing::xs()).padding(Spacing::xs()),
        |col, (index, notification)| {
            let is_selected = selected_index == Some(index);
            // Only pass action index if this notification is selected
            let action_index = if is_selected {
                selected_action_index
            } else {
                None
            };

            // Get animation state for this notification (if any)
            let animation = notification_animations.get(&notification.id);

            col.push(notification_card::notification_card(
                notification,
                animation,
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
