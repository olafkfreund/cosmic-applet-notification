# COSMIC Notification Applet - User Guide

**Version**: 0.1.0
**Last Updated**: 2026-01-15

## Table of Contents

- [Introduction](#introduction)
- [Getting Started](#getting-started)
- [Basic Usage](#basic-usage)
- [Features](#features)
  - [Notification Display](#notification-display)
  - [Keyboard Navigation](#keyboard-navigation)
  - [Custom Positioning](#custom-positioning)
  - [Filtering & Do Not Disturb](#filtering--do-not-disturb)
  - [Animations](#animations)
  - [Accessibility](#accessibility)
- [Configuration](#configuration)
- [Keyboard Shortcuts Reference](#keyboard-shortcuts-reference)
- [Troubleshooting](#troubleshooting)
- [FAQ](#faq)

## Introduction

The COSMIC Notification Applet is a custom notification manager for the COSMIC Desktop Environment that enhances the notification experience with powerful features like keyboard navigation, custom positioning, smart filtering, and smooth animations.

### Key Features

- 🎯 **Custom Positioning** - Place notifications exactly where you want them
- ⌨️ **Full Keyboard Control** - Navigate and manage notifications without a mouse
- 🎨 **Smooth Animations** - Elegant fade-in/fade-out effects (with accessibility support)
- 🔕 **Do Not Disturb Mode** - Silence notifications (critical ones still get through)
- 🔍 **Smart Filtering** - Filter by app or urgency level
- 🔗 **Clickable URLs** - Open links directly from notifications
- 🎬 **Action Buttons** - Interact with notifications (reply, dismiss, etc.)
- 📜 **Notification History** - Review past notifications
- ♿ **Accessibility** - Respects system reduced-motion preferences

## Getting Started

### Installation

#### On NixOS (Recommended)

```bash
# Clone the repository
git clone https://github.com/yourusername/cosmic-applet-notification.git
cd cosmic-applet-notification

# Build and run in development mode
nix develop
just build
just run
```

#### Installation to System

```bash
# Build the release version
nix build

# Copy to COSMIC applets directory (adjust path as needed)
cp result/bin/cosmic-applet-notifications ~/.local/share/cosmic/applets/
```

### First Launch

After installation, the applet should appear in your COSMIC panel automatically. If not:

1. Restart the COSMIC panel: `cosmic-panel --reload`
2. Check for the bell icon (🔔) in your panel
3. Click the icon to open the notification popup

## Basic Usage

### Viewing Notifications

1. **Click the panel icon** - Opens the notification popup
2. **Notifications display** - Most recent at the top
3. **Scroll** - Use mouse wheel or touchpad to scroll through notifications
4. **Dismiss** - Click the ✕ button on any notification to remove it

### Interacting with Notifications

**Clickable URLs**: URLs in notification text are automatically detected and clickable.

**Action Buttons**: If a notification has action buttons (like "Reply", "Dismiss"), they appear at the bottom of the notification card.

**Opening URLs**: Click on blue underlined links to open them in your default browser.

## Features

### Notification Display

#### Visual Layout

Each notification shows:
- **App Icon** (if provided)
- **App Name** - The application that sent the notification
- **Timestamp** - Relative time (e.g., "2m ago", "1h ago")
- **Summary** - Bold title text
- **Body** - Detailed message with clickable URLs
- **Action Buttons** - Optional buttons for interaction
- **Dismiss Button** (✕) - Close the notification

#### Empty State

When no notifications are present, you'll see:
```
No Notifications
You're all caught up!
```

### Keyboard Navigation

Full keyboard control for power users and accessibility.

#### Navigation Keys

**Arrow Keys**:
- `↑ Up Arrow` - Navigate to previous notification (wraps to bottom)
- `↓ Down Arrow` - Navigate to next notification (wraps to top)

The selected notification is highlighted with an accent-colored border.

#### Action Keys

- `Enter` - Activate selected notification
  - If the notification has a URL, opens the URL
  - If the notification has actions, invokes the first action
- `Delete` - Dismiss the selected notification
- `Tab` - Cycle through action buttons (for multi-action notifications)
- `1-9` - Quick action invocation
  - Press `1` to invoke the first action
  - Press `2` to invoke the second action
  - etc. (up to 9 actions)

#### Global Shortcuts

These work even when the popup isn't focused:

- `Escape` - Close the notification popup
- `Ctrl+D` - Toggle Do Not Disturb mode
- `Ctrl+1` - Show all notifications (no urgency filter)
- `Ctrl+2` - Show normal and critical notifications only
- `Ctrl+3` - Show critical notifications only

### Custom Positioning

Control exactly where the notification popup appears.

#### Position Modes

1. **Auto Mode** (default)
   - Popup appears near the applet icon
   - Automatically adjusts for panel edge

2. **Panel Relative Mode**
   - Position popup relative to panel edge
   - Customizable anchor point and offsets

#### Anchor Points

When in Panel Relative mode, choose where the popup anchors:

- **Start** - Left (horizontal panel) or Top (vertical panel)
- **Center** - Middle of the panel
- **End** - Right (horizontal panel) or Bottom (vertical panel)
- **Applet Icon** - At the applet's location (like Auto mode)

#### Offsets

Fine-tune the position with pixel-perfect offsets:

- **X Offset**: -500px to +500px (horizontal adjustment)
- **Y Offset**: -500px to +500px (vertical adjustment)

Positive values move right/down, negative values move left/up.

#### Snap to Edge

**Feature**: Automatically snap popup to screen edge when close.

**How it works**:
- If popup is within the snap threshold (default: 20px) of a screen edge
- The popup will snap exactly to that edge
- Makes perfect edge alignment easy

**Configuration**:
- **Snap Threshold**: 5-100px (how close to trigger snap)
- **Toggle**: Enable/disable snap-to-edge

#### Position Preview

**Testing your settings**:
1. Adjust position settings
2. Click "Preview Position" button
3. Popup appears at configured position
4. Auto-closes after 3 seconds

### Filtering & Do Not Disturb

#### Do Not Disturb Mode

**Activation**:
- Click DND toggle in settings
- Or press `Ctrl+D` keyboard shortcut

**Behavior**:
- Suppresses Low and Normal urgency notifications
- Critical notifications still appear (system warnings, errors)
- Icon shows DND status (🔕)

#### Urgency Level Filtering

Three filter levels available:

1. **All Notifications** (Level 0)
   - Shows Low, Normal, and Critical
   - Default setting

2. **Normal and Critical** (Level 1)
   - Hides Low urgency notifications
   - Shows important notifications

3. **Critical Only** (Level 2)
   - Only system-critical notifications
   - Maximum focus mode

**Keyboard Shortcuts**:
- `Ctrl+1` - All notifications
- `Ctrl+2` - Normal and Critical
- `Ctrl+3` - Critical only

#### Per-Application Filtering

**Block specific apps**:
1. Open notification settings
2. Find the app in the filter list
3. Toggle the app off
4. That app's notifications will be hidden

**Use cases**:
- Block noisy apps during work
- Hide non-essential notifications
- Custom notification experience per app

### Animations

Smooth animations for a polished experience.

#### Animation Types

1. **Appear Animation** (300ms)
   - Fade in (opacity: 0 → 1)
   - Slide in (translate: -50px → 0)
   - Scale (0.95 → 1.0)
   - Easing: Cubic Out (smooth deceleration)

2. **Dismiss Animation** (200ms)
   - Fade out (opacity: 1 → 0)
   - Slide out (translate: 0 → -50px)
   - Scale (1.0 → 0.95)
   - Easing: Ease In (smooth acceleration)

#### Configuration

**Animation Settings** (in config file):
```ron
animations: AnimationConfig(
    enabled: true,                  // Master toggle
    notification_appear: true,      // Enable/disable appear animations
    notification_dismiss: true,     // Enable/disable dismiss animations
    show_progress: true,            // Show progress bars for timed notifications
),
```

**Note**: Visual transform rendering is currently limited by the iced GUI framework. Animation state is calculated correctly but visual effects may not be visible yet. Follow [Issue #15](https://github.com/yourusername/cosmic-applet-notification/issues/15) for updates.

### Accessibility

#### Reduced Motion Support

**What it is**: System-wide setting for users with motion sensitivity or vestibular disorders.

**How it works**:
- Applet automatically detects the `prefers-reduced-motion` setting
- Uses XDG Desktop Portal (cross-desktop standard)
- When enabled, all animations are instantly disabled
- No configuration needed - respects system preference

**Enabling Reduced Motion**:

On GNOME-based systems:
```bash
gsettings set org.gnome.desktop.interface enable-animations false
```

On other desktops: Check your system settings for "Reduce Animation" or "Reduced Motion" options.

**What happens**:
- Notifications appear instantly (no fade-in)
- Dismissals are immediate (no fade-out)
- 60fps animation subscription disabled (saves CPU)
- User experience remains fully functional

#### Keyboard Navigation

Full keyboard access ensures:
- Users who can't use a mouse can fully interact
- Power users can work more efficiently
- Screen reader compatibility (via COSMIC accessibility)

#### Visual Feedback

- **Selection highlight** - Accent-colored border on selected notification
- **Focus indicators** - Clear visual cues for keyboard focus
- **High contrast support** - Works with COSMIC high contrast themes

## Configuration

### Configuration File Location

The applet stores configuration at:
```
~/.config/cosmic/com.cosmic.applet.notifications/config.ron
```

### Configuration Options

#### General Settings

```ron
AppletConfig(
    // Do Not Disturb mode
    do_not_disturb: false,

    // Minimum urgency level (0=All, 1=Normal+, 2=Critical)
    min_urgency_level: 0,

    // Notification history
    history_enabled: true,
    max_history_items: 100,
    history_retention_days: 7,
)
```

#### Popup Position Settings

```ron
popup_position: PopupPosition(
    mode: Auto,  // or PanelRelative
    anchor: AppletIcon,  // Start, Center, End, or AppletIcon
    offset_x: 0,  // -500 to 500
    offset_y: 0,  // -500 to 500
    snap_to_edge: true,
    snap_threshold: 20,  // pixels
),
```

#### Animation Settings

```ron
animations: AnimationConfig(
    enabled: true,
    notification_appear: true,
    notification_dismiss: true,
    show_progress: true,
),
```

#### App Filters

```ron
app_filters: {
    "Spotify": false,  // Block Spotify notifications
    "Slack": true,     // Allow Slack notifications
    "Email": true,     // Allow Email notifications
},
```

### Editing Configuration

1. **Locate the file**: `~/.config/cosmic/com.cosmic.applet.notifications/config.ron`
2. **Edit with text editor**: `nano config.ron` or use your preferred editor
3. **Save changes**
4. **Restart applet**: Configuration reloads automatically or restart COSMIC panel

**Tip**: The config file uses RON (Rusty Object Notation) format, which is similar to JSON but more flexible.

## Keyboard Shortcuts Reference

### Navigation
| Shortcut | Action |
|----------|--------|
| `↑` Up Arrow | Previous notification (wraps to bottom) |
| `↓` Down Arrow | Next notification (wraps to top) |

### Actions
| Shortcut | Action |
|----------|--------|
| `Enter` | Activate selected notification (open URL or first action) |
| `Delete` | Dismiss selected notification |
| `Tab` | Cycle through action buttons |
| `1-9` | Quick action invocation (1 = first action, 2 = second, etc.) |

### Global
| Shortcut | Action |
|----------|--------|
| `Escape` | Close notification popup |
| `Ctrl+D` | Toggle Do Not Disturb |
| `Ctrl+1` | Show all notifications |
| `Ctrl+2` | Show normal + critical only |
| `Ctrl+3` | Show critical only |

## Troubleshooting

### Notifications Not Appearing

**Check the applet is running**:
```bash
ps aux | grep cosmic-applet-notifications
```

**Check D-Bus connection**:
```bash
# Monitor notifications
dbus-monitor "interface='org.freedesktop.Notifications'"

# Send test notification
notify-send "Test" "This is a test notification"
```

**Check logs**:
```bash
journalctl --user -u cosmic-panel -f | grep notification
```

### Keyboard Shortcuts Not Working

1. **Ensure popup is focused** - Click the popup window first
2. **Check for conflicting shortcuts** - Other apps may capture the same keys
3. **Restart COSMIC panel** - `cosmic-panel --reload`

### Animations Not Smooth

**Check if reduced motion is enabled**:
```bash
# On GNOME systems
gsettings get org.gnome.desktop.interface enable-animations
```

**Enable trace logging to see animation state**:
```bash
RUST_LOG=cosmic_applet_notifications=trace cosmic-applet-notifications
```

**Note**: Visual transform rendering is limited by the iced framework. Animation state is calculated correctly but visual effects may be pending framework updates.

### Configuration Not Saving

**Check file permissions**:
```bash
ls -la ~/.config/cosmic/com.cosmic.applet.notifications/
```

**Manually edit config**:
```bash
nano ~/.config/cosmic/com.cosmic.applet.notifications/config.ron
```

**Reset to defaults**:
```bash
rm ~/.config/cosmic/com.cosmic.applet.notifications/config.ron
# Restart applet to generate new config
```

## FAQ

### Q: Can I use this on non-COSMIC desktops?

**A**: Currently, the applet is designed specifically for COSMIC Desktop and uses libcosmic. Support for other desktops would require significant changes.

### Q: How do I update the applet?

**A**: Pull the latest changes and rebuild:
```bash
git pull origin main
nix develop
just build
```

### Q: Can I customize the notification sounds?

**A**: Sound support is planned for a future release. Currently, the applet respects system notification sounds.

### Q: Does this replace the default COSMIC notifications?

**A**: This is an alternative notification manager. You may need to disable the default cosmic-notifications daemon to avoid conflicts.

### Q: How much RAM does the applet use?

**A**: Typically 20-50MB at idle, up to 200MB with 100+ notifications in history. See performance metrics in PROJECT_PLAN.md.

### Q: Can I export my notification history?

**A**: History export is planned for v1.1. Currently, history is stored in the configuration directory.

### Q: Why aren't my animations showing?

**A**: The animation system is fully implemented, but visual rendering is limited by the iced GUI framework. Animation state is calculated correctly. Follow [Issue #15](https://github.com/yourusername/cosmic-applet-notification/issues/15) for updates on visual transforms.

### Q: How do I report bugs or request features?

**A**: Please open an issue on GitHub with:
- Description of the problem or feature request
- Steps to reproduce (for bugs)
- Expected vs. actual behavior
- Logs (if applicable): `RUST_LOG=trace cosmic-applet-notifications`

### Q: Is my notification history private?

**A**: Yes. All notification history is stored locally in your home directory (`~/.config/cosmic/`). No data is sent externally.

### Q: Can I contribute translations?

**A**: Internationalization (i18n) support is planned. Once implemented, translation contributions will be welcome!

---

## Additional Resources

- **[PROJECT_PLAN.md](./PROJECT_PLAN.md)** - Development roadmap and milestones
- **[ARCHITECTURE.md](./ARCHITECTURE.md)** - Technical architecture details
- **[DEVELOPMENT.md](./DEVELOPMENT.md)** - Development workflows
- **[GitHub Issues](https://github.com/yourusername/cosmic-applet-notification/issues)** - Bug reports and feature requests

---

**Need Help?**
- 💬 GitHub Discussions
- 🐛 GitHub Issues
- 💻 Matrix: #cosmic:nixos.org

---

Built with ❤️ for COSMIC Desktop
