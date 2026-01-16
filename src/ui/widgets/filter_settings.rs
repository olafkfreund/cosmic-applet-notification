// Filter settings widget
//
// Displays notification filter settings and controls for managing
// per-application filters, urgency levels, and Do Not Disturb mode.
// Follows COSMIC design patterns for consistent appearance.

use cosmic::iced::Length;
use cosmic::widget::{button, column, container, divider, row, text, toggler};
use cosmic::Element;

use crate::config::AppletConfig;
use crate::ui::theme::Spacing;

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
    let mut content = column().spacing(Spacing::s()).padding(Spacing::m());

    // Section header
    content = content.push(text::title3("Notification Settings"));

    // Keyboard shortcuts hint (using caption instead of too-small size 10)
    content = content.push(text::caption(
        "Shortcuts: Esc=Close, Ctrl+D=DND, Ctrl+1/2/3=Urgency",
    ));

    content = content.push(divider::horizontal::default());

    // Do Not Disturb toggle
    let dnd_row = row()
        .push(text::body("Do Not Disturb").width(Length::Fill))
        .push(toggler(config.do_not_disturb).on_toggle(move |_| on_toggle_dnd.clone()))
        .spacing(Spacing::xs())
        .align_y(cosmic::iced::Alignment::Center);

    content = content.push(dnd_row);

    // Urgency level selector
    content = content.push(text::title4("Minimum Urgency Level"));

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
        .spacing(Spacing::xs());

    content = content.push(urgency_buttons);

    // App filters section
    if !config.app_filters.is_empty() {
        content = content.push(divider::horizontal::default());

        content = content.push(text::title4("App Filters"));

        // Sort apps alphabetically for consistent display
        let mut apps: Vec<(&String, &bool)> = config.app_filters.iter().collect();
        apps.sort_by(|a, b| a.0.cmp(b.0));

        for (app_name, is_blocked) in apps {
            let app_name_clone = app_name.clone();
            let on_app_filter_toggle_clone = on_app_filter_toggle.clone();

            let status_text = if *is_blocked {
                text::body("Blocked")
            } else {
                text::body("Allowed")
            };

            let app_row = row()
                .push(text::body(app_name).width(Length::Fill))
                .push(status_text)
                .push(toggler(!is_blocked).on_toggle(move |enabled| {
                    on_app_filter_toggle_clone(app_name_clone.clone(), enabled)
                }))
                .spacing(Spacing::xs())
                .align_y(cosmic::iced::Alignment::Center);

            content = content.push(app_row);
        }
    }

    container(content).width(Length::Fill).into()
}

/// Create an urgency level button
///
/// Uses suggested style for selected button, standard for unselected
fn urgency_button<'a, Message>(
    label: &'a str,
    is_selected: bool,
    on_press: impl Fn(u8) -> Message + 'a,
    level: u8,
) -> Element<'a, Message>
where
    Message: Clone + 'a + 'static,
{
    let btn = if is_selected {
        button::suggested(label)
            .on_press(on_press(level))
            .padding([Spacing::xxs(), Spacing::s()])
    } else {
        button::standard(label)
            .on_press(on_press(level))
            .padding([Spacing::xxs(), Spacing::s()])
    };

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
