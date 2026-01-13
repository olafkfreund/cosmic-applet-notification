# Quick Start Guide

Get up and running with COSMIC Notification Applet development in 5 minutes!

## Prerequisites

✅ You need:
- NixOS with flakes enabled
- Git installed
- COSMIC Desktop running

## 1. Clone & Setup (1 minute)

```bash
# Clone your repository (once you create it)
git clone https://github.com/yourusername/cosmic-notification-applet
cd cosmic-notification-applet

# Enter development environment (downloads dependencies)
nix develop

# You'll see:
# ╔═════════════════════════════════════════════╗
# ║  COSMIC Notification Applet Dev Shell  🦀  ║
# ╚═════════════════════════════════════════════╝
```

## 2. Build & Run (2 minutes)

```bash
# Build the project
just build

# Run the applet (you'll see a basic icon)
just run

# In another terminal, send a test notification
notify-send "Hello" "COSMIC Notification Applet is running!"
```

## 3. Verify Setup (1 minute)

```bash
# Run tests
just test

# Check code quality
just check

# Format code
just fmt
```

## 4. Development Loop (ongoing)

```bash
# Edit code in src/
# Run with debug logging
RUST_LOG=debug just run

# Send test notifications
just test-notifications

# Watch for D-Bus signals
just dbus-monitor
```

## Project Navigation

### Start Here

1. **README.md** - Project overview
2. **ARCHITECTURE.md** - How it works
3. **PROJECT_PLAN.md** - What to build
4. **DEVELOPMENT.md** - How to develop

### For AI Assistants

5. **CLAUDE.md** - Project context
6. **docs/skills/** - Deep technical guides

### Implementation Guide

```
Phase 1: Foundation (Current)
├── Implement D-Bus listener (src/dbus/)
├── Implement notification manager (src/manager/)
├── Create basic UI (src/ui/)
└── Test with real notifications

Phase 2: Core Features
├── Add popup window with notifications
├── Implement notification history
└── Add configuration system

Phase 3: Polish
├── Add customization options
├── Improve UI/UX
└── Performance optimization
```

## Common Commands

```bash
# Development
just build              # Build project
just run               # Run applet
just run-debug         # Run with debug logging
just test              # Run tests
just check             # Lint with clippy
just fmt               # Format code

# Testing
just test-notifications  # Send test notifications
just dbus-monitor       # Watch D-Bus
just logs              # View panel logs

# Installation
just install           # Install to /usr/local
sudo just install      # System installation
just uninstall        # Remove installation

# Utilities
just clean            # Clean build artifacts
just doc              # Generate documentation
just update           # Update dependencies
```

## Understanding the Code

### Entry Point (`src/main.rs`)

```rust
fn main() {
    cosmic::applet::run::<NotificationApplet>(false, ())
}
```

### Message Flow

```
User Action → Message → update() → Command → Side Effect
                ↓
            State Change
                ↓
            view() → UI Update
```

### Adding a Feature

1. **Define message** in `Message` enum
2. **Handle message** in `update()`
3. **Update UI** in `view()`
4. **Test** with `just test`

## Debugging

### Enable Logging

```bash
# All debug logs
RUST_LOG=debug just run

# Specific module
RUST_LOG=cosmic_notification_applet::dbus=trace just run

# Multiple modules
RUST_LOG=cosmic_notification_applet=debug,zbus=trace just run
```

### Check D-Bus

```bash
# Watch all notifications
just dbus-monitor

# Check server info
just dbus-info

# Send test notification
notify-send "Test" "Debug message"
```

### View Logs

```bash
# Live panel logs
just logs

# Or manually
journalctl --user -u cosmic-panel -f
```

## Getting Help

### Documentation

| Question | Read This |
|----------|-----------|
| "How does it work?" | ARCHITECTURE.md |
| "What should I build?" | PROJECT_PLAN.md |
| "How do I develop?" | DEVELOPMENT.md |
| "How do I use X?" | docs/skills/X_skill.md |

### AI Assistance

```bash
# Provide these files to AI:
- CLAUDE.md (project context)
- Relevant skill file (technical details)
- Current code (what you're working on)
```

### Community

- **Matrix**: #cosmic:nixos.org
- **Issues**: GitHub Issues
- **Discussions**: GitHub Discussions

## Next Steps

### Immediate (Today)

1. ✅ Setup complete - you're here!
2. 📖 Read ARCHITECTURE.md
3. 🔨 Implement D-Bus listener
4. 🧪 Test with `notify-send`

### Short Term (This Week)

1. Complete Phase 1 foundation
2. Get basic notification display working
3. Add notification history
4. Write tests

### Long Term (This Month)

1. Complete Phase 2 core features
2. Add customization options
3. Polish UI/UX
4. Package for NixOS

## Troubleshooting

### "Command not found: just"

```bash
# You're not in the nix dev shell
nix develop
```

### "Applet doesn't appear"

```bash
# Restart COSMIC panel
just restart-panel
```

### "Build fails"

```bash
# Clean and rebuild
just clean
nix develop
just build
```

### "Can't receive notifications"

```bash
# Check D-Bus is running
echo $DBUS_SESSION_BUS_ADDRESS

# Test with notify-send
notify-send "Test" "Message"

# Watch D-Bus
just dbus-monitor
```

## Success Checklist

After running through this guide, you should have:

- [x] Development environment set up
- [x] Project builds without errors
- [x] Can run the applet
- [x] Understand the project structure
- [x] Know where to find documentation
- [x] Know common development commands

## You're Ready! 🚀

The project is fully set up and ready for development. Start with Phase 1 in the PROJECT_PLAN.md and refer to the skills when implementing features.

**Happy coding!**

---

**Last Updated**: 2025-01-13
