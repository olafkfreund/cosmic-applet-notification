# libcosmic Applet Skill: COSMIC Panel Applet Development

## Overview

This skill provides comprehensive knowledge about creating panel applets for COSMIC Desktop using libcosmic. Applets are lightweight applications embedded in the COSMIC panel that provide quick access to functionality.

## Core Concepts

### What is a COSMIC Applet?

A COSMIC applet is a specialized application that:
- Runs embedded in the COSMIC panel
- Has a small icon in the panel
- Can show popup windows
- Integrates with COSMIC's theming
- Uses Wayland layer shell for popups

### Applet vs Application

| Feature | Application | Applet |
|---------|------------|---------|
| Entry point | `cosmic::app::run()` | `cosmic::applet::run()` |
| Window decoration | Yes | No (transparent) |
| Window management | Standard | Panel-managed |
| Features flag | `app` | `applet` |

## Setup

### Cargo.toml Configuration

```toml
[package]
name = "cosmic-notification-applet"
version = "0.1.0"
edition = "2021"

[dependencies]
libcosmic = { git = "https://github.com/pop-os/libcosmic", features = ["applet"] }
tokio = { version = "1", features = ["full"] }
zbus = { version = "4", features = ["tokio"] }

[profile.release]
opt-level = 3
lto = true
```

### Desktop Entry

The applet requires a special desktop entry:

```desktop
[Desktop Entry]
Name=Notification Applet
Name[hu]=Értesítési Kisalkalmazás
Type=Application
Exec=cosmic-applet-notifications
Terminal=false
Categories=COSMIC;
Keywords=COSMIC;Notifications;
Icon=com.system76.CosmicAppletNotifications
StartupNotify=false
NoDisplay=true
X-CosmicApplet=true
X-CosmicAppletHoverPopup=Auto
X-CosmicAppletPriority=5
```

**Key Fields**:
- `X-CosmicApplet=true` - Marks this as a COSMIC applet
- `X-CosmicHoverPopup=Auto` - Enable hover popup behavior
- `NoDisplay=true` - Don't show in application launchers
- `Categories=COSMIC;` - COSMIC-specific category

### Applet Icon
```rust
fn view(&self) -> Element<Message> {
    let icon_name = if self.has_notifications {
        "notification-symbolic"
    } else {
        "notification-disabled-symbolic"
    };
    
    let icon_btn = self
        .core
        .applet
        .icon_button(icon_name)
        .on_press_down(Message::TogglePopup);
    
    // Add badge if there are unread notifications
    if self.unread_count > 0 {
        // Overlay badge on icon
        notification_badge(icon_button, self.unread_count)
    } else {
        icon_button.into()
    }
}
```

### Popup Window Management

```rust
#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    ClosePopup,
    // ... other messages
}

fn update(&mut self, message: Message) -> Command<Message> {
    match message {
        Message::TogglePopup => {
            if let Some(id) = self.popup_id.take() {
                // Close existing popup
                return destroy_popup(id);
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
                
                return get_popup(popup_settings);
            }
        }
        
        Message::ClosePopup => {
            if let Some(id) = self.popup_id.take() {
                return destroy_popup(id);
            }
        }
    }
    Command::none()
}
```

## Best Practices

### 1. Always use `applet::run` instead of `app::run`
```rust
fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<NotificationApplet>(false, ())
}
```

### 2. Use `autosize_window` for panels
```rust
fn view(&self) -> Element<Message> {
    self.core
        .applet
        .icon_button(&self.icon_name)
        .on_press(Message::TogglePopup)
        .into()
}
```

### 3. Always use applet style
```rust
fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
    Some(cosmic::applet::style())
}
```

### 4. Proper popup management
```rust
fn update(&mut self, message: Message) -> Command<Message> {
    match message {
        Message::TogglePopup => {
            if let Some(popup_id) = self.popup_id.take() {
                // Close existing popup
                return destroy_popup(popup_id);
            } else {
                // Create new popup
                let new_id = window::Id::unique();
                self.popup_id = Some(new_id);
                
                let popup_settings = self.core.applet.get_popup_settings(
                    self.core.main_window_id().unwrap(),
                    new_id,
                    None, // Let panel determine size
                    None,
                    None,
                );
                
                return get_popup(popup_settings);
            }
        }
        Command::none()
    }
}
```

## Best Practices

1. **Use applet-specific features**
   ```toml
   [dependencies]
   libcosmic = { git = "...", features = ["applet"] }
   ```

2. **Keep panel icon simple**
   - Single icon with optional badge
   - Don't update too frequently
   - Use symbolic icons

3. **Popup window guidelines**
   - Max width: 400-500px
   - Max height: Fit screen with padding
   - Auto-size to content
   - Respect panel anchor position

4. **Handle theme changes**
   ```rust
   fn subscription(&self) -> Subscription<Message> {
       cosmic::theme::subscription().map(Message::ThemeChanged)
   }
   ```

5. **Proper cleanup**
   ```rust
   impl Drop for NotificationApplet {
       fn drop(&mut self) {
           // Clean up resources
           // Close D-Bus connections
           // Save state
       }
   }
   ```

## Common Pitfalls

❌ **Blocking the UI thread**
```rust
// Wrong - blocks UI
fn update(&mut self, message: Message) -> Command<Message> {
    std::thread::sleep(Duration::from_secs(1)); // BAD!
    Command::none()
}
```

✅ **Use Commands for async work**
```rust
fn update(&mut self, message: Message) -> Command<Message> {
    match message {
        Message::LoadData => {
            return Command::perform(
                async { load_data().await },
                Message::DataLoaded
            );
        }
        // ...
    }
}
```

## Reference

- [libcosmic Book](https://pop-os.github.io/libcosmic-book/)
- [libcosmic API Docs](https://pop-os.github.io/libcosmic/)
- [cosmic-applet-template](https://github.com/pop-os/cosmic-applet-template)
- [COSMIC Applets Source](https://github.com/pop-os/cosmic-applets)

---

**Last Updated**: 2025-01-13
