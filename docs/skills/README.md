# Skills Directory

This directory contains specialized knowledge files ("skills") that provide deep-dive information on specific aspects of the COSMIC Notification Applet project.

## Available Skills

### Core Technologies

- **[zbus_skill.md](./zbus_skill.md)** - D-Bus communication with zbus library
  - Signal subscription
  - D-Bus method implementation
  - Async patterns with tokio
  - Error handling

- **[libcosmic_applet_skill.md](./libcosmic_applet_skill.md)** - COSMIC applet development
  - Applet architecture
  - Panel integration
  - Popup window management
  - Theme integration

- **[notification_spec_skill.md](./notification_spec_skill.md)** - freedesktop.org notification specification
  - Protocol details
  - Hint parsing
  - Action handling
  - Security considerations

### Development Environment

- **[nixos_rust_skill.md](./nixos_rust_skill.md)** - NixOS + Rust development
  - Flake structure
  - Development workflow
  - Dependency management
  - Common issues

### Best Practices

- **[cosmic_best_practices_skill.md](./cosmic_best_practices_skill.md)** - COSMIC development patterns
  - Project structure
  - State management
  - Configuration
  - Performance optimization

## How to Use Skills

### For AI Assistants

These files are designed to be referenced by AI assistants (Claude, ChatGPT, Copilot) when providing help with the project. Each skill contains:

1. **Overview** - High-level introduction
2. **Core Concepts** - Key principles and patterns
3. **Implementation Examples** - Real code examples
4. **Best Practices** - Dos and don'ts
5. **Common Pitfalls** - Things to avoid
6. **Reference** - Links to official documentation

### For Developers

Skills are also useful for human developers as:
- Quick reference guides
- Learning resources
- Code example repositories
- Decision-making frameworks

## When to Read a Skill

- **Starting a new feature** - Read relevant skills first
- **Stuck on a problem** - Check if there's a skill covering that area
- **Code review** - Verify code follows skill guidelines
- **Onboarding** - Read all skills to understand project patterns

## Creating New Skills

When adding a new skill:

1. Use the existing format (Overview, Core Concepts, etc.)
2. Include practical code examples
3. Document common pitfalls
4. Link to official documentation
5. Update this README
6. Reference the skill in CLAUDE.md

## Skill Maintenance

Skills should be updated when:
- API changes occur
- Better patterns are discovered
- Common issues are identified
- New best practices emerge

---

**Last Updated**: 2025-01-13
