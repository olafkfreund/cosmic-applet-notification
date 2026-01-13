# COSMIC Development Best Practices Skill

## Overview

This skill provides best practices, patterns, and guidelines for developing applications and applets for the COSMIC Desktop Environment.

## Project Structure

### Recommended Layout

```
cosmic-project/
├── src/
│   ├── main.rs              # Entry point
│   ├── app.rs               # Main application/applet struct
│   ├── config.rs            # Configuration management
│   ├── core/                # Core business logic
│   ├── ui/                  # UI components
│   │   ├── mod.rs
│   │   ├── pages/          # Different screens
│   │   └── widgets/        # Custom widgets
│   └── localization/        # i18n support
├── i18n/                    # Translation files
├── data/
│   ├── *.desktop           # Desktop entry
│   ├── icons/              # Application icons
│   └── *.metainfo.xml     # AppStream metadata
├── tests/
├── Cargo.toml
├── justfile
└── README.md
```

## Application State Management

### Use cosmic::Application Trait

```rust
use cosmic::{Application, Element, Theme};
use cosmic::iced::Command;

pub struct MyApp {
    core: cosmic::app::Core,
    // Your state here
}

impl Application for MyApp {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    
    const APP_ID: &'static str = "com.example.MyApp";
    
    fn core(&self) -> &cosmic::app::Core {
        &self.core
    }
    
    fn core_mut(&mut self) -> &mut cosmic::app::Core {
        &mut self.core
    }
    
    fn init(
        core: cosmic::app::Core,
        _flags: Self::Flags,
    ) -> (Self, Command<Self::Message>) {
        let app = MyApp { core };
        (app, Command::none())
    }
    
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            // Handle messages
        }
        Command::none()
    }
    
    fn view(&self) -> Element<Self::Message> {
        // Build UI
    }
}
```

## Message Design

### Well-Structured Messages

```rust
#[derive(Debug, Clone)]
pub enum Message {
    // User interactions
    ButtonPressed,
    TextInput(String),
    
    // System events
    ConfigChanged(Config),
    ThemeChanged(Theme),
    
    // Async results
    DataLoaded(Result<Data, Error>),
    
    // Navigation
    Navigate(Page),
    
    // Internal
    Tick,
}
```

### Message Categories

1. **User Actions** - Direct user input
2. **System Events** - OS/Desktop events
3. **Async Results** - Background task completions
4. **Internal Events** - App state changes

## Configuration Management

### Use cosmic-config

```rust
use cosmic_config::{Config, CosmicConfigEntry};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub theme: String,
    pub language: String,
    pub custom_setting: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            language: "en".to_string(),
            custom_setting: false,
        }
    }
}

// Load config
let config = Config::new("com.example.MyApp", 1)?;
let app_config: AppConfig = config.get("settings")?;

// Save config
config.set("settings", &app_config)?;

// Watch for changes
let rx = config.watch();
```

## Theming

### Respect System Theme

```rust
use cosmic::theme;

fn view(&self) -> Element<Message> {
    let theme = self.core.system_theme();
    
    container(content)
        .style(theme::Container::Primary)
        .into()
}
```

### Custom Styles

```rust
use cosmic::iced::widget::container;
use cosmic::theme;

fn custom_container_style() -> theme::Container {
    theme::Container::custom(|theme| {
        container::Appearance {
            background: Some(theme.background.base.into()),
            border_radius: theme.corner_radii.radius_m.into(),
            border_width: 1.0,
            border_color: theme.background.divider,
            ..Default::default()
        }
    })
}
```

## Widget Usage

### Prefer cosmic Widgets

```rust
use cosmic::widget::{button, container, text};

fn build_ui(&self) -> Element<Message> {
    container(
        column![
            text("Hello COSMIC"),
            button::standard("Click me")
                .on_press(Message::ButtonPressed)
        ]
    ).into()
}
```

### Common Patterns

```rust
// Padded container
container(content)
    .padding(theme.space_m())
    
// Scrollable list
scrollable(
    column(items)
)

// Responsive spacing
row![item1, item2]
    .spacing(theme.space_s())
```

## Async Operations

### Use Commands for Side Effects

```rust
fn update(&mut self, message: Message) -> Command<Message> {
    match message {
        Message::LoadData => {
            return Command::perform(
                async { load_data_async().await },
                |result| Message::DataLoaded(result)
            );
        }
        Message::DataLoaded(result) => {
            match result {
                Ok(data) => self.data = Some(data),
                Err(e) => tracing::error!("Failed to load: {}", e),
            }
        }
    }
    Command::none()
}
```

## Error Handling

### User-Friendly Errors

```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Failed to load configuration")]
    ConfigError,
    
    #[error("Network request failed: {0}")]
    NetworkError(String),
    
    #[error("Invalid input: {0}")]
    ValidationError(String),
}

// Show errors to user
fn show_error(&self, error: &AppError) -> Element<Message> {
    container(
        text(format!("Error: {}", error))
            .style(theme::Text::Error)
    ).into()
}
```

## Localization

### Support i18n from Start

```rust
use cosmic::app::message::cosmic_i18n;

cosmic_i18n!(
    "com.example.myapp",
    "i18n"
);

fn localized_text(&self) -> String {
    fl!("hello-world")
}
```

**Translation file** (`i18n/en/messages.ftl`):
```fluent
hello-world = Hello, World!
button-save = Save
error-network = Network error occurred
```

## Performance

### Optimize Rendering

```rust
// Cache expensive computations
#[derive(Default)]
struct Cache {
    computed_value: Option<ExpensiveData>,
}

impl Cache {
    fn get_or_compute(&mut self) -> &ExpensiveData {
        self.computed_value.get_or_insert_with(|| {
            // Expensive computation
        })
    }
}
```

### Lazy Loading

```rust
// Load data only when needed
enum DataState {
    NotLoaded,
    Loading,
    Loaded(Data),
    Error(String),
}

fn ensure_loaded(&mut self) -> Command<Message> {
    match self.data_state {
        DataState::NotLoaded => {
            self.data_state = DataState::Loading;
            Command::perform(load_data(), Message::DataLoaded)
        }
        _ => Command::none(),
    }
}
```

## Accessibility

### Keyboard Navigation

```rust
use cosmic::iced::keyboard;

fn subscription(&self) -> Subscription<Message> {
    keyboard::on_key_press(|key, _modifiers| {
        match key {
            keyboard::Key::Named(keyboard::key::Named::Escape) => {
                Some(Message::Close)
            }
            _ => None,
        }
    })
}
```

### Screen Reader Support

```rust
// Add labels to interactive elements
button::standard("Save")
    .on_press(Message::Save)
    .label("Save your changes") // Screen reader description
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_message_handling() {
        let mut app = MyApp::default();
        let cmd = app.update(Message::ButtonPressed);
        assert!(app.button_pressed);
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_config_persistence() {
    let config = AppConfig::default();
    save_config(&config).await.unwrap();
    let loaded = load_config().await.unwrap();
    assert_eq!(config, loaded);
}
```

## Documentation

### Doc Comments

```rust
/// Main application state for MyApp.
///
/// # Examples
///
/// ```
/// let app = MyApp::default();
/// ```
pub struct MyApp {
    /// Core COSMIC application state
    core: cosmic::app::Core,
}

/// Processes user input and updates application state.
///
/// # Arguments
///
/// * `message` - The message to process
///
/// # Returns
///
/// Command to execute side effects
fn update(&mut self, message: Message) -> Command<Message> {
    // ...
}
```

## Common Pitfalls

### ❌ Don't

```rust
// Blocking the UI thread
fn update(&mut self, message: Message) -> Command<Message> {
    std::thread::sleep(Duration::from_secs(1)); // BAD!
    Command::none()
}

// Unwrapping in production
let value = some_option.unwrap(); // DANGEROUS!

// Not handling errors
let _ = file.write_all(data); // Silent failures!
```

### ✅ Do

```rust
// Use async for long operations
fn update(&mut self, message: Message) -> Command<Message> {
    Command::perform(
        async { tokio::time::sleep(Duration::from_secs(1)).await },
        |_| Message::Complete
    )
}

// Handle Options properly
let value = some_option.unwrap_or_default();

// Propagate errors
file.write_all(data)
    .map_err(|e| AppError::from(e))?;
```

## Resource Management

### Clean Up Resources

```rust
impl Drop for MyApp {
    fn drop(&mut self) {
        // Save state
        if let Err(e) = self.save_state() {
            tracing::error!("Failed to save state: {}", e);
        }
        
        // Close connections
        self.close_connections();
    }
}
```

## Desktop Integration

### Desktop Entry

```desktop
[Desktop Entry]
Name=My COSMIC App
Comment=A great COSMIC application
Exec=my-cosmic-app
Icon=com.example.MyApp
Terminal=false
Type=Application
Categories=COSMIC;Utility;
Keywords=cosmic;utility;
```

### AppStream Metadata

```xml
<?xml version="1.0" encoding="UTF-8"?>
<component type="desktop-application">
  <id>com.example.MyApp</id>
  <name>My COSMIC App</name>
  <summary>A great COSMIC application</summary>
  <description>
    <p>Detailed description of your application.</p>
  </description>
  <screenshots>
    <screenshot type="default">
      <image>https://example.com/screenshot.png</image>
    </screenshot>
  </screenshots>
</component>
```

## Release Checklist

- [ ] Version bumped in Cargo.toml
- [ ] CHANGELOG.md updated
- [ ] All tests passing
- [ ] Documentation updated
- [ ] Desktop entry correct
- [ ] Icons included
- [ ] Translations complete
- [ ] No clippy warnings
- [ ] Release notes written

## Resources

- [COSMIC Epoch](https://github.com/pop-os/cosmic-epoch)
- [libcosmic](https://github.com/pop-os/libcosmic)
- [COSMIC Apps](https://github.com/pop-os/cosmic-applets)
- [iced Documentation](https://docs.rs/iced/)

---

**Last Updated**: 2025-01-13
