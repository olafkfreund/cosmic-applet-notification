{ lib
, rustPlatform
, pkg-config
, libxkbcommon
, wayland
, wayland-protocols
, libGL
, vulkan-loader
, mesa
, libinput
, fontconfig
, freetype
, dbus
, expat
}:

rustPlatform.buildRustPackage rec {
  pname = "cosmic-applet-notifications";
  version = "0.1.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
    # libcosmic and other git dependencies need output hashes
    # These will be automatically determined during first build
    # outputHashes = {
    #   "libcosmic-0.1.0" = lib.fakeHash;
    # };
  };

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    # Wayland and display
    libxkbcommon
    wayland
    wayland-protocols

    # Graphics
    libGL
    vulkan-loader
    mesa

    # Input and event handling
    libinput

    # Font rendering
    fontconfig
    freetype

    # D-Bus (required for notifications)
    dbus

    # Additional dependencies for libcosmic/iced
    expat
  ];

  # Install desktop entry and icon
  postInstall = ''
    # Install desktop entry
    install -Dm644 data/com.system76.CosmicAppletNotifications.desktop \
      $out/share/applications/com.system76.CosmicAppletNotifications.desktop

    # Install icon
    install -Dm644 data/icons/com.system76.CosmicAppletNotifications.svg \
      $out/share/icons/hicolor/scalable/apps/com.system76.CosmicAppletNotifications.svg
  '';

  # Required environment for Wayland applications
  postFixup = ''
    patchelf --add-rpath ${lib.makeLibraryPath buildInputs} $out/bin/cosmic-applet-notifications
  '';

  meta = with lib; {
    description = "Custom notification display applet for COSMIC Desktop";
    longDescription = ''
      A COSMIC Desktop panel applet that provides custom notification display
      with enhanced features including:
      - Notification history and persistence
      - Per-application filtering
      - Urgency-based filtering
      - Clickable URLs in notification bodies
      - Action button support
      - Do Not Disturb mode
      - Keyboard shortcuts
      - Configurable appearance
    '';
    homepage = "https://github.com/olafkfreund/cosmic-applet-notification";
    license = licenses.gpl3Only;
    maintainers = with maintainers; [ ];
    mainProgram = "cosmic-applet-notifications";
    platforms = platforms.linux;
  };
}
