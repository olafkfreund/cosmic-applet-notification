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
    # Git dependencies require output hashes for vendoring
    # Using lib.fakeHash initially - build will provide real hashes
    outputHashes = {
      "accesskit-0.16.0" = lib.fakeHash;
      "accesskit_atspi_common-0.9.0" = lib.fakeHash;
      "accesskit_consumer-0.24.0" = lib.fakeHash;
      "accesskit_macos-0.17.0" = lib.fakeHash;
      "accesskit_unix-0.12.0" = lib.fakeHash;
      "accesskit_windows-0.22.0" = lib.fakeHash;
      "accesskit_winit-0.22.0" = lib.fakeHash;
      "atomicwrites-0.4.2" = lib.fakeHash;
      "clipboard_macos-0.1.0" = lib.fakeHash;
      "clipboard_wayland-0.2.2" = lib.fakeHash;
      "clipboard_x11-0.4.2" = lib.fakeHash;
      "cosmic-client-toolkit-0.1.0" = lib.fakeHash;
      "cosmic-config-0.1.0" = lib.fakeHash;
      "cosmic-config-derive-0.1.0" = lib.fakeHash;
      "cosmic-freedesktop-icons-0.4.0" = lib.fakeHash;
      "cosmic-panel-config-0.1.0" = lib.fakeHash;
      "cosmic-protocols-0.1.0" = lib.fakeHash;
      "cosmic-settings-daemon-0.1.0" = lib.fakeHash;
      "cosmic-text-0.16.0" = lib.fakeHash;
      "cosmic-theme-0.1.0" = lib.fakeHash;
      "dnd-0.1.0" = lib.fakeHash;
      "dpi-0.1.1" = lib.fakeHash;
      "iced-0.14.0-dev" = lib.fakeHash;
      "iced_accessibility-0.1.0" = lib.fakeHash;
      "iced_core-0.14.0-dev" = lib.fakeHash;
      "iced_futures-0.14.0-dev" = lib.fakeHash;
      "iced_glyphon-0.6.0" = lib.fakeHash;
      "iced_graphics-0.14.0-dev" = lib.fakeHash;
      "iced_renderer-0.14.0-dev" = lib.fakeHash;
      "iced_runtime-0.14.0-dev" = lib.fakeHash;
      "iced_tiny_skia-0.14.0-dev" = lib.fakeHash;
      "iced_wgpu-0.14.0-dev" = lib.fakeHash;
      "iced_widget-0.14.0-dev" = lib.fakeHash;
      "iced_winit-0.14.0-dev" = lib.fakeHash;
      "libcosmic-0.1.0" = lib.fakeHash;
      "mime-0.1.0" = lib.fakeHash;
      "smithay-clipboard-0.8.0" = lib.fakeHash;
      "softbuffer-0.4.1" = lib.fakeHash;
      "window_clipboard-0.4.1" = lib.fakeHash;
      "winit-0.30.5" = lib.fakeHash;
      "xdg-shell-wrapper-config-0.1.0" = lib.fakeHash;
    };
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
