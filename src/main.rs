// COSMIC Notification Applet
// Main entry point

use cosmic::iced::Task;
use cosmic::{Application, Element};
use cosmic_applet_notifications::{config, dbus, manager, ui};

/// Main application state
pub struct NotificationApplet {
    /// COSMIC application core
    core: cosmic::app::Core,

    /// Notification manager
    manager: manager::NotificationManager,

    /// Configuration helper
    config_helper: config::ConfigHelper,

    /// Current configuration
    config: config::AppletConfig,

    /// Current popup window ID
    popup_id: Option<cosmic::iced::window::Id>,
}

/// Messages that drive the application
#[derive(Debug, Clone)]
pub enum Message {
    /// Toggle the notification popup
    TogglePopup,

    /// Close the popup
    ClosePopup,

    /// A new notification was received from D-Bus
    NotificationReceived(dbus::Notification),

    /// Dismiss a notification by ID
    DismissNotification(u32),

    /// Update configuration
    UpdateConfig(config::AppletConfig),

    /// Open a URL from a notification
    OpenUrl(String),

    /// Invoke a notification action
    InvokeAction {
        notification_id: u32,
        action_key: String,
    },

    /// Toggle Do Not Disturb mode
    ToggleDND,

    /// Set minimum urgency level (0=Low, 1=Normal, 2=Critical)
    SetUrgencyLevel(u8),

    /// Toggle app filter (app_name, enabled)
    ToggleAppFilter(String, bool),

    /// Tick for periodic updates
    Tick,

    /// Keyboard event
    KeyboardEvent(cosmic::iced::keyboard::Event),
}

// Implement From<Notification> for Message to work with subscription
impl From<dbus::Notification> for Message {
    fn from(notification: dbus::Notification) -> Self {
        Message::NotificationReceived(notification)
    }
}

impl Application for NotificationApplet {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "com.system76.CosmicAppletNotifications";

    fn core(&self) -> &cosmic::app::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::app::Core {
        &mut self.core
    }

    fn init(
        core: cosmic::app::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        // Load configuration
        let config_helper = config::ConfigHelper::new();
        let config = config_helper.load();

        tracing::info!("Configuration loaded from {:?}", config_helper.path());
        tracing::debug!("Config: {:?}", config);

        // Initialize manager with history from disk and config settings
        let mut manager = if config.history_enabled {
            manager::NotificationManager::with_history(
                config.max_history_items,
                config.history_retention_days,
            )
        } else {
            manager::NotificationManager::new()
        };
        manager.set_do_not_disturb(config.do_not_disturb);
        manager.set_min_urgency_level(config.min_urgency_level);
        manager.load_app_filters(config.app_filters.clone());

        let app = NotificationApplet {
            core,
            manager,
            config_helper,
            config,
            popup_id: None,
        };

        (app, Task::none())
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        use cosmic::iced::window;

        match message {
            Message::TogglePopup => {
                if let Some(id) = self.popup_id.take() {
                    // Close existing popup
                    return cosmic::iced::platform_specific::shell::commands::popup::destroy_popup(
                        id,
                    );
                } else {
                    // Create new popup
                    let id = window::Id::unique();
                    self.popup_id = Some(id);

                    // Calculate position based on configuration
                    let panel_edge = ui::positioning::PanelEdge::detect();
                    let (anchor, gravity, offset) = ui::positioning::calculate_popup_position(
                        &self.config.popup_position,
                        panel_edge,
                    );

                    tracing::debug!(
                        "Popup position: mode={:?}, panel={:?}, anchor={:?}, gravity={:?}, offset={:?}",
                        self.config.popup_position.mode,
                        panel_edge,
                        anchor,
                        gravity,
                        offset
                    );

                    // Get default popup settings
                    let mut popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        id,
                        Some((self.config.popup_width, self.config.popup_height)),
                        None,
                        None,
                    );

                    // Override positioner with calculated values
                    popup_settings.positioner.anchor = anchor;
                    popup_settings.positioner.gravity = gravity;
                    popup_settings.positioner.offset = offset;

                    return cosmic::iced::platform_specific::shell::commands::popup::get_popup(
                        popup_settings,
                    );
                }
            }

            Message::ClosePopup => {
                if let Some(id) = self.popup_id.take() {
                    return cosmic::iced::platform_specific::shell::commands::popup::destroy_popup(
                        id,
                    );
                }
            }

            Message::NotificationReceived(notification) => {
                // Add notification to manager
                let action = self.manager.add_notification(notification.clone());

                tracing::info!(
                    "Received notification from {}: {} (action: {:?})",
                    notification.app_name,
                    notification.summary,
                    action
                );
            }

            Message::DismissNotification(id) => {
                // Remove notification from manager
                if self.manager.remove_notification(id) {
                    tracing::debug!("Dismissed notification {}", id);
                } else {
                    tracing::warn!("Failed to dismiss notification {} (not found)", id);
                }
            }

            Message::UpdateConfig(new_config) => {
                // Validate and save config
                let mut config = new_config;
                if !config.validate() {
                    tracing::warn!("Invalid config, sanitizing");
                    config.sanitize();
                }

                // Apply config to manager
                self.manager.set_do_not_disturb(config.do_not_disturb);
                self.manager.set_min_urgency_level(config.min_urgency_level);
                self.manager.load_app_filters(config.app_filters.clone());

                // Save config
                if let Err(e) = self.config_helper.save(&config) {
                    tracing::error!("Failed to save config: {}", e);
                } else {
                    tracing::info!("Configuration saved");
                }

                self.config = config;
            }

            Message::OpenUrl(url) => {
                // Open URL using system handler (xdg-open)
                if let Err(e) = ui::url_parser::open_url(&url) {
                    tracing::error!("Failed to open URL {}: {}", url, e);
                } else {
                    tracing::info!("Opened URL: {}", url);
                }
            }

            Message::InvokeAction {
                notification_id,
                action_key,
            } => {
                // Send ActionInvoked signal to D-Bus
                let action_key_clone = action_key.clone();
                tokio::spawn(async move {
                    if let Err(e) =
                        dbus::send_action_invoked(notification_id, &action_key_clone).await
                    {
                        tracing::error!(
                            "Failed to send ActionInvoked for notification {}: {}",
                            notification_id,
                            e
                        );
                    }
                });

                tracing::info!(
                    "Action '{}' invoked for notification {}",
                    action_key,
                    notification_id
                );
            }

            Message::ToggleDND => {
                // Toggle Do Not Disturb mode
                self.config.do_not_disturb = !self.config.do_not_disturb;
                self.manager.set_do_not_disturb(self.config.do_not_disturb);

                // Save config
                if let Err(e) = self.config_helper.save(&self.config) {
                    tracing::error!("Failed to save config: {}", e);
                } else {
                    tracing::info!(
                        "Do Not Disturb {}",
                        if self.config.do_not_disturb {
                            "enabled"
                        } else {
                            "disabled"
                        }
                    );
                }
            }

            Message::SetUrgencyLevel(level) => {
                // Set minimum urgency level
                self.config.min_urgency_level = level.min(2); // Clamp to 0-2
                self.manager
                    .set_min_urgency_level(self.config.min_urgency_level);

                // Save config
                if let Err(e) = self.config_helper.save(&self.config) {
                    tracing::error!("Failed to save config: {}", e);
                } else {
                    tracing::info!(
                        "Minimum urgency level set to {}",
                        self.config.min_urgency_level
                    );
                }
            }

            Message::ToggleAppFilter(app_name, enabled) => {
                // Update app filter (enabled = show, !enabled = block)
                self.config.app_filters.insert(app_name.clone(), !enabled);
                self.manager.set_app_filter(app_name.clone(), enabled);

                // Save config
                if let Err(e) = self.config_helper.save(&self.config) {
                    tracing::error!("Failed to save config: {}", e);
                } else {
                    tracing::info!(
                        "App filter for '{}' set to {}",
                        app_name,
                        if enabled { "allowed" } else { "blocked" }
                    );
                }
            }

            Message::KeyboardEvent(event) => {
                use cosmic::iced::keyboard::{Event as KeyEvent, Key};

                match event {
                    KeyEvent::KeyPressed { key, modifiers, .. } => {
                        match key {
                            // Escape key closes popup
                            Key::Named(cosmic::iced::keyboard::key::Named::Escape) => {
                                if self.popup_id.is_some() {
                                    return self.update(Message::ClosePopup);
                                }
                            }

                            // Ctrl+D toggles Do Not Disturb
                            Key::Character(c) if c.as_str() == "d" && modifiers.control() => {
                                return self.update(Message::ToggleDND);
                            }

                            // Ctrl+1/2/3 for urgency levels
                            Key::Character(c) if c.as_str() == "1" && modifiers.control() => {
                                return self.update(Message::SetUrgencyLevel(0));
                            }
                            Key::Character(c) if c.as_str() == "2" && modifiers.control() => {
                                return self.update(Message::SetUrgencyLevel(1));
                            }
                            Key::Character(c) if c.as_str() == "3" && modifiers.control() => {
                                return self.update(Message::SetUrgencyLevel(2));
                            }

                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            Message::Tick => {
                // Check for expired notifications and remove them
                let expired_ids = self.manager.get_expired_notifications();

                for id in expired_ids {
                    self.manager.remove_notification(id);
                    tracing::debug!("Removed expired notification {}", id);
                }

                // Cleanup old notifications from history based on config
                if self.config.history_enabled {
                    let removed = self.manager.cleanup_history(
                        self.config.max_history_items,
                        self.config.history_retention_days,
                    );

                    if removed > 0 {
                        tracing::debug!("Cleaned up {} old notifications from history", removed);
                    }

                    // Save history to disk periodically
                    if let Err(e) = self.manager.save_history() {
                        tracing::error!("Failed to save notification history: {}", e);
                    } else {
                        tracing::trace!("Saved notification history to disk");
                    }
                }
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<Self::Message> {
        // Panel icon with notification count badge
        // TODO: Add notification count badge overlay when layer_container API is stable
        self.core
            .applet
            .icon_button("notification-symbolic")
            .on_press_down(Message::TogglePopup)
            .into()
    }

    fn view_window(&self, id: cosmic::iced::window::Id) -> Element<Self::Message> {
        use cosmic::widget::{column, divider, text};

        if Some(id) == self.popup_id {
            // Get active notifications from manager
            let notifications = self.manager.get_active_notifications();

            // Create notification list view with clickable URLs and action buttons
            let notification_list = ui::widgets::notification_list(
                notifications,
                |id| Message::DismissNotification(id),
                |url| Message::OpenUrl(url),
                |notification_id, action_key| Message::InvokeAction {
                    notification_id,
                    action_key,
                },
            );

            // Create filter settings view
            let filter_settings = ui::widgets::filter_settings(
                &self.config,
                Message::ToggleDND,
                |level| Message::SetUrgencyLevel(level),
                |app_name, enabled| Message::ToggleAppFilter(app_name, enabled),
            );

            // Combine notification list and settings
            let content = column()
                .push(notification_list)
                .push(divider::horizontal::default())
                .push(filter_settings)
                .spacing(0.0);

            self.core.applet.popup_container(content).into()
        } else {
            // Other windows
            text("Unknown window").into()
        }
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }

    fn subscription(&self) -> cosmic::iced::Subscription<Self::Message> {
        use cosmic::iced::time;
        use std::time::Duration;

        // Combine D-Bus notifications, periodic tick, and keyboard events
        cosmic::iced::Subscription::batch([
            // D-Bus notification listener
            dbus::subscribe(),
            // Periodic tick every 60 seconds to check for expired notifications
            time::every(Duration::from_secs(60)).map(|_| Message::Tick),
            // Keyboard events for shortcuts
            cosmic::iced::event::listen().map(|event| {
                if let cosmic::iced::Event::Keyboard(keyboard_event) = event {
                    Message::KeyboardEvent(keyboard_event)
                } else {
                    // Ignore non-keyboard events
                    Message::Tick // Use Tick as a no-op
                }
            }),
        ])
    }
}

fn main() -> cosmic::iced::Result {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    tracing::info!("Starting COSMIC Notification Applet");

    // Run the applet
    cosmic::applet::run::<NotificationApplet>(())
}
