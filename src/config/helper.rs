// Configuration helper functions
//
// Handles loading, saving, and watching configuration changes.

use super::{AppletConfig, CONFIG_VERSION};
use std::path::PathBuf;

/// Configuration helper
///
/// Manages config persistence using cosmic-config (when available)
/// Falls back to file-based storage if cosmic-config unavailable.
pub struct ConfigHelper {
    config_path: PathBuf,
}

impl ConfigHelper {
    /// Create a new config helper
    ///
    /// Determines config path based on XDG base directory spec.
    /// Attempts to create config directory hierarchy on initialization.
    pub fn new() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("cosmic")
            .join("com.system76.CosmicAppletNotifications")
            .join("v1");

        // Attempt to create full directory hierarchy including v1 subdirectory
        // Errors are logged but don't prevent construction - save() will fail
        // gracefully with proper error if directory creation fails
        if let Err(e) = std::fs::create_dir_all(&config_dir) {
            tracing::warn!(
                "Failed to create config directory {:?}: {}. Config save will fail.",
                config_dir,
                e
            );
        }

        let config_path = config_dir.join("config.ron");

        Self { config_path }
    }

    /// Load configuration from disk
    ///
    /// Returns default config if file doesn't exist or is invalid.
    /// Automatically sanitizes and migrates loaded config.
    pub fn load(&self) -> AppletConfig {
        match std::fs::read_to_string(&self.config_path) {
            Ok(content) => {
                match ron::from_str::<AppletConfig>(&content) {
                    Ok(mut config) => {
                        tracing::info!("Loaded configuration from {:?}", self.config_path);

                        // Migrate if needed
                        if config.version < CONFIG_VERSION {
                            config.migrate(config.version);
                            // Save migrated config
                            if let Err(e) = self.save(&config) {
                                tracing::warn!("Failed to save migrated config: {}", e);
                            }
                        }

                        // Sanitize values
                        if !config.validate() {
                            tracing::warn!("Config validation failed, sanitizing");
                            config.sanitize();
                            // Save sanitized config
                            if let Err(e) = self.save(&config) {
                                tracing::warn!("Failed to save sanitized config: {}", e);
                            }
                        }

                        config
                    }
                    Err(e) => {
                        tracing::error!("Failed to parse config: {}", e);
                        tracing::info!("Using default configuration");
                        let default = AppletConfig::default();
                        // Save default config
                        if let Err(e) = self.save(&default) {
                            tracing::warn!("Failed to save default config: {}", e);
                        }
                        default
                    }
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                tracing::info!("No config file found, creating default");
                let default = AppletConfig::default();
                // Save default config
                if let Err(e) = self.save(&default) {
                    tracing::warn!("Failed to save default config: {}", e);
                }
                default
            }
            Err(e) => {
                tracing::error!("Failed to read config file: {}", e);
                tracing::info!("Using default configuration");
                AppletConfig::default()
            }
        }
    }

    /// Save configuration to disk
    ///
    /// Creates parent directories if needed.
    /// Sets restrictive file permissions (0600) for security.
    pub fn save(&self, config: &AppletConfig) -> Result<(), std::io::Error> {
        // Ensure parent directory exists
        if let Some(parent) = self.config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Serialize config
        let serialized = ron::ser::to_string_pretty(config, Default::default())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        // Write to file
        std::fs::write(&self.config_path, serialized)?;

        // Set restrictive permissions (owner read/write only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&self.config_path)?.permissions();
            perms.set_mode(0o600);
            std::fs::set_permissions(&self.config_path, perms)?;
        }

        tracing::info!("Saved configuration to {:?}", self.config_path);
        Ok(())
    }

    /// Get config file path
    pub fn path(&self) -> &PathBuf {
        &self.config_path
    }
}

impl Default for ConfigHelper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_helper() -> (ConfigHelper, TempDir) {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("config.ron");

        let helper = ConfigHelper {
            config_path: config_path.clone(),
        };

        (helper, temp_dir)
    }

    #[test]
    fn test_load_nonexistent_creates_default() {
        let (helper, _temp) = create_test_helper();

        let config = helper.load();
        assert_eq!(config, AppletConfig::default());

        // Should have created the file
        assert!(helper.config_path.exists());
    }

    #[test]
    fn test_save_and_load() {
        let (helper, _temp) = create_test_helper();

        let mut config = AppletConfig::default();
        config.do_not_disturb = true;
        config.max_visible_notifications = 15;

        // Save
        helper.save(&config).unwrap();

        // Load
        let loaded = helper.load();
        assert_eq!(loaded.do_not_disturb, true);
        assert_eq!(loaded.max_visible_notifications, 15);
    }

    #[test]
    fn test_load_invalid_config_falls_back_to_default() {
        let (helper, _temp) = create_test_helper();

        // Write invalid RON
        std::fs::write(&helper.config_path, "invalid ron content").unwrap();

        let config = helper.load();
        assert_eq!(config, AppletConfig::default());
    }

    #[test]
    fn test_sanitize_on_load() {
        let (helper, _temp) = create_test_helper();

        // Create config with invalid values
        let mut config = AppletConfig::default();
        config.max_visible_notifications = 100; // Out of range

        // Manually save without validation
        let serialized = ron::to_string(&config).unwrap();
        std::fs::create_dir_all(helper.config_path.parent().unwrap()).unwrap();
        std::fs::write(&helper.config_path, serialized).unwrap();

        // Load should sanitize
        let loaded = helper.load();
        assert_eq!(loaded.max_visible_notifications, 50); // Clamped to max
    }
}
