# NixOS module for cosmic-applet-notifications
#
# Usage in configuration.nix:
#   services.cosmic-applet-notifications.enable = true;

{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.services.cosmic-applet-notifications;
in
{
  options.services.cosmic-applet-notifications = {
    enable = mkEnableOption "COSMIC notification applet";

    package = mkOption {
      type = types.package;
      description = ''
        The cosmic-applet-notifications package to use.
        By default, uses the package from nixpkgs or the flake.
      '';
    };
  };

  config = mkIf cfg.enable {
    # Install the package
    environment.systemPackages = [ cfg.package ];

    # Ensure D-Bus service is available
    services.dbus.enable = true;
  };

  meta = {
    maintainers = with maintainers; [ ];
  };
}
