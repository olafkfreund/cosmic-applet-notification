# Installation Guide

**COSMIC Notification Applet** - Installation instructions for NixOS and COSMIC Desktop.

**Version**: 0.1.0
**Last Updated**: 2026-01-15

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation Methods](#installation-methods)
- [Post-Installation](#post-installation)
- [Configuration](#configuration)
- [Development Setup](#development-setup)
- [Updating](#updating)
- [Uninstallation](#uninstallation)
- [Troubleshooting](#troubleshooting)

## Prerequisites

### System Requirements

- **Operating System**: NixOS (22.05 or later recommended)
- **Desktop Environment**: COSMIC Desktop (Alpha 6 or later)
- **Nix**: Flakes enabled
- **Rust**: 1.90.0 or later (automatically provided by Nix)

### Enabling Nix Flakes

If you haven't enabled flakes yet, add to your NixOS configuration:

```nix
# configuration.nix
{
  nix.settings.experimental-features = [ "nix-command" "flakes" ];
}
```

Then rebuild:
```bash
sudo nixos-rebuild switch
```

### Installing COSMIC Desktop

If COSMIC Desktop isn't installed yet:

```nix
# configuration.nix
{
  services.desktopManager.cosmic.enable = true;
}
```

See [COSMIC Desktop documentation](https://github.com/pop-os/cosmic-epoch) for details.

## Installation Methods

### Method 1: Using the Flake (Recommended)

**Best for**: Regular users who want easy installation and updates.

Add the applet to your NixOS configuration:

```nix
# flake.nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    cosmic-applet-notifications.url = "github:olafkfreund/cosmic-applet-notification";
  };

  outputs = { self, nixpkgs, cosmic-applet-notifications, ... }: {
    nixosConfigurations.yourhostname = nixpkgs.lib.nixosSystem {
      modules = [
        ./configuration.nix
        cosmic-applet-notifications.nixosModules.default
      ];
    };
  };
}
```

Then in your `configuration.nix`:

```nix
{
  services.cosmic-applet-notifications.enable = true;
}
```

Rebuild your system:

```bash
sudo nixos-rebuild switch --flake .#yourhostname
```

### Method 2: Direct Package Installation

**Best for**: Users who prefer direct package management.

Add directly to your system packages:

```nix
# configuration.nix
{ inputs, pkgs, ... }:

{
  environment.systemPackages = [
    inputs.cosmic-applet-notifications.packages.${pkgs.system}.default
  ];
}
```

Rebuild:
```bash
sudo nixos-rebuild switch
```

### Method 3: Home Manager Installation

**Best for**: User-specific installation without system-wide changes.

For user-specific installation with Home Manager:

```nix
# home.nix
{ inputs, pkgs, ... }:

{
  home.packages = [
    inputs.cosmic-applet-notifications.packages.${pkgs.system}.default
  ];
}
```

Apply changes:
```bash
home-manager switch
```

### Method 4: Local Build from Source

**Best for**: Users who want to build locally or contribute.

Clone the repository and build:

```bash
git clone https://github.com/olafkfreund/cosmic-applet-notification.git
cd cosmic-applet-notification
nix build
```

Install to your profile:

```bash
nix profile install .
```

Or copy to user directory:

```bash
# Create applet directory if it doesn't exist
mkdir -p ~/.local/share/cosmic/applets

# Copy binary from build result
cp result/bin/cosmic-applet-notifications ~/.local/share/cosmic/applets/
```

## Post-Installation

### Verify Installation

```bash
# Check if binary is installed
which cosmic-applet-notifications

# Check if applet is running
ps aux | grep cosmic-applet-notifications
```

### Restart COSMIC Panel

After installation, restart the panel to load the applet:

```bash
cosmic-panel --reload

# Or log out and log back in
```

### First Launch

1. **Look for the bell icon** (🔔) in your COSMIC panel
2. **Click the icon** to open the notification popup
3. **Send a test notification**:
   ```bash
   notify-send "Test" "Hello from COSMIC Notification Applet!"
   ```
4. The notification should appear in the popup

## Configuration

### Configuration Location

After installation, configuration is stored at:

```
~/.config/cosmic/com.cosmic.applet.notifications/config.ron
```

### Quick Configuration

**Use a pre-made example**:
```bash
# Copy default configuration with comments
cp examples/default-config.ron ~/.config/cosmic/com.cosmic.applet.notifications/config.ron

# Or choose from specialized configs:
# - minimal-config.ron (lightweight)
# - power-user-config.ron (advanced features)
# - focus-mode-config.ron (deep work)
# - accessibility-config.ron (reduced motion)
```

**Edit configuration**:
```bash
nano ~/.config/cosmic/com.cosmic.applet.notifications/config.ron
```

**Apply changes**:
```bash
cosmic-panel --reload
```

See [USER_GUIDE.md](./USER_GUIDE.md) for detailed configuration options.

### Setting as Default Notification Handler

To use this applet as your primary notification handler:

```bash
# Disable default cosmic-notifications (if running)
systemctl --user disable cosmic-notifications
systemctl --user stop cosmic-notifications

# Or for other notification daemons
systemctl --user disable mako
systemctl --user disable dunst
```

## Development Setup

For development work:

```bash
# Enter development shell
nix develop

# Or with direnv (recommended for automatic environment loading)
echo "use flake" > .envrc
direnv allow
```

### Available Commands in Dev Shell

- `just build` - Build the project
- `just run` - Run the applet
- `just test` - Run tests
- `just check` - Run type checks and lints
- `just fmt` - Format code
- `just check-all` - Run all quality checks
- `just build-release` - Build optimized release version

See [DEVELOPMENT.md](./DEVELOPMENT.md) for detailed development workflows.

## Updating

### Flake Installation

```bash
# Update flake inputs
nix flake update

# Rebuild system
sudo nixos-rebuild switch --flake .#yourhostname

# Restart panel
cosmic-panel --reload
```

### Direct Package / Home Manager

```bash
# Update flake lock
nix flake update

# Rebuild
sudo nixos-rebuild switch
# Or for Home Manager
home-manager switch

# Restart panel
cosmic-panel --reload
```

### Source Installation

```bash
cd cosmic-applet-notification

# Pull latest changes
git pull origin main

# Rebuild
nix build

# Reinstall
nix profile upgrade cosmic-applet-notifications
# Or copy to user directory
cp result/bin/cosmic-applet-notifications ~/.local/share/cosmic/applets/

# Restart
cosmic-panel --reload
```

## Uninstallation

### NixOS Module

Remove from your configuration.nix:

```nix
services.cosmic-applet-notifications.enable = false;
```

Then rebuild:

```bash
sudo nixos-rebuild switch
```

### Direct Package

Remove from system packages and rebuild:

```nix
# Remove from environment.systemPackages
environment.systemPackages = [
  # cosmic-applet-notifications  # Remove this line
];
```

```bash
sudo nixos-rebuild switch
```

### Home Manager

Remove from home.nix and switch:

```nix
home.packages = [
  # cosmic-applet-notifications  # Remove this line
];
```

```bash
home-manager switch
```

### Nix Profile

```bash
nix profile remove cosmic-applet-notifications
```

### Manual Installation

```bash
# Remove binary
rm ~/.local/share/cosmic/applets/cosmic-applet-notifications

# Remove configuration (optional)
rm -rf ~/.config/cosmic/com.cosmic.applet.notifications/

# Restart panel
cosmic-panel --reload
```

## Troubleshooting

### Applet Doesn't Appear in Panel

**Check installation**:
```bash
which cosmic-applet-notifications
ls ~/.local/share/cosmic/applets/
```

**Restart COSMIC session**:
```bash
# Log out and log back in
# Or restart the panel
cosmic-panel --reload
```

**Check logs**:
```bash
journalctl --user -u cosmic-panel -f
```

### D-Bus Connection Issues

**Ensure D-Bus is running**:
```bash
systemctl --user status dbus
```

**Monitor D-Bus notifications**:
```bash
dbus-monitor "interface='org.freedesktop.Notifications'"
```

**Send test notification**:
```bash
notify-send "Test" "Testing D-Bus"
```

### Build Failures

**Update flake lock**:
```bash
nix flake update
```

**Check for dependency issues**:
```bash
nix flake check
```

**Clean build artifacts**:
```bash
nix develop -c cargo clean
nix develop -c cargo build
```

**Check Rust version**:
```bash
nix develop -c rustc --version
# Should be 1.90.0 or later
```

### Runtime Errors

**Enable debug logging**:
```bash
RUST_LOG=cosmic_applet_notifications=debug cosmic-applet-notifications
```

**Enable trace logging** (verbose):
```bash
RUST_LOG=cosmic_applet_notifications=trace cosmic-applet-notifications
```

**Check application logs**:
```bash
journalctl --user -u cosmic-panel | grep cosmic-applet-notifications
```

### Configuration Issues

**Reset to defaults**:
```bash
# Backup current config
cp ~/.config/cosmic/com.cosmic.applet.notifications/config.ron ~/config-backup.ron

# Remove config
rm ~/.config/cosmic/com.cosmic.applet.notifications/config.ron

# Restart applet (will create new default config)
cosmic-panel --reload
```

**Use example configuration**:
```bash
cp examples/default-config.ron ~/.config/cosmic/com.cosmic.applet.notifications/config.ron
cosmic-panel --reload
```

### Performance Issues

**Check resource usage**:
```bash
top | grep cosmic-applet-notifications
```

**Disable animations**:
```bash
# Edit config
nano ~/.config/cosmic/com.cosmic.applet.notifications/config.ron

# Set animations.enabled = false
# Restart panel
cosmic-panel --reload
```

**Enable system reduced motion**:
```bash
gsettings set org.gnome.desktop.interface enable-animations false
```

### Still Having Issues?

1. **Check documentation**:
   - [USER_GUIDE.md](./USER_GUIDE.md) - Feature documentation
   - [DEVELOPMENT.md](./DEVELOPMENT.md) - Development guide
   - [ARCHITECTURE.md](./ARCHITECTURE.md) - Technical details

2. **Search existing issues**:
   - [GitHub Issues](https://github.com/olafkfreund/cosmic-applet-notification/issues)

3. **Report a bug**:
   - Include logs: `RUST_LOG=trace cosmic-applet-notifications`
   - Include system info: NixOS version, COSMIC version
   - Steps to reproduce the problem

4. **Community support**:
   - Matrix: #cosmic:nixos.org
   - GitHub Discussions

## System Requirements Summary

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| NixOS | 22.05 | 23.11+ |
| COSMIC | Alpha 6 | Alpha 7+ |
| Rust | 1.90.0 | Latest stable |
| RAM | 50MB | 100MB |
| Disk | 20MB | 50MB |

## Next Steps

After installation:

1. **Read the user guide**: [USER_GUIDE.md](./USER_GUIDE.md)
2. **Try keyboard shortcuts**: See keyboard navigation section
3. **Customize configuration**: Copy examples from `examples/`
4. **Test features**: Send notifications with `notify-send`
5. **Report issues**: Help improve the applet!

---

**Thank you for installing COSMIC Notification Applet!** 🎉

For questions or feedback:
- 💬 GitHub Discussions
- 🐛 GitHub Issues: https://github.com/olafkfreund/cosmic-applet-notification/issues
- 📖 [User Guide](./USER_GUIDE.md)

---

Built with ❤️ for COSMIC Desktop on NixOS
