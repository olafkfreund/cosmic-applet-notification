# NixOS Rust Development Skill

## Overview

This skill covers developing Rust projects on NixOS with flakes, specifically for COSMIC Desktop development.

## Flake Structure

```nix
{
  description = "COSMIC Notification Applet";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        
        # Rust toolchain
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
        
        # Build dependencies for COSMIC
        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          just
        ];
        
        buildInputs = with pkgs; [
          libxkbcommon
          wayland
          libGL
          # COSMIC dependencies
        ];
        
      in {
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;
          
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
          
          shellHook = ''
            echo "🦀 Rust + COSMIC development environment"
            echo "Run 'just build' to build the project"
          '';
        };
        
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "cosmic-notification-applet";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          
          inherit nativeBuildInputs buildInputs;
        };
      };
    );
}
```

## Development Workflow

### Enter Development Shell
```bash
nix develop
```

### Build Project
```bash
# Inside nix develop
just build

# Or without entering shell
nix develop -c just build
```

### Run Project
```bash
nix develop -c just run
```

### Update Dependencies
```bash
# Update flake inputs
nix flake update

# Update Cargo dependencies
cargo update
```

## Common Issues

### rust-analyzer not working
```nix
# Ensure rust-src extension is included
rustToolchain = pkgs.rust-bin.stable.latest.default.override {
  extensions = [ "rust-src" "rust-analyzer" ];
};
```

### Missing system libraries
```bash
# Find required libraries
nix-shell -p nix-index --run "nix-locate libwayland"
```

### direnv Integration
Create `.envrc`:
```bash
use flake
```

Then: `direnv allow`

## Best Practices

1. Pin exact nixpkgs commit in production
2. Use rust-overlay for latest toolchain
3. Include all system dependencies in flake
4. Use `just` for build automation
5. Keep flake.lock in version control

---

**Last Updated**: 2025-01-13
