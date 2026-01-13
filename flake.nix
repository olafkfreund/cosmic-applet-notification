{
  description = "COSMIC Notification Applet - Custom notification display for COSMIC Desktop";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Rust toolchain with necessary components
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src" # For rust-analyzer
            "rust-analyzer" # LSP
            "clippy" # Linter
          ];
        };

        # Native build dependencies
        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          just # Build system

          # Development tools
          cargo-flamegraph # Performance profiling
          cargo-watch # Auto-rebuild on changes
          cargo-outdated # Check for outdated dependencies
          cargo-audit # Security audit
          cargo-expand # Expand macros for debugging

          # Testing tools
          cargo-nextest # Better test runner
          cargo-tarpaulin # Code coverage (Linux only)

          # Nix tools
          nixpkgs-fmt # Nix code formatter
          statix # Nix linter
          deadnix # Find unused Nix code

          # Utilities
          dbus # D-Bus tools for testing
          libnotify # notify-send for testing
          direnv # Auto-load environment
        ];

        # Runtime dependencies
        buildInputs = with pkgs; [
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

          # COSMIC dependencies
          # Note: libcosmic is built from git, dependencies come through Cargo
          # The following are system dependencies that libcosmic needs:
          # - wayland, libxkbcommon (above)
          # - mesa/vulkan (above)
        ];

      in
      {
        # Development shell
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;

          # Environment variables
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
          PKG_CONFIG_PATH = pkgs.lib.makeSearchPath "lib/pkgconfig" buildInputs;

          # Rust backtrace for better error messages
          RUST_BACKTRACE = "1";

          shellHook = ''
            echo "╔═════════════════════════════════════════════╗"
            echo "║  COSMIC Notification Applet Dev Shell  🦀  ║"
            echo "╚═════════════════════════════════════════════╝"
            echo ""
            echo "🔧 Development Tools Available:"
            echo "  Build:    just build, just run, just build-release"
            echo "  Test:     just test, cargo nextest run"
            echo "  Quality:  just check, just fmt, just check-all"
            echo "  Profile:  just flamegraph"
            echo "  Watch:    just watch, cargo watch -x test"
            echo "  D-Bus:    just test-notifications, just dbus-monitor"
            echo ""
            echo "📦 Rust toolchain: $(rustc --version)"
            echo "🔍 rust-analyzer: $(rust-analyzer --version 2>/dev/null || echo 'available')"
            echo ""
            echo "💡 Tip: Run 'just' to see all available commands"
            echo "💡 Tip: Use 'direnv allow' for automatic environment loading"
            echo ""
          '';
        };

        # Package for building the applet
        packages = {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "cosmic-notification-applet";
            version = "0.1.0";

            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
              # Add git dependencies here if needed
              # outputHashes = {
              #   "libcosmic-0.1.0" = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
              # };
            };

            inherit nativeBuildInputs buildInputs;

            # Install phase
            installPhase = ''
              mkdir -p $out/bin
              mkdir -p $out/share/applications
              mkdir -p $out/share/icons/hicolor/scalable/apps
              
              # Install binary
              install -Dm755 target/release/cosmic-applet-notifications $out/bin/
              
              # Install desktop entry
              install -Dm644 data/com.system76.CosmicAppletNotifications.desktop \
                $out/share/applications/
              
              # Install icon
              install -Dm644 data/icons/com.system76.CosmicAppletNotifications.svg \
                $out/share/icons/hicolor/scalable/apps/
            '';

            meta = with pkgs.lib; {
              description = "Custom notification display applet for COSMIC Desktop";
              homepage = "https://github.com/yourusername/cosmic-notification-applet";
              license = licenses.gpl3Only;
              maintainers = [ ];
              platforms = platforms.linux;
            };
          };
        };

        # Checks (run with `nix flake check`)
        checks = {
          build = self.packages.${system}.default;

          clippy = pkgs.rustPlatform.buildRustPackage {
            pname = "cosmic-notification-applet-clippy";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            inherit nativeBuildInputs buildInputs;

            buildPhase = ''
              cargo clippy --all-targets -- -D warnings
            '';

            installPhase = "mkdir $out";
          };
        };

        # Formatter (run with `nix fmt`)
        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
