# GitHub Actions Workflows

This directory contains CI/CD workflows for the COSMIC Notification Applet.

## Workflows

### CI Workflow (`ci.yml`)

Main continuous integration workflow that runs on all pushes to `main` and all pull requests.

**Jobs:**

1. **Build and Test**
   - Builds the NixOS package
   - Runs the full test suite
   - **Pushes to Cachix** (only on main branch)
   - Builds and caches the development environment

2. **Format Check**
   - Verifies Rust code formatting with `cargo fmt`
   - Uses Cachix for faster dependency resolution

3. **Clippy Lint**
   - Runs Clippy linter with warnings as errors
   - Uses Cachix for faster dependency resolution

## Cachix Integration

### What is Cachix?

Cachix is a binary cache service for Nix that significantly speeds up builds by caching compiled packages. Instead of rebuilding dependencies from source every time, CI jobs can download pre-built binaries.

### How It Works

1. **On Pull Requests:**
   - Cachix is used in **read-only mode** (`skipPush: true`)
   - Downloads cached packages from the `cosmic` cache
   - Speeds up PR checks without consuming push quota

2. **On Main Branch:**
   - Cachix is used in **read-write mode** (`skipPush: false`)
   - Downloads cached packages AND uploads new builds
   - Newly built packages become available for future builds
   - Development shell is explicitly pushed for faster setup

### Configuration

**Required Secret:**
- `CACHIX_AUTH_TOKEN` - Authentication token for the Cachix `cosmic` cache
  - Location: Repository Settings → Secrets and Variables → Actions
  - This should be your Cachix auth token (from https://app.cachix.org/personal-auth-tokens)
  - For security, use a per-cache token (not a personal token)

**Cache Name:**
- `cosmic` - The Cachix binary cache name
- Public cache: Anyone can download, only authorized users can push

### Benefits

- **Faster CI:** Build times reduced from minutes to seconds for unchanged dependencies
- **Cost Savings:** Less compute time on GitHub Actions runners
- **Better DX:** Developers get faster feedback on PRs
- **Shared Cache:** All contributors benefit from cached builds

### Monitoring

View cache usage and statistics at:
- https://app.cachix.org/cache/cosmic

### Local Development

To use the same cache locally:

```bash
# Install cachix
nix-env -iA cachix -f https://cachix.org/api/v1/install

# Use the cosmic cache
cachix use cosmic

# Now your local builds will use cached packages
nix build
nix develop
```

### Troubleshooting

**Cache not being used:**
- Verify the cache name is correct (`cosmic`)
- Check that the secret `CACHIX_AUTH_TOKEN` is set correctly
- Ensure your Nix version supports flakes

**Packages not being pushed:**
- Only happens on `main` branch (not PRs)
- Check workflow logs for push confirmation
- Verify the `CACHIX_AUTH_TOKEN` has write permissions to your cache

**Build still slow:**
- First build after changes will be slow (building from source)
- Subsequent builds will be much faster (using cache)
- Check if dependencies changed (cache miss)

## Adding New Workflows

When creating new workflows:

1. Add Cachix integration for faster builds:
   ```yaml
   - uses: cachix/cachix-action@v14
     with:
       name: cosmic
       authToken: '${{ secrets.CACHIX_AUTH_TOKEN }}'
       skipPush: true  # or false for main branch
   ```

2. Use conditional pushing for main branch:
   ```yaml
   skipPush: ${{ github.ref != 'refs/heads/main' }}
   ```

3. Build logs for debugging:
   ```yaml
   - run: nix build --print-build-logs
   ```

## References

- [Cachix Documentation](https://docs.cachix.org/)
- [cachix-action GitHub](https://github.com/cachix/cachix-action)
- [Nix Flakes Guide](https://nixos.wiki/wiki/Flakes)
