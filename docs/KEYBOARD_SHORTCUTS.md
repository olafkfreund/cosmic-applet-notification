# Keyboard Shortcuts Guide

## Overview

The COSMIC Notification Applet provides comprehensive keyboard support for accessibility and power users. All notification management tasks can be performed using keyboard shortcuts without requiring a mouse.

## Navigation Shortcuts

### Notification Selection

| Shortcut | Action | Behavior |
|----------|--------|----------|
| `↑` Up Arrow | Previous notification | Moves selection up in the list. At the top, wraps to bottom. |
| `↓` Down Arrow | Next notification | Moves selection down in the list. At the bottom, wraps to top. |

**Visual Feedback**: The selected notification is highlighted with an accent color border and subtle background tint.

**Notes**:
- Navigation only works when the popup is open
- If no notification is selected, pressing Up starts from the bottom, Down starts from the top
- Changing notifications automatically resets action button selection

### Action Button Navigation

| Shortcut | Action | Behavior |
|----------|--------|----------|
| `Tab` | Cycle actions | Cycles through available action buttons in the selected notification. Wraps to first when reaching the last. |

**Visual Feedback**: The selected action button is highlighted with the "Suggested" button style (accent color).

**Notes**:
- Only works when a notification is selected
- Only applies to notifications that have action buttons
- If notification has no actions, Tab has no effect

## Action Shortcuts

### Notification Actions

| Shortcut | Action | Behavior |
|----------|--------|----------|
| `Enter` | Activate | Opens the first URL found in the notification body, or invokes the first action if no URL exists. |
| `Delete` | Dismiss | Removes the selected notification from the active list and clears selection. |

**Notes**:
- Enter prioritizes URLs over action buttons
- After dismissing with Delete, selection automatically clears
- Only works when a notification is selected

### Quick Action Shortcuts

| Shortcut | Action | Behavior |
|----------|--------|----------|
| `1` | Invoke action 1 | Directly invokes the first action without needing to Tab to it |
| `2` | Invoke action 2 | Directly invokes the second action |
| `3` | Invoke action 3 | Directly invokes the third action |
| `4-9` | Invoke action 4-9 | Directly invokes actions 4 through 9 |

**Notes**:
- Only works when a notification is selected
- Number corresponds to action position (1 = first, 2 = second, etc.)
- If action doesn't exist at that position, shortcut has no effect
- Useful for notifications with multiple actions (e.g., "Reply", "Mark Read", "Delete")

## Global Shortcuts

These shortcuts work regardless of whether the popup is open:

| Shortcut | Action | Behavior |
|----------|--------|----------|
| `Escape` | Close popup | Closes the notification popup and clears selection |
| `Ctrl+D` | Toggle DND | Toggles Do Not Disturb mode on/off |
| `Ctrl+1` | Show all | Sets filter to show all notifications (urgency level: 0) |
| `Ctrl+2` | Normal+ only | Sets filter to show only Normal and Critical notifications (urgency level: 1) |
| `Ctrl+3` | Critical only | Sets filter to show only Critical notifications (urgency level: 2) |

**Notes**:
- Global shortcuts work even when popup is closed
- DND state persists across restarts
- Urgency filter applies immediately to the active notification list

## Usage Examples

### Example 1: Reviewing and Dismissing Notifications

1. Click the applet icon to open the popup (or use system shortcut)
2. Press `↓` to select the first notification
3. Read the notification content
4. Press `Delete` to dismiss it
5. Press `↓` to move to the next notification
6. Repeat as needed
7. Press `Escape` to close the popup

### Example 2: Quick Action on Notification

1. Open the popup
2. Press `↓` or `↑` to navigate to the desired notification
3. Press `2` to invoke the second action directly (e.g., "Mark as Read")
4. Notification remains selected for further actions if needed

### Example 3: Opening Links from Notifications

1. Open the popup
2. Navigate to a notification containing a URL
3. Press `Enter` to open the link in your default browser
4. Browser opens automatically; applet remains open

### Example 4: Cycling Through Actions

1. Open the popup
2. Navigate to a notification with multiple actions
3. Press `Tab` repeatedly to highlight each action button
4. When desired action is highlighted, press `Enter` to invoke it
5. Or use number keys (`1-9`) to directly invoke without cycling

### Example 5: Quick Do Not Disturb

1. Press `Ctrl+D` at any time (popup open or closed)
2. DND mode activates immediately
3. All non-critical notifications are blocked
4. Press `Ctrl+D` again to disable DND

## Accessibility Features

### Keyboard-Only Operation

All features are accessible without a mouse:
- ✅ Navigate notification list
- ✅ Select notifications
- ✅ Open URLs
- ✅ Invoke actions
- ✅ Dismiss notifications
- ✅ Change filters
- ✅ Toggle DND mode

### Visual Feedback

Clear visual indicators for keyboard state:
- **Selected Notification**: Accent border (2px) + subtle background (15% opacity)
- **Selected Action**: Accent-colored button with "Suggested" style
- **Focus State**: Standard COSMIC focus indicators on all interactive elements

### Screen Reader Support

- Semantic HTML structure for notification content
- Proper ARIA labels on action buttons
- Focus management follows keyboard navigation
- State changes announced appropriately

## Tips and Best Practices

### Efficient Workflow

1. **Use number keys for common actions**: If you frequently use the same action (e.g., "Mark as Read"), remember its position and use the number key
2. **Wrap-around navigation**: Don't hesitate to press Up from the top or Down from the bottom - it wraps around
3. **Quick dismiss**: `↓` + `Delete` + `↓` + `Delete` creates a fast dismissal flow
4. **DND for focus**: Use `Ctrl+D` when you need uninterrupted work time

### Avoiding Common Mistakes

- ❌ Pressing number keys with Ctrl held (this triggers urgency filters instead)
- ❌ Trying to navigate when popup is closed (open popup first)
- ❌ Expecting Tab to work on notifications without actions (it only cycles action buttons)

### Power User Tips

- Combine urgency filters with navigation: `Ctrl+3` (Critical only) + `↓` (navigate critical notifications)
- Quick triage: `Ctrl+2` (Normal+), then dismiss all normals with repeated `↓` + `Delete`
- URL workflow: If notification has both URL and actions, Enter opens URL, number keys invoke actions

## Customization

Currently, keyboard shortcuts are **not configurable**. They are designed following COSMIC Desktop and accessibility best practices.

Future versions may include:
- Custom keybinding configuration
- Additional global shortcuts
- Vim-style navigation modes (hjkl)

## Troubleshooting

### Shortcuts Not Working

**Problem**: Keyboard shortcuts don't respond

**Solutions**:
1. Ensure the popup is open (most shortcuts require popup focus)
2. Check if another application has captured the shortcut
3. Verify you're not holding modifier keys unintentionally
4. Try clicking inside the popup to ensure it has focus

### Selection Disappears

**Problem**: Selected notification suddenly deselected

**Solutions**:
- This is normal behavior when:
  - Notification expires and is removed
  - You close the popup (Escape)
  - The notification you selected was dismissed
- Simply select another notification with Up/Down

### Action Number Doesn't Work

**Problem**: Pressing a number key doesn't invoke the action

**Solutions**:
1. Ensure a notification is selected first (use Up/Down)
2. Check that the notification has that many actions (e.g., pressing 3 on notification with only 2 actions does nothing)
3. Make sure you're not holding Ctrl or Alt keys

## Support

For issues or feature requests related to keyboard shortcuts, please:
1. Check this documentation first
2. Review existing issues: https://github.com/your-username/cosmic-applet-notification/issues
3. Open a new issue with the `keyboard-navigation` label

---

**Last Updated**: 2026-01-15
**Version**: Phase 4 Complete
