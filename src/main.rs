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

    /// Index of currently selected notification for keyboard navigation
    /// None means no selection, Some(index) is the position in the active notifications list
    selected_notification_index: Option<usize>,

    /// Index of currently selected action within the selected notification
    /// Used for Tab key cycling through action buttons
    selected_action_index: Option<usize>,

    /// Animation states for notifications (notification_id -> animation)
    notification_animations: std::collections::HashMap<u32, ui::animation::NotificationAnimation>,

    /// Popup animation state
    popup_animation: Option<ui::animation::PopupAnimation>,

    /// Progress indicators for timed notifications
    progress_indicators: std::collections::HashMap<u32, ui::animation::ProgressIndicator>,

    /// Whether reduced motion is preferred (accessibility)
    prefers_reduced_motion: bool,
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

    /// Set position mode (Auto / Panel Relative)
    SetPositionMode(config::PositionMode),

    /// Set panel anchor point
    SetPanelAnchor(config::PanelAnchor),

    /// Set X offset for popup position
    SetOffsetX(i32),

    /// Set Y offset for popup position
    SetOffsetY(i32),

    /// Toggle snap-to-edge feature
    ToggleSnapToEdge,

    /// Set snap-to-edge threshold
    SetSnapThreshold(u32),

    /// Preview the current popup position
    PreviewPosition,

    // Keyboard Navigation Messages
    /// Navigate to previous notification (Up arrow)
    NavigateUp,

    /// Navigate to next notification (Down arrow)
    NavigateDown,

    /// Activate selected notification (Enter key)
    ActivateSelected,

    /// Dismiss selected notification (Delete key)
    DismissSelected,

    /// Clear notification selection
    ClearSelection,

    /// Cycle through action buttons in selected notification (Tab key)
    CycleActions,

    /// Invoke action by number key (1-9)
    InvokeQuickAction(u8),

    // Animation Messages
    /// Animation frame update (16ms ~ 60fps)
    AnimationFrame,

    /// Start appearing animation for notification
    StartAppearAnimation(u32),

    /// Start dismissing animation for notification
    StartDismissAnimation(u32),

    /// Complete notification dismissal after animation
    CompleteNotificationDismissal(u32),

    /// Update prefers-reduced-motion accessibility setting
    UpdatePrefersReducedMotion(bool),

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

// Helper methods for NotificationApplet
impl NotificationApplet {
    /// Clear both notification and action selection
    fn clear_selection(&mut self) {
        self.selected_notification_index = None;
        self.selected_action_index = None;
    }

    /// Clear only action selection (when changing notifications)
    fn clear_action_selection(&mut self) {
        self.selected_action_index = None;
    }

    /// Clear selection if notification list is empty
    /// Returns true if list was empty
    fn clear_selection_if_no_notifications(&mut self) -> bool {
        if self.manager.get_active_notifications().is_empty() {
            self.clear_selection();
            true
        } else {
            false
        }
    }

    /// Validate and fix selection indices after notifications change
    /// Call this after removing notifications to keep selection in bounds
    fn validate_selection(&mut self) {
        let notification_count = self.manager.get_active_notifications().len();

        if notification_count == 0 {
            self.clear_selection();
            return;
        }

        // Fix out-of-bounds notification selection
        if let Some(idx) = self.selected_notification_index {
            if idx >= notification_count {
                self.selected_notification_index = Some(notification_count - 1);
                self.clear_action_selection();
            }
        }

        // Fix out-of-bounds action selection
        if let Some(notif_idx) = self.selected_notification_index {
            if let Some(action_idx) = self.selected_action_index {
                let active_notifications = self.manager.get_active_notifications();
                if let Some(notification) = active_notifications.get(notif_idx) {
                    if action_idx >= notification.actions.len() {
                        self.clear_action_selection();
                    }
                }
            }
        }
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
            selected_notification_index: None,
            selected_action_index: None,
            notification_animations: std::collections::HashMap::new(),
            popup_animation: None,
            progress_indicators: std::collections::HashMap::new(),
            prefers_reduced_motion: false, // Will be detected asynchronously
        };

        // Detect prefers-reduced-motion accessibility setting on startup
        let detect_task = Task::future(async {
            let prefers_reduced =
                cosmic_applet_notifications::accessibility::detect_prefers_reduced_motion().await;
            cosmic::Action::App(Message::UpdatePrefersReducedMotion(prefers_reduced))
        });

        (app, detect_task)
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
                    // Clear selection when closing popup
                    self.clear_selection();
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

                // Start appear animation if enabled (respect accessibility preferences)
                if self.config.animations.enabled
                    && self.config.animations.notification_appear
                    && !self.prefers_reduced_motion
                {
                    return self.update(Message::StartAppearAnimation(notification.id));
                }

                // Create progress indicator for timed notifications
                if self.config.animations.show_progress && notification.expire_timeout > 0 {
                    let indicator = ui::animation::ProgressIndicator::new(
                        notification.id,
                        notification.expire_timeout as i64,
                    );
                    self.progress_indicators.insert(notification.id, indicator);
                }
            }

            Message::DismissNotification(id) => {
                // Start dismiss animation if enabled (respect accessibility preferences)
                if self.config.animations.enabled
                    && self.config.animations.notification_dismiss
                    && !self.prefers_reduced_motion
                {
                    return self.update(Message::StartDismissAnimation(id));
                } else {
                    // Immediate dismissal without animation
                    return self.update(Message::CompleteNotificationDismissal(id));
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

            Message::SetPositionMode(mode) => {
                // Update position mode
                self.config.popup_position.mode = mode;

                // Save config
                if let Err(e) = self.config_helper.save(&self.config) {
                    tracing::error!("Failed to save config: {}", e);
                } else {
                    tracing::info!("Position mode set to {:?}", mode);
                }
            }

            Message::SetPanelAnchor(anchor) => {
                // Update panel anchor
                self.config.popup_position.anchor = anchor;

                // Save config
                if let Err(e) = self.config_helper.save(&self.config) {
                    tracing::error!("Failed to save config: {}", e);
                } else {
                    tracing::info!("Panel anchor set to {:?}", anchor);
                }
            }

            Message::SetOffsetX(x) => {
                // Update X offset
                self.config.popup_position.offset_x = x;

                // Save config
                if let Err(e) = self.config_helper.save(&self.config) {
                    tracing::error!("Failed to save config: {}", e);
                } else {
                    tracing::debug!("Offset X set to {}", x);
                }
            }

            Message::SetOffsetY(y) => {
                // Update Y offset
                self.config.popup_position.offset_y = y;

                // Save config
                if let Err(e) = self.config_helper.save(&self.config) {
                    tracing::error!("Failed to save config: {}", e);
                } else {
                    tracing::debug!("Offset Y set to {}", y);
                }
            }

            Message::ToggleSnapToEdge => {
                // Toggle snap-to-edge
                self.config.popup_position.snap_to_edge = !self.config.popup_position.snap_to_edge;

                // Save config
                if let Err(e) = self.config_helper.save(&self.config) {
                    tracing::error!("Failed to save config: {}", e);
                } else {
                    tracing::info!(
                        "Snap to edge {}",
                        if self.config.popup_position.snap_to_edge {
                            "enabled"
                        } else {
                            "disabled"
                        }
                    );
                }
            }

            Message::SetSnapThreshold(threshold) => {
                // Update snap threshold
                self.config.popup_position.snap_threshold = threshold;

                // Save config
                if let Err(e) = self.config_helper.save(&self.config) {
                    tracing::error!("Failed to save config: {}", e);
                } else {
                    tracing::debug!("Snap threshold set to {}", threshold);
                }
            }

            Message::PreviewPosition => {
                // Close current popup if open, then reopen to show new position
                if let Some(id) = self.popup_id.take() {
                    tracing::info!("Closing popup for position preview");
                    return cosmic::iced::platform_specific::shell::commands::popup::destroy_popup(
                        id,
                    );
                } else {
                    // Open popup to preview position
                    tracing::info!("Opening popup to preview position");
                    return self.update(Message::TogglePopup);
                }
            }

            Message::NavigateUp => {
                // Move selection up in notification list
                if self.clear_selection_if_no_notifications() {
                    return Task::none();
                }

                let active_notifications = self.manager.get_active_notifications();
                self.selected_notification_index = Some(match self.selected_notification_index {
                    None => active_notifications.len() - 1, // Start from bottom
                    Some(0) => active_notifications.len() - 1, // Wrap to bottom
                    Some(idx) => idx - 1,                   // Move up
                });

                // Clear action selection when changing notifications
                self.clear_action_selection();

                tracing::debug!(
                    "Navigate up: selected index = {:?}",
                    self.selected_notification_index
                );
            }

            Message::NavigateDown => {
                // Move selection down in notification list
                if self.clear_selection_if_no_notifications() {
                    return Task::none();
                }

                let active_notifications = self.manager.get_active_notifications();
                self.selected_notification_index = Some(match self.selected_notification_index {
                    None => 0,                                               // Start from top
                    Some(idx) if idx + 1 >= active_notifications.len() => 0, // Wrap to top
                    Some(idx) => idx + 1,                                    // Move down
                });

                // Clear action selection when changing notifications
                self.clear_action_selection();

                tracing::debug!(
                    "Navigate down: selected index = {:?}",
                    self.selected_notification_index
                );
            }

            Message::ActivateSelected => {
                // Activate the selected notification (open URL or invoke first action)
                if let Some(idx) = self.selected_notification_index {
                    let active_notifications = self.manager.get_active_notifications();
                    if let Some(notification) = active_notifications.get(idx) {
                        // First try to open a URL if present in body
                        if let Some(url) = ui::url_parser::extract_first_url(&notification.body) {
                            tracing::info!(
                                "Activating selected notification {}: opening URL {}",
                                notification.id,
                                url
                            );
                            return self.update(Message::OpenUrl(url));
                        }

                        // If no URL, try to invoke first action
                        if !notification.actions.is_empty() {
                            let action_key = notification.actions[0].key.clone();
                            tracing::info!(
                                "Activating selected notification {}: invoking action {}",
                                notification.id,
                                action_key
                            );
                            return self.update(Message::InvokeAction {
                                notification_id: notification.id,
                                action_key,
                            });
                        }

                        tracing::debug!(
                            "Selected notification {} has no URL or actions to activate",
                            notification.id
                        );
                    }
                }
            }

            Message::DismissSelected => {
                // Dismiss the selected notification
                if let Some(idx) = self.selected_notification_index {
                    let active_notifications = self.manager.get_active_notifications();
                    if let Some(notification) = active_notifications.get(idx) {
                        let notification_id = notification.id;
                        tracing::info!("Dismissing selected notification {}", notification_id);

                        // Clear selection before dismissing
                        self.clear_selection();

                        return self.update(Message::DismissNotification(notification_id));
                    }
                }
            }

            Message::ClearSelection => {
                // Clear notification selection
                tracing::debug!("Clearing notification selection");
                self.clear_selection();
            }

            Message::CycleActions => {
                // Cycle through action buttons in selected notification
                if let Some(notif_idx) = self.selected_notification_index {
                    let active_notifications = self.manager.get_active_notifications();
                    if let Some(notification) = active_notifications.get(notif_idx) {
                        let action_count = notification.actions.len();

                        if action_count == 0 {
                            // No actions to cycle
                            self.selected_action_index = None;
                            return Task::none();
                        }

                        // Cycle to next action
                        self.selected_action_index = Some(match self.selected_action_index {
                            None => 0,                                 // Start from first action
                            Some(idx) if idx + 1 >= action_count => 0, // Wrap to first
                            Some(idx) => idx + 1,                      // Move to next
                        });

                        tracing::debug!(
                            "Cycle actions: selected action index = {:?} (of {})",
                            self.selected_action_index,
                            action_count
                        );
                    }
                }
            }

            Message::InvokeQuickAction(action_number) => {
                // Invoke action by number key (1-9)
                if let Some(notif_idx) = self.selected_notification_index {
                    let active_notifications = self.manager.get_active_notifications();
                    if let Some(notification) = active_notifications.get(notif_idx) {
                        // Convert 1-based action number to 0-based index
                        let action_idx = (action_number - 1) as usize;

                        if action_idx < notification.actions.len() {
                            let action_key = notification.actions[action_idx].key.clone();
                            tracing::info!(
                                "Quick action {}: invoking action {} for notification {}",
                                action_number,
                                action_key,
                                notification.id
                            );
                            return self.update(Message::InvokeAction {
                                notification_id: notification.id,
                                action_key,
                            });
                        } else {
                            tracing::debug!(
                                "Quick action {}: no action at index {} for notification {}",
                                action_number,
                                action_idx,
                                notification.id
                            );
                        }
                    }
                }
            }

            Message::KeyboardEvent(event) => {
                use cosmic::iced::keyboard::{Event as KeyEvent, Key};

                if let KeyEvent::KeyPressed { key, modifiers, .. } = event {
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

                        // Arrow keys for navigation
                        Key::Named(cosmic::iced::keyboard::key::Named::ArrowUp) => {
                            if self.popup_id.is_some() {
                                return self.update(Message::NavigateUp);
                            }
                        }
                        Key::Named(cosmic::iced::keyboard::key::Named::ArrowDown) => {
                            if self.popup_id.is_some() {
                                return self.update(Message::NavigateDown);
                            }
                        }

                        // Enter key activates selected notification
                        Key::Named(cosmic::iced::keyboard::key::Named::Enter) => {
                            if self.popup_id.is_some() {
                                return self.update(Message::ActivateSelected);
                            }
                        }

                        // Delete key dismisses selected notification
                        Key::Named(cosmic::iced::keyboard::key::Named::Delete) => {
                            if self.popup_id.is_some() {
                                return self.update(Message::DismissSelected);
                            }
                        }

                        // Tab key cycles through actions
                        Key::Named(cosmic::iced::keyboard::key::Named::Tab) => {
                            if self.popup_id.is_some() && !modifiers.shift() {
                                return self.update(Message::CycleActions);
                            }
                        }

                        // Number keys (1-9) for quick action invocation
                        Key::Character(c) if !modifiers.control() && !modifiers.alt() => {
                            if self.popup_id.is_some() {
                                if let Some(digit) = c.chars().next().and_then(|ch| ch.to_digit(10)) {
                                    if (1..=9).contains(&digit) {
                                        return self.update(Message::InvokeQuickAction(digit as u8));
                                    }
                                }
                            }
                        }

                        _ => {}
                    }
                }
            }

            Message::Tick => {
                // Check for expired notifications and remove them
                let expired_ids = self.manager.get_expired_notifications();

                for id in expired_ids {
                    self.manager.remove_notification(id);
                    tracing::debug!("Removed expired notification {}", id);
                }

                // Validate selection after removing notifications
                self.validate_selection();

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

            // Animation messages (Phase 4B)
            Message::AnimationFrame => {
                // Check all notification animations for completion
                let mut completed_animations = Vec::new();

                for (notification_id, animation) in &self.notification_animations {
                    // Check if animation is complete (animations track time internally)
                    if animation.is_complete() {
                        completed_animations.push(*notification_id);
                    }
                }

                // Update progress indicators (remove expired ones)
                self.progress_indicators
                    .retain(|_, indicator| !indicator.is_expired());

                // Remove completed animations and trigger completion handlers
                for notification_id in completed_animations {
                    if let Some(animation) = self.notification_animations.remove(&notification_id) {
                        // If this was a dismiss animation, complete the dismissal
                        if matches!(
                            animation.animation_type,
                            ui::animation::NotificationAnimationType::Dismissing
                        ) {
                            return self
                                .update(Message::CompleteNotificationDismissal(notification_id));
                        }
                    }
                }
            }

            Message::StartAppearAnimation(notification_id) => {
                // Check if animations are enabled
                if !self.config.animations.enabled || !self.config.animations.notification_appear {
                    return Task::none();
                }

                // Create appear animation
                let animation = ui::animation::NotificationAnimation::appearing(
                    notification_id,
                    ui::animation::AnimationDuration::NORMAL,
                );

                self.notification_animations
                    .insert(notification_id, animation);

                tracing::debug!(
                    "Started appear animation for notification {}",
                    notification_id
                );
            }

            Message::StartDismissAnimation(notification_id) => {
                // Check if animations are enabled
                if !self.config.animations.enabled || !self.config.animations.notification_dismiss {
                    // Skip animation and dismiss immediately
                    return self.update(Message::CompleteNotificationDismissal(notification_id));
                }

                // Create dismiss animation
                let animation = ui::animation::NotificationAnimation::dismissing(
                    notification_id,
                    ui::animation::AnimationDuration::FAST,
                );

                self.notification_animations
                    .insert(notification_id, animation);

                tracing::debug!(
                    "Started dismiss animation for notification {}",
                    notification_id
                );
            }

            Message::CompleteNotificationDismissal(notification_id) => {
                // Remove the notification from manager
                if self.manager.remove_notification(notification_id) {
                    tracing::debug!("Completed dismissal of notification {}", notification_id);
                } else {
                    tracing::warn!(
                        "Failed to complete dismissal of notification {} (not found)",
                        notification_id
                    );
                }

                // Clean up animation state
                self.notification_animations.remove(&notification_id);
                self.progress_indicators.remove(&notification_id);

                // Validate selection after removing notification
                self.validate_selection();
            }

            Message::UpdatePrefersReducedMotion(prefers_reduced) => {
                self.prefers_reduced_motion = prefers_reduced;

                if prefers_reduced {
                    tracing::info!("Accessibility: prefers-reduced-motion enabled - animations will be disabled");
                } else {
                    tracing::debug!("Accessibility: prefers-reduced-motion disabled - animations will respect config");
                }
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        // Panel icon with notification count badge
        // TODO: Add notification count badge overlay when layer_container API is stable
        self.core
            .applet
            .icon_button("notification-symbolic")
            .on_press_down(Message::TogglePopup)
            .into()
    }

    fn view_window(&self, id: cosmic::iced::window::Id) -> Element<'_, Self::Message> {
        use cosmic::widget::{column, divider, text};

        if Some(id) == self.popup_id {
            // Get active notifications from manager
            let notifications = self.manager.get_active_notifications();

            // Create notification list view with clickable URLs and action buttons
            let notification_list = ui::widgets::notification_list(
                notifications,
                &self.notification_animations,
                self.selected_notification_index,
                self.selected_action_index,
                Message::DismissNotification,
                Message::OpenUrl,
                |notification_id, action_key| Message::InvokeAction {
                    notification_id,
                    action_key,
                },
            );

            // Create filter settings view
            let filter_settings = ui::widgets::filter_settings(
                &self.config,
                Message::ToggleDND,
                Message::SetUrgencyLevel,
                Message::ToggleAppFilter,
            );

            // Create position settings view
            let position_settings = ui::widgets::position_settings(
                &self.config.popup_position,
                Message::SetPositionMode,
                Message::SetPanelAnchor,
                Message::SetOffsetX,
                Message::SetOffsetY,
                Message::ToggleSnapToEdge,
                Message::SetSnapThreshold,
                Message::PreviewPosition,
            );

            // Combine notification list and settings
            let content = column()
                .push(notification_list)
                .push(divider::horizontal::default())
                .push(filter_settings)
                .push(divider::horizontal::default())
                .push(position_settings)
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

        let mut subscriptions = vec![
            // D-Bus notification listener
            dbus::subscribe(),
            // Periodic tick every 60 seconds to check for expired notifications
            time::every(Duration::from_secs(60)).map(|_| Message::Tick),
            // Keyboard events for shortcuts
            cosmic::iced::event::listen_with(|event, _status, _window| {
                if let cosmic::iced::Event::Keyboard(keyboard_event) = event {
                    Some(Message::KeyboardEvent(keyboard_event))
                } else {
                    None
                }
            }),
        ];

        // Add animation frame subscription if animations are enabled and there are active animations
        // (respect accessibility preferences)
        if self.config.animations.enabled
            && !self.prefers_reduced_motion
            && (!self.notification_animations.is_empty()
                || self.popup_animation.is_some()
                || !self.progress_indicators.is_empty())
        {
            // 60fps animation updates
            subscriptions
                .push(time::every(Duration::from_millis(16)).map(|_| Message::AnimationFrame));
        }

        cosmic::iced::Subscription::batch(subscriptions)
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
