// COSMIC Notification Applet
// Main entry point

use cosmic::app::Settings;
use cosmic::{Application, Element};
use cosmic_applet_notifications::{config, dbus, manager, ui};

/// Main application state
pub struct NotificationApplet {
    /// COSMIC application core
    core: cosmic::app::Core,

    /// Notification manager
    manager: manager::NotificationManager,

    /// Current popup window ID
    popup_id: Option<cosmic::iced::window::Id>,
    // TODO: Add configuration
    // config: config::AppletConfig,
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

    /// Tick for periodic updates
    Tick,
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
    ) -> (Self, cosmic::iced::Command<Self::Message>) {
        let app = NotificationApplet {
            core,
            manager: manager::NotificationManager::new(),
            popup_id: None,
        };

        // TODO: Load configuration

        (app, cosmic::iced::Command::none())
    }

    fn update(&mut self, message: Self::Message) -> cosmic::iced::Command<Self::Message> {
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

                    let popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        id,
                        Some((400, 600)), // width, height
                        None,
                        None,
                    );

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

                // TODO: Update UI to show notification count in panel icon (Issue #6)
            }

            Message::Tick => {
                // Check for expired notifications and remove them
                let expired_ids = self.manager.get_expired_notifications();

                for id in expired_ids {
                    self.manager.remove_notification(id);
                    tracing::debug!("Removed expired notification {}", id);
                }

                // TODO: Update UI if needed
            }
        }

        cosmic::iced::Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        // Panel icon
        self.core
            .applet
            .icon_button("notification-symbolic")
            .on_press_down(Message::TogglePopup)
            .into()
    }

    fn view_window(&self, id: cosmic::iced::window::Id) -> Element<Self::Message> {
        use cosmic::widget::{container, text};

        if Some(id) == self.popup_id {
            // Popup window content
            let content = container(text("Notification Applet").size(16)).padding(20);

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
        // Subscribe to D-Bus notifications using the subscription pattern
        // This replaces the need for manual threading and channels
        dbus::subscribe()
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
    cosmic::applet::run::<NotificationApplet>(false, ())
}
