# Installation Guide

## NixOS Installation

### Method 1: Using the Flake (Recommended)

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
sudo nixos-rebuild switch
```

### Method 2: Direct Package Installation

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

### Method 3: Local Build from Source

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

## Home Manager Installation

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

## Development Setup

For development work:

```bash
# Enter development shell
nix develop

# Or with direnv (recommended)
echo "use flake" > .envrc
direnv allow
```

Available commands in dev shell:
- `just build` - Build the project
- `just run` - Run the applet
- `just test` - Run tests
- `just check` - Run all checks
- `just fmt` - Format code

## Configuration

After installation, the applet will appear in the COSMIC panel. Configuration is stored at:

```
~/.config/cosmic/com.system76.CosmicAppletNotifications/v1/config.ron
```

Settings can be adjusted through the applet's settings panel (click the notification icon and scroll down).

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

Remove from system packages and rebuild.

### Nix Profile

```bash
nix profile remove cosmic-applet-notifications
```

## Troubleshooting

### Applet doesn't appear in panel

1. Log out and log back in to reload COSMIC
2. Check logs: `journalctl --user -u cosmic-panel`
3. Verify installation: `which cosmic-applet-notifications`

### D-Bus connection issues

Ensure D-Bus is running:

```bash
systemctl --user status dbus
```

### Build failures

If you encounter build errors with libcosmic or other git dependencies:

1. Update the flake lock: `nix flake update`
2. Check if you need to add output hashes for git dependencies in `package.nix`

### Missing dependencies

If you get pkg-config errors during build, ensure all system dependencies are listed in `package.nix` buildInputs.

## Requirements

- **OS**: NixOS (or Nix package manager on Linux)
- **Desktop**: COSMIC Desktop Environment
- **Nix**: Flakes support enabled

To enable flakes, add to `~/.config/nix/nix.conf` or `/etc/nix/nix.conf`:

```
experimental-features = nix-command flakes
```

## Support

For issues, please report at:
https://github.com/olafkfreund/cosmic-applet-notification/issues
