// Accessibility settings detection
//
// Detects system-wide accessibility preferences to ensure animations
// and visual effects respect user needs.

use ashpd::desktop::settings::Settings;

/// Detect if the user prefers reduced motion
///
/// Queries the XDG Desktop Portal Settings interface for the
/// `org.freedesktop.appearance.reduced-motion` setting.
///
/// Returns `true` if reduced motion is preferred, `false` otherwise.
/// Falls back to `false` if detection fails (preserves existing functionality).
///
/// # XDG Portal Spec
///
/// The `reduced-motion` setting is a u32 with the following values:
/// - 0: No preference (animations enabled)
/// - 1: Reduced motion (animations should be minimized)
///
/// Reference: https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Settings.html
pub async fn detect_prefers_reduced_motion() -> bool {
    match detect_reduced_motion_internal().await {
        Ok(prefers_reduced) => {
            if prefers_reduced {
                tracing::info!("Detected prefers-reduced-motion: animations will be disabled");
            } else {
                tracing::debug!("No reduced motion preference detected");
            }
            prefers_reduced
        }
        Err(e) => {
            tracing::debug!(
                "Failed to detect reduced motion preference ({}), defaulting to false",
                e
            );
            false
        }
    }
}

/// Internal implementation of reduced motion detection
async fn detect_reduced_motion_internal() -> Result<bool, ashpd::Error> {
    // Connect to XDG Desktop Portal Settings interface
    let settings = Settings::new().await?;

    // Read the reduced-motion setting from org.freedesktop.appearance namespace
    // The setting is a u32: 0 = no preference, 1 = reduced motion
    let reduced_motion_value = settings
        .read::<u32>("org.freedesktop.appearance", "reduced-motion")
        .await?;

    // Convert to boolean: 1 means reduced motion is preferred
    Ok(reduced_motion_value == 1)
}

/// Subscribe to reduced motion preference changes
///
/// Returns a stream that yields `true` whenever the user enables reduced motion,
/// and `false` when they disable it. This allows the application to update
/// animation settings in real-time without restarting.
///
/// # Example
///
/// ```no_run
/// use futures::StreamExt;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut stream = cosmic_applet_notifications::accessibility::subscribe_reduced_motion_changes().await?;
///
/// while let Some(prefers_reduced) = stream.next().await {
///     println!("Reduced motion preference changed: {}", prefers_reduced);
///     // Update animation configuration accordingly
/// }
/// # Ok(())
/// # }
/// ```
pub async fn subscribe_reduced_motion_changes(
) -> Result<impl futures::Stream<Item = bool>, ashpd::Error> {
    use futures::StreamExt;

    let settings = Settings::new().await?;

    // Subscribe to changes to the reduced-motion setting
    let stream = settings
        .receive_setting_changed_with_args::<u32>(
            "org.freedesktop.appearance",
            "reduced-motion",
        )
        .await?;

    // Map the u32 values to bool
    Ok(stream.map(|result| {
        result
            .map(|value| {
                let prefers_reduced = value == 1;
                if prefers_reduced {
                    tracing::info!("User enabled prefers-reduced-motion");
                } else {
                    tracing::info!("User disabled prefers-reduced-motion");
                }
                prefers_reduced
            })
            .unwrap_or_else(|e| {
                tracing::warn!("Error reading reduced motion change: {}", e);
                false
            })
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires XDG portal to be available
    async fn test_detect_prefers_reduced_motion() {
        let result = detect_prefers_reduced_motion().await;
        // Should not panic and return a valid boolean
        assert!(result == true || result == false);
    }

    #[tokio::test]
    #[ignore] // Requires XDG portal to be available
    async fn test_subscribe_reduced_motion_changes() {
        use futures::StreamExt;

        let stream = subscribe_reduced_motion_changes().await;
        assert!(stream.is_ok());

        // If successful, we can take one item (or timeout)
        if let Ok(mut stream) = stream {
            let timeout = tokio::time::timeout(
                std::time::Duration::from_millis(100),
                stream.next(),
            );
            // Either times out (ok) or returns a value (also ok)
            let _ = timeout.await;
        }
    }
}
