# Configuration Examples

This directory contains example configuration files for the COSMIC Notification Applet.

## Available Examples

### `default-config.ron`
**Description**: Default configuration with comprehensive comments explaining each option.

**Best for**: Understanding all available settings and starting your own custom configuration.

**Features**:
- All options documented with inline comments
- Balanced settings for general use
- Moderate history retention
- Animations enabled
- Auto positioning

**Use case**: First-time users, learning the configuration format

---

### `minimal-config.ron`
**Description**: Minimalist configuration with only essential features.

**Best for**: Users who want a simple, clean notification experience.

**Features**:
- No history (live notifications only)
- No animations (instant display)
- Auto positioning
- No app filtering
- Lightweight and fast

**Use case**: Minimalists, low-resource systems, users who prefer instant feedback

---

### `power-user-config.ron`
**Description**: Advanced configuration for keyboard-driven power users.

**Best for**: Users who want maximum control and customization.

**Features**:
- Custom positioning (top-right corner)
- Extended history (500 items, 30 days)
- Aggressive app filtering
- Full animations
- Optimized for keyboard workflow
- Filter out low-urgency notifications by default

**Use case**: Power users, developers, keyboard-focused workflows

---

### `focus-mode-config.ron`
**Description**: Deep work configuration with aggressive distraction blocking.

**Best for**: Users who need maximum focus and minimal interruptions.

**Features**:
- Do Not Disturb enabled by default
- Critical notifications only
- All social/entertainment apps blocked
- No animations (non-distracting)
- Minimal history (1 day)

**Use case**: Deep work sessions, coding marathons, studying, writing

---

### `accessibility-config.ron`
**Description**: Optimized for accessibility needs.

**Best for**: Users with motion sensitivity, visual impairments, or motor control challenges.

**Features**:
- All animations disabled (respects reduced motion preferences)
- Consistent, predictable positioning (center)
- Extended history for later review
- Full keyboard navigation support
- Works with high-contrast themes

**Use case**: Users with vestibular disorders, motion sensitivity, screen reader users, motor control challenges

---

## How to Use These Examples

### Option 1: Copy and Use Directly

```bash
# Choose an example (replace minimal-config.ron with your choice)
cp examples/minimal-config.ron ~/.config/cosmic/com.cosmic.applet.notifications/config.ron

# Restart the COSMIC panel or applet
cosmic-panel --reload
```

### Option 2: Use as a Starting Point

```bash
# Copy an example to edit
cp examples/power-user-config.ron ~/.config/cosmic/com.cosmic.applet.notifications/config.ron

# Edit the configuration
nano ~/.config/cosmic/com.cosmic.applet.notifications/config.ron

# Restart to apply changes
cosmic-panel --reload
```

### Option 3: Mix and Match

You can combine features from different examples:

1. Start with `default-config.ron` as a base
2. Copy the `popup_position` section from `power-user-config.ron`
3. Copy the `animations` section from `accessibility-config.ron`
4. Add `app_filters` from `focus-mode-config.ron`

## Configuration Format

The configuration uses **RON (Rusty Object Notation)** format:

- Similar to JSON but more flexible
- Supports comments with `//`
- Trailing commas are allowed
- More human-friendly than JSON

### RON Resources
- [RON GitHub Repository](https://github.com/ron-rs/ron)
- [RON Specification](https://github.com/ron-rs/ron/blob/master/docs/grammar.md)

## Configuration Options Reference

### Filtering Options

```ron
do_not_disturb: bool,      // Enable DND mode
min_urgency_level: u8,     // 0=All, 1=Normal+, 2=Critical
```

### History Options

```ron
history_enabled: bool,           // Enable persistent history
max_history_items: usize,        // Maximum number of notifications
history_retention_days: u64,     // Days before auto-deletion
```

### Position Options

```ron
popup_position: PopupPosition(
    mode: Auto | PanelRelative,
    anchor: Start | Center | End | AppletIcon,
    offset_x: i32,               // -500 to 500
    offset_y: i32,               // -500 to 500
    snap_to_edge: bool,
    snap_threshold: u32,         // 5 to 100 pixels
),
```

### Animation Options

```ron
animations: AnimationConfig(
    enabled: bool,                 // Master toggle
    notification_appear: bool,
    notification_dismiss: bool,
    show_progress: bool,
),
```

### App Filter Options

```ron
app_filters: {
    "AppName": true,   // Allow
    "OtherApp": false, // Block
},
```

## Tips

### Finding App Names

To find the exact name an app uses for notifications:

```bash
# Monitor D-Bus for notification signals
dbus-monitor "interface='org.freedesktop.Notifications'" | grep -i "string"

# Send a test notification from the app
# Check the monitor output for the app_name field
```

### Testing Configuration

```bash
# Enable debug logging
RUST_LOG=cosmic_applet_notifications=debug cosmic-applet-notifications

# Send test notifications
notify-send "Test" "Testing configuration"
notify-send -u critical "Critical Test" "This is urgent"
```

### Validation

The applet automatically validates and sanitizes invalid configurations:

- Out-of-range values are clamped to valid ranges
- Invalid enum values fall back to defaults
- Missing fields use default values

Check logs for validation warnings:
```bash
journalctl --user -u cosmic-panel | grep cosmic-applet-notifications
```

## Need Help?

- **Full Documentation**: See [USER_GUIDE.md](../USER_GUIDE.md)
- **Technical Details**: See [ARCHITECTURE.md](../ARCHITECTURE.md)
- **Issues**: [GitHub Issues](https://github.com/yourusername/cosmic-applet-notification/issues)

---

Happy configuring! 🎨
