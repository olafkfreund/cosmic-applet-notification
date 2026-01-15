// Filter settings widget
//
// Displays notification filter settings and controls for managing
// per-application filters, urgency levels, and Do Not Disturb mode.

use cosmic::iced::Length;
use cosmic::widget::{button, column, container, divider, row, text, toggler};
use cosmic::Element;

use crate::config::AppletConfig;
use std::collections::HashMap;

/// Create a filter settings widget
///
/// Displays controls for:
/// - Do Not Disturb mode toggle
/// - Minimum urgency level selection
/// - Per-application filter management
pub fn filter_settings<'a, Message>(
    config: &AppletConfig,
    on_toggle_dnd: Message,
    on_urgency_change: impl Fn(u8) -> Message + 'a + Clone,
    on_app_filter_toggle: impl Fn(String, bool) -> Message + 'a + Clone,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let mut content = column![].spacing(12).padding(16);

    // Section header
    content = content.push(
        text("Notification Settings")
            .size(16)
            .style(cosmic::theme::Text::Default),
    );

    content = content.push(divider::horizontal::default());

    // Do Not Disturb toggle
    let dnd_row = row![
        text("Do Not Disturb")
            .size(14)
            .width(Length::Fill)
            .style(cosmic::theme::Text::Default),
        toggler(None, config.do_not_disturb, |_| on_toggle_dnd),
    ]
    .spacing(8)
    .align_items(cosmic::iced::Alignment::Center);

    content = content.push(dnd_row);

    // Urgency level selector
    content = content.push(
        text("Minimum Urgency Level")
            .size(14)
            .style(cosmic::theme::Text::Default),
    );

    let urgency_buttons = row![
        urgency_button(
            "All",
            config.min_urgency_level == 0,
            on_urgency_change.clone(),
            0
        ),
        urgency_button(
            "Normal+",
            config.min_urgency_level == 1,
            on_urgency_change.clone(),
            1
        ),
        urgency_button(
            "Critical",
            config.min_urgency_level == 2,
            on_urgency_change.clone(),
            2
        ),
    ]
    .spacing(8);

    content = content.push(urgency_buttons);

    // App filters section
    if !config.app_filters.is_empty() {
        content = content.push(divider::horizontal::default());

        content = content.push(
            text("App Filters")
                .size(14)
                .style(cosmic::theme::Text::Default),
        );

        // Sort apps alphabetically for consistent display
        let mut apps: Vec<(&String, &bool)> = config.app_filters.iter().collect();
        apps.sort_by(|a, b| a.0.cmp(b.0));

        for (app_name, is_blocked) in apps {
            let app_row = row![
                text(app_name)
                    .size(12)
                    .width(Length::Fill)
                    .style(cosmic::theme::Text::Default),
                text(if *is_blocked { "Blocked" } else { "Allowed" })
                    .size(12)
                    .style(if *is_blocked {
                        cosmic::theme::Text::Muted
                    } else {
                        cosmic::theme::Text::Accent
                    }),
                toggler(None, !is_blocked, {
                    let app_name = app_name.clone();
                    let on_app_filter_toggle = on_app_filter_toggle.clone();
                    move |enabled| on_app_filter_toggle(app_name.clone(), enabled)
                }),
            ]
            .spacing(8)
            .align_items(cosmic::iced::Alignment::Center);

            content = content.push(app_row);
        }
    }

    container(content)
        .width(Length::Fill)
        .style(cosmic::theme::Container::Background)
        .into()
}

/// Create an urgency level button
fn urgency_button<'a, Message>(
    label: &str,
    is_selected: bool,
    on_press: impl Fn(u8) -> Message + 'a,
    level: u8,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let btn = button(text(label).size(12))
        .on_press(on_press(level))
        .padding([6, 12])
        .style(if is_selected {
            cosmic::theme::Button::Suggested
        } else {
            cosmic::theme::Button::Standard
        });

    btn.into()
}

/// Message for filter settings updates
#[derive(Debug, Clone)]
pub enum FilterSettingsMessage {
    /// Toggle Do Not Disturb mode
    ToggleDND,
    /// Change minimum urgency level
    SetUrgencyLevel(u8),
    /// Toggle app filter (app_name, enabled)
    ToggleAppFilter(String, bool),
}
