# Contributing to COSMIC Notification Applet

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

Be respectful, inclusive, and considerate. We're all here to make great software.

## Getting Started

### Prerequisites

- NixOS with flakes enabled
- Git
- Basic Rust knowledge
- COSMIC Desktop Environment

### Setup

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/yourusername/cosmic-notification-applet
   cd cosmic-notification-applet
   ```
3. Enter development environment:
   ```bash
   nix develop
   ```
4. Create a feature branch:
   ```bash
   git checkout -b feature/my-feature
   ```

## Development Workflow

### Making Changes

1. Make your changes
2. Run tests:
   ```bash
   just test
   ```
3. Check code quality:
   ```bash
   just check
   ```
4. Format code:
   ```bash
   just fmt
   ```

### Commit Guidelines

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples**:
```bash
feat(ui): add notification history view
fix(dbus): handle disconnection properly
docs(readme): update installation instructions
```

### Pull Request Process

1. Update documentation if needed
2. Add tests for new features
3. Ensure all checks pass:
   ```bash
   just check-all
   ```
4. Push to your fork
5. Create a Pull Request
6. Wait for review

### PR Checklist

- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] Changelog entry added
- [ ] All checks passing
- [ ] Commits follow convention
- [ ] PR description explains changes

## Development Guidelines

### Code Style

- Follow Rust standard style guide
- Use `rustfmt` for formatting
- Run `clippy` and fix warnings
- Maximum line length: 100 characters

### Testing

- Write unit tests for new functionality
- Add integration tests for cross-component features
- Test with real notifications
- Check performance with stress tests

### Documentation

- Add doc comments to public items
- Update relevant markdown files
- Include examples where helpful
- Keep ARCHITECTURE.md current

## Areas for Contribution

### Good First Issues

- Documentation improvements
- Test coverage
- Translation support
- Bug fixes

### Feature Requests

Before implementing a new feature:
1. Check if an issue exists
2. Discuss the feature in an issue
3. Get approval from maintainers
4. Create a design document for large features

## Community

### Getting Help

- GitHub Issues: Bug reports and features
- GitHub Discussions: Questions and ideas
- Matrix: #cosmic:nixos.org

### Reporting Bugs

Include:
- COSMIC version
- NixOS version
- Steps to reproduce
- Expected vs actual behavior
- Relevant logs

**Template**:
```markdown
**Environment**
- COSMIC: [version]
- NixOS: [version]
- Applet: [version]

**Description**
[Clear description of the bug]

**Steps to Reproduce**
1. Step one
2. Step two
3. ...

**Expected Behavior**
[What should happen]

**Actual Behavior**
[What actually happens]

**Logs**
```
[Relevant log output]
```
```

## License

By contributing, you agree that your contributions will be licensed under the project's GPL-3.0 license.

## Recognition

Contributors are recognized in:
- CONTRIBUTORS.md file
- GitHub contributors page
- Release notes

## Questions?

Feel free to ask questions in:
- GitHub Discussions
- Project issues
- Matrix channel

Thank you for contributing! 🚀
