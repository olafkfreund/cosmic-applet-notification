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
    # Hashes generated using nix-prefetch-git for each repository
    outputHashes = {
      "accesskit-0.16.0" = "sha256-uoLcd116WXQTu1ZTfJDEl9+3UPpGBN/QuJpkkGyRADQ=";
      "accesskit_atspi_common-0.9.0" = "sha256-uoLcd116WXQTu1ZTfJDEl9+3UPpGBN/QuJpkkGyRADQ=";
      "accesskit_consumer-0.24.0" = "sha256-uoLcd116WXQTu1ZTfJDEl9+3UPpGBN/QuJpkkGyRADQ=";
      "accesskit_macos-0.17.0" = "sha256-uoLcd116WXQTu1ZTfJDEl9+3UPpGBN/QuJpkkGyRADQ=";
      "accesskit_unix-0.12.0" = "sha256-uoLcd116WXQTu1ZTfJDEl9+3UPpGBN/QuJpkkGyRADQ=";
      "accesskit_windows-0.22.0" = "sha256-uoLcd116WXQTu1ZTfJDEl9+3UPpGBN/QuJpkkGyRADQ=";
      "accesskit_winit-0.22.0" = "sha256-uoLcd116WXQTu1ZTfJDEl9+3UPpGBN/QuJpkkGyRADQ=";
      "atomicwrites-0.4.2" = "sha256-QZSuGPrJXh+svMeFWqAXoqZQxLq/WfIiamqvjJNVhxA=";
      "clipboard_macos-0.1.0" = "sha256-+8CGmBf1Gl9gnBDtuKtkzUE5rySebhH7Bsq/kNlJofY=";
      "clipboard_wayland-0.2.2" = "sha256-+8CGmBf1Gl9gnBDtuKtkzUE5rySebhH7Bsq/kNlJofY=";
      "clipboard_x11-0.4.2" = "sha256-+8CGmBf1Gl9gnBDtuKtkzUE5rySebhH7Bsq/kNlJofY=";
      "cosmic-client-toolkit-0.1.0" = "sha256-KvXQJ/EIRyrlmi80WKl2T9Bn+j7GCfQlcjgcEVUxPkc=";
      "cosmic-config-0.1.0" = "sha256-iehy2ZYSfPm6WCsx+WM0reLPJCuXWy9FzYOT2LlP1Hk=";
      "cosmic-config-derive-0.1.0" = "sha256-iehy2ZYSfPm6WCsx+WM0reLPJCuXWy9FzYOT2LlP1Hk=";
      "cosmic-freedesktop-icons-0.4.0" = "sha256-D4bWHQ4Dp8UGiZjc6geh2c2SGYhB7mX13THpCUie1c4=";
      "cosmic-panel-config-0.1.0" = "sha256-1Xwe1uONJbl4wq6QBbTI1suLiSlTzU4e/5WBccvghHE=";
      "cosmic-protocols-0.1.0" = "sha256-KvXQJ/EIRyrlmi80WKl2T9Bn+j7GCfQlcjgcEVUxPkc=";
      "cosmic-settings-daemon-0.1.0" = "sha256-1yVIL3SQnOEtTHoLiZgBH21holNxcOuToyQ+QdvqoBg=";
      "cosmic-text-0.16.0" = "sha256-Dqi2eeMId4oiTLR0rd+cRKAUu7PSAa4t0ELzk4kqvqg=";
      "cosmic-theme-0.1.0" = "sha256-iehy2ZYSfPm6WCsx+WM0reLPJCuXWy9FzYOT2LlP1Hk=";
      "dnd-0.1.0" = "sha256-+8CGmBf1Gl9gnBDtuKtkzUE5rySebhH7Bsq/kNlJofY=";
      "dpi-0.1.1" = "sha256-PeHUUvJpntEhmAy8PSkXponc9OZ3YcQgpEe9sV4l8ig=";
      "iced-0.14.0-dev" = "sha256-iehy2ZYSfPm6WCsx+WM0reLPJCuXWy9FzYOT2LlP1Hk=";
      "iced_accessibility-0.1.0" = "sha256-iehy2ZYSfPm6WCsx+WM0reLPJCuXWy9FzYOT2LlP1Hk=";
      "iced_core-0.14.0-dev" = "sha256-iehy2ZYSfPm6WCsx+WM0reLPJCuXWy9FzYOT2LlP1Hk=";
      "iced_futures-0.14.0-dev" = "sha256-iehy2ZYSfPm6WCsx+WM0reLPJCuXWy9FzYOT2LlP1Hk=";
      "iced_glyphon-0.6.0" = "sha256-u1vnsOjP8npQ57NNSikotuHxpi4Mp/rV9038vAgCsfQ=";
      "iced_graphics-0.14.0-dev" = "sha256-iehy2ZYSfPm6WCsx+WM0reLPJCuXWy9FzYOT2LlP1Hk=";
      "iced_renderer-0.14.0-dev" = "sha256-iehy2ZYSfPm6WCsx+WM0reLPJCuXWy9FzYOT2LlP1Hk=";
      "iced_runtime-0.14.0-dev" = "sha256-iehy2ZYSfPm6WCsx+WM0reLPJCuXWy9FzYOT2LlP1Hk=";
      "iced_tiny_skia-0.14.0-dev" = "sha256-iehy2ZYSfPm6WCsx+WM0reLPJCuXWy9FzYOT2LlP1Hk=";
      "iced_wgpu-0.14.0-dev" = "sha256-iehy2ZYSfPm6WCsx+WM0reLPJCuXWy9FzYOT2LlP1Hk=";
      "iced_widget-0.14.0-dev" = "sha256-iehy2ZYSfPm6WCsx+WM0reLPJCuXWy9FzYOT2LlP1Hk=";
      "iced_winit-0.14.0-dev" = "sha256-iehy2ZYSfPm6WCsx+WM0reLPJCuXWy9FzYOT2LlP1Hk=";
      "libcosmic-0.1.0" = "sha256-iehy2ZYSfPm6WCsx+WM0reLPJCuXWy9FzYOT2LlP1Hk=";
      "mime-0.1.0" = "sha256-+8CGmBf1Gl9gnBDtuKtkzUE5rySebhH7Bsq/kNlJofY=";
      "smithay-clipboard-0.8.0" = "sha256-4InFXm0ahrqFrtNLeqIuE3yeOpxKZJZx+Bc0yQDtv34=";
      "softbuffer-0.4.1" = "sha256-/ocK79Lr5ywP/bb5mrcm7eTzeBbwpOazojvFUsAjMKM=";
      "window_clipboard-0.4.1" = "sha256-+8CGmBf1Gl9gnBDtuKtkzUE5rySebhH7Bsq/kNlJofY=";
      "winit-0.30.5" = "sha256-PeHUUvJpntEhmAy8PSkXponc9OZ3YcQgpEe9sV4l8ig=";
      "xdg-shell-wrapper-config-0.1.0" = "sha256-1Xwe1uONJbl4wq6QBbTI1suLiSlTzU4e/5WBccvghHE=";
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
