// Filter settings widget
//
// Displays notification filter settings and controls for managing
// per-application filters, urgency levels, and Do Not Disturb mode.

use cosmic::Element;
use cosmic::iced::Length;
use cosmic::widget::{column, container, divider, row, text, toggler};

// Import button constructors
use cosmic::widget::button::standard as button_standard;

use crate::config::AppletConfig;

/// Create a filter settings widget
///
/// Displays controls for:
/// - Do Not Disturb mode toggle
/// - Minimum urgency level selection
/// - Per-application filter management
pub fn filter_settings<'a, Message>(
    config: &'a AppletConfig,
    on_toggle_dnd: Message,
    on_urgency_change: impl Fn(u8) -> Message + 'a + Clone,
    on_app_filter_toggle: impl Fn(String, bool) -> Message + 'a + Clone,
) -> Element<'a, Message>
where
    Message: Clone + 'a + 'static,
{
    let mut content = column().spacing(12.0).padding(16.0);

    // Section header
    content = content.push(text("Notification Settings").size(16));

    // Keyboard shortcuts hint
    content = content.push(text("Shortcuts: Esc=Close, Ctrl+D=DND, Ctrl+1/2/3=Urgency").size(10));

    content = content.push(divider::horizontal::default());

    // Do Not Disturb toggle
    let dnd_row = row()
        .push(text("Do Not Disturb").size(14).width(Length::Fill))
        .push(toggler(config.do_not_disturb).on_toggle(move |_| on_toggle_dnd.clone()))
        .spacing(8.0)
        .align_y(cosmic::iced::Alignment::Center);

    content = content.push(dnd_row);

    // Urgency level selector
    content = content.push(text("Minimum Urgency Level").size(14));

    let urgency_buttons = row()
        .push(urgency_button(
            "All",
            config.min_urgency_level == 0,
            on_urgency_change.clone(),
            0,
        ))
        .push(urgency_button(
            "Normal+",
            config.min_urgency_level == 1,
            on_urgency_change.clone(),
            1,
        ))
        .push(urgency_button(
            "Critical",
            config.min_urgency_level == 2,
            on_urgency_change.clone(),
            2,
        ))
        .spacing(8.0);

    content = content.push(urgency_buttons);

    // App filters section
    if !config.app_filters.is_empty() {
        content = content.push(divider::horizontal::default());

        content = content.push(text("App Filters").size(14));

        // Sort apps alphabetically for consistent display
        let mut apps: Vec<(&String, &bool)> = config.app_filters.iter().collect();
        apps.sort_by(|a, b| a.0.cmp(b.0));

        for (app_name, is_blocked) in apps {
            let app_name_clone = app_name.clone();
            let on_app_filter_toggle_clone = on_app_filter_toggle.clone();
            let app_row = row()
                .push(text(app_name).size(12).width(Length::Fill))
                .push(text(if *is_blocked { "Blocked" } else { "Allowed" }).size(12))
                .push(toggler(!is_blocked).on_toggle(move |enabled| {
                    on_app_filter_toggle_clone(app_name_clone.clone(), enabled)
                }))
                .spacing(8.0)
                .align_y(cosmic::iced::Alignment::Center);

            content = content.push(app_row);
        }
    }

    container(content).width(Length::Fill).into()
}

/// Create an urgency level button
fn urgency_button<'a, Message>(
    label: &'a str,
    _is_selected: bool,
    on_press: impl Fn(u8) -> Message + 'a,
    level: u8,
) -> Element<'a, Message>
where
    Message: Clone + 'a + 'static,
{
    let btn = button_standard(label)
        .on_press(on_press(level))
        .padding([6, 12]);

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
