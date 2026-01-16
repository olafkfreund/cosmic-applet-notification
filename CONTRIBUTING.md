# Contributing to COSMIC Notification Applet

Thank you for your interest in contributing to the COSMIC Notification Applet! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Branch Protection and Pull Requests](#branch-protection-and-pull-requests)
- [Commit Message Guidelines](#commit-message-guidelines)
- [Testing Requirements](#testing-requirements)
- [Documentation](#documentation)
- [Code Quality](#code-quality)
- [Debugging](#debugging)
- [Release Process](#release-process)
- [Getting Help](#getting-help)

## Code of Conduct

This project follows standard open source community guidelines:

- Be respectful and inclusive
- Focus on constructive feedback
- Help create a welcoming environment for all contributors
- Report unacceptable behavior to the project maintainers

## Getting Started

### Prerequisites

Before contributing, ensure you have:

- **NixOS 22.05 or later** (or Nix package manager on another Linux distribution)
- **Nix flakes enabled** in your configuration
- **COSMIC Desktop Environment** (Alpha 6 or later) for testing
- **Git** for version control
- **GitHub account** for pull requests

### Development Environment Setup

1. **Fork and clone the repository:**

```bash
git clone https://github.com/YOUR_USERNAME/cosmic-applet-notification.git
cd cosmic-applet-notification
```

2. **Enter development shell:**

```bash
# Enter Nix development environment
nix develop

# Or use direnv for automatic environment loading
echo "use flake" > .envrc
direnv allow
```

The development shell provides:
- Rust toolchain (1.90.0+)
- All system dependencies
- Development tools (rustfmt, clippy, rust-analyzer)
- Build utilities (just, cargo)

3. **Verify setup:**

```bash
# Build the project
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy
```

## Development Workflow

### Branch Strategy

Starting with v0.1.0, we use a **feature branch workflow** with branch protection on `main`:

**Branch Naming Convention:**

- `feature/description` - New features (e.g., `feature/add-notification-groups`)
- `fix/description` - Bug fixes (e.g., `fix/memory-leak-in-history`)
- `docs/description` - Documentation updates (e.g., `docs/improve-install-guide`)
- `refactor/description` - Code refactoring (e.g., `refactor/simplify-dbus-listener`)
- `test/description` - Test additions/improvements (e.g., `test/add-filter-tests`)
- `chore/description` - Maintenance tasks (e.g., `chore/update-dependencies`)

### Creating a Feature Branch

1. **Ensure your local main is up to date:**

```bash
git checkout main
git pull origin main
```

2. **Create a feature branch:**

```bash
git checkout -b feature/your-feature-name
```

3. **Make your changes:**

- Write code following our [Code Quality](#code-quality) standards
- Add tests for new functionality
- Update documentation as needed
- Ensure all tests pass

4. **Commit your changes:**

Follow our [Commit Message Guidelines](#commit-message-guidelines)

```bash
git add .
git commit -m "feat: add notification grouping feature"
```

5. **Push to your fork:**

```bash
git push origin feature/your-feature-name
```

6. **Create a Pull Request:**

- Go to the GitHub repository
- Click "New Pull Request"
- Select your feature branch
- Fill out the PR template
- Request review from maintainers

## Branch Protection and Pull Requests

The `main` branch is protected with the following rules:

### Branch Protection Rules

- ✅ **Require pull request before merging**
  - Direct commits to `main` are not allowed
  - All changes must go through PR review

- ✅ **Require status checks to pass**
  - All CI checks must pass (build, tests, formatting)
  - Conversations must be resolved

- ✅ **Require conversation resolution**
  - All review comments must be addressed

- ✅ **Require linear history**
  - Use rebase or squash merge (no merge commits)

### Pull Request Process

1. **Create PR from feature branch**
2. **Fill out PR template completely**
3. **Ensure all CI checks pass**
4. **Request review from maintainers**
5. **Address review feedback**
6. **Resolve all conversations**
7. **Wait for approval**
8. **Maintainer will merge using squash or rebase**

### PR Requirements Checklist

Before requesting review, ensure:

- [ ] All tests pass locally (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Documentation updated (if applicable)
- [ ] CHANGELOG.md updated (for user-facing changes)
- [ ] Commit messages follow guidelines
- [ ] PR description is clear and complete

## Commit Message Guidelines

We follow **Conventional Commits** specification for clear and structured commit messages.

### Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Type

Must be one of the following:

- **feat**: New feature
- **fix**: Bug fix
- **docs**: Documentation changes
- **style**: Code style changes (formatting, missing semicolons, etc.)
- **refactor**: Code refactoring (no functional changes)
- **perf**: Performance improvements
- **test**: Adding or updating tests
- **chore**: Maintenance tasks (dependencies, build config, etc.)
- **ci**: CI/CD configuration changes

### Scope (Optional)

The scope specifies the area of change:

- `dbus` - D-Bus listener and types
- `manager` - Notification manager
- `ui` - User interface components
- `config` - Configuration system
- `positioning` - Popup positioning
- `animation` - Animation system
- `accessibility` - Accessibility features
- `nix` - Nix packaging

### Subject

- Use imperative mood ("add feature" not "added feature")
- Don't capitalize first letter
- No period at the end
- Maximum 50 characters

### Body (Optional)

- Explain what and why (not how)
- Wrap at 72 characters
- Separate from subject with blank line

### Footer (Optional)

- Reference issues: `Fixes #123`, `Closes #456`
- Breaking changes: `BREAKING CHANGE: description`

### Examples

```bash
# Simple feature
git commit -m "feat(dbus): add support for notification images"

# Bug fix with scope
git commit -m "fix(manager): resolve memory leak in history cleanup"

# Documentation
git commit -m "docs: update installation guide for NixOS"

# With body and footer
git commit -m "feat(ui): add keyboard navigation for notifications

Implement arrow key navigation through notification list.
Add visual indicators for selected notification.

Fixes #42"

# Breaking change
git commit -m "refactor(config)!: change config file format to TOML

BREAKING CHANGE: Configuration files must be migrated from RON to TOML.
See MIGRATION.md for migration instructions."
```

## Testing Requirements

All contributions must include appropriate tests.

### Test Types

1. **Unit Tests**: Test individual functions and modules
2. **Integration Tests**: Test component interactions
3. **Doc Tests**: Test code examples in documentation

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific module tests
cargo test manager::tests
cargo test dbus::tests

# Run ignored tests (require XDG portal)
cargo test -- --ignored

# Run with logging
RUST_LOG=debug cargo test
```

### Test Coverage Requirements

- New features must have unit tests
- Public APIs must have doc tests
- Bug fixes must include regression tests
- Integration tests for cross-module functionality

### Writing Good Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_with_descriptive_name() {
        // Arrange: Set up test data
        let config = create_test_config();

        // Act: Perform the operation
        let result = process_notification(config);

        // Assert: Verify expected outcome
        assert_eq!(result.status, NotificationStatus::Active);
    }
}
```

## Documentation

### Documentation Requirements

- Public APIs must have doc comments (`///`)
- Include examples in doc comments when helpful
- Update user-facing documentation for new features
- Document breaking changes in CHANGELOG.md

### Doc Comment Style

```rust
/// Brief one-line description of the function.
///
/// More detailed explanation if needed. Explain what the function does,
/// when to use it, and any important behavior.
///
/// # Arguments
///
/// * `notification` - The notification to process
/// * `config` - Application configuration
///
/// # Returns
///
/// Returns `Ok(())` if successful, or an error describing the failure.
///
/// # Errors
///
/// Returns `AppletError::DBusError` if D-Bus communication fails.
///
/// # Examples
///
/// ```
/// use cosmic_applet_notifications::manager::NotificationManager;
///
/// let mut manager = NotificationManager::new();
/// manager.add_notification(notification)?;
/// ```
pub fn add_notification(&mut self, notification: Notification) -> Result<(), AppletError> {
    // Implementation
}
```

### Documentation Files to Update

- **README.md** - User-facing project overview
- **USER_GUIDE.md** - Feature documentation
- **INSTALL.md** - Installation instructions
- **API_DOCUMENTATION.md** - API reference
- **DEVELOPMENT.md** - Development workflows
- **ARCHITECTURE.md** - Technical design
- **CHANGELOG.md** - User-facing changes

## Code Quality

### Formatting

We use `rustfmt` with default configuration:

```bash
# Format all code
cargo fmt

# Check formatting without modifying
cargo fmt -- --check
```

### Linting

We use `clippy` with strict settings:

```bash
# Run clippy
cargo clippy -- -D warnings

# Fix clippy suggestions automatically (where possible)
cargo clippy --fix
```

### Code Style Guidelines

**Naming Conventions:**

- Types: `PascalCase` (e.g., `NotificationManager`)
- Functions/Methods: `snake_case` (e.g., `process_notification`)
- Constants: `UPPER_SNAKE_CASE` (e.g., `MAX_HISTORY_SIZE`)
- Modules: `snake_case` (e.g., `notification_card`)

**Error Handling:**

- Use `Result<T, AppletError>` for fallible operations
- Use `thiserror` for error definitions
- Provide context in error messages
- Don't panic in library code

**Async Patterns:**

- Use `tokio::spawn` for long-running tasks
- Use channels (`mpsc`) for inter-task communication
- Document which functions require async context
- Avoid blocking operations in async code

**Maximum Line Length:** 100 characters

### Performance Considerations

- Profile before optimizing
- Avoid unnecessary allocations
- Use appropriate data structures
- Consider memory usage for long-running operations

## Debugging

### Enable Logging

```bash
# Debug level
RUST_LOG=cosmic_applet_notifications=debug cargo run

# Trace level (verbose)
RUST_LOG=cosmic_applet_notifications=trace cargo run

# Check panel logs
journalctl --user -u cosmic-panel -f | grep cosmic-applet-notifications
```

### Monitor D-Bus

```bash
# Watch notification signals
dbus-monitor "interface='org.freedesktop.Notifications'"

# Send test notification
notify-send "Test" "Testing the applet"
notify-send -u critical "Critical" "High priority notification"
```

### Performance Profiling

```bash
# Generate flamegraph
cargo flamegraph

# Or use just command
just flamegraph
```

### Common Issues

**Applet doesn't appear in panel:**
- Check desktop entry has `X-CosmicApplet=true`
- Verify install location matches COSMIC paths
- Check COSMIC panel logs

**Build errors:**
- Ensure using Rust 1.90.0 or later
- Check all system dependencies installed
- Try `cargo clean` and rebuild

## Release Process

### Versioning

We follow **Semantic Versioning** (SemVer):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with all changes
3. Run full test suite: `cargo test`
4. Build release version: `cargo build --release`
5. Test on NixOS system
6. Commit changes: `chore: bump version to X.Y.Z`
7. Create git tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`
8. Push tag: `git push origin vX.Y.Z`
9. Create GitHub release with CHANGELOG excerpt

## Getting Help

### Resources

- **Documentation**: Check `docs/` directory and `*.md` files
- **Examples**: See `examples/` directory for configuration examples
- **Tests**: Look at test files for usage examples

### Community

- **GitHub Issues**: [Report bugs or request features](https://github.com/olafkfreund/cosmic-applet-notification/issues)
- **GitHub Discussions**: Ask questions and discuss ideas
- **Matrix Chat**: #cosmic:nixos.org (COSMIC desktop development)

### Reporting Issues

When reporting bugs, please include:

- NixOS and COSMIC Desktop versions
- Steps to reproduce the issue
- Expected vs actual behavior
- Relevant logs: `RUST_LOG=trace cosmic-applet-notifications`
- Configuration file (if applicable)

### Feature Requests

Feature requests are welcome! Please:

- Check existing issues for duplicates
- Describe the use case and benefit
- Provide examples or mockups if applicable
- Consider contributing the implementation

## Thank You!

Thank you for contributing to the COSMIC Notification Applet! Your contributions help improve the COSMIC Desktop experience for everyone.

---

**Questions?** Open an issue or discussion on GitHub, or reach out on Matrix.
