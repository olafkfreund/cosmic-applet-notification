// URL parsing module
//
// Detects and extracts URLs from notification text for rendering as clickable links.

use regex::Regex;
use std::sync::OnceLock;

/// Text segment - either plain text or a URL
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextSegment {
    /// Plain text segment
    Text(String),
    /// URL segment with display text and the URL
    Link { text: String, url: String },
}

/// URL regex pattern
///
/// Matches common URL formats:
/// - http:// and https:// URLs
/// - mailto: links
/// - ftp:// links
/// - URLs without scheme if they start with www.
///
/// Does NOT match javascript: or data: URLs for security.
fn url_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| {
        Regex::new(
            r#"(?x)
            (?P<url>
                # http/https/ftp with ://
                (?:https?|ftp)://[^\s<>"']+
                |
                # mailto: without // (mailto:user@domain.com)
                mailto:[^\s<>"']+
                |
                # www. without scheme
                www\.[^\s<>"']+
            )
        "#,
        )
        .expect("Failed to compile URL regex")
    })
}

/// Parse text into segments of plain text and URLs
///
/// Returns a vector of TextSegment enums representing the parsed text.
///
/// # Examples
///
/// ```
/// use cosmic_applet_notifications::ui::url_parser::parse_text;
///
/// let segments = parse_text("Check out https://example.com for more info!");
/// // Returns: [Text("Check out "), Link{...}, Text(" for more info!")]
/// ```
pub fn parse_text(text: &str) -> Vec<TextSegment> {
    let regex = url_regex();
    let mut segments = Vec::new();
    let mut last_end = 0;

    for capture in regex.captures_iter(text) {
        let url_match = capture.name("url").unwrap();
        let start = url_match.start();
        let end = url_match.end();
        let url = url_match.as_str();

        // Add any text before this URL
        if start > last_end {
            segments.push(TextSegment::Text(text[last_end..start].to_string()));
        }

        // Validate and add the URL
        if let Some(validated_url) = validate_url(url) {
            segments.push(TextSegment::Link {
                text: url.to_string(),
                url: validated_url,
            });
        } else {
            // If validation fails, treat as plain text
            segments.push(TextSegment::Text(url.to_string()));
        }

        last_end = end;
    }

    // Add any remaining text after the last URL
    if last_end < text.len() {
        segments.push(TextSegment::Text(text[last_end..].to_string()));
    }

    // If no URLs found, return the entire text as a single segment
    if segments.is_empty() {
        segments.push(TextSegment::Text(text.to_string()));
    }

    segments
}

/// Validate and normalize a URL
///
/// Returns None for dangerous or malformed URLs.
/// Normalizes URLs by adding scheme when missing (www.example.com -> https://www.example.com).
fn validate_url(url: &str) -> Option<String> {
    // Reject dangerous URL schemes
    let url_lower = url.to_lowercase();
    if url_lower.starts_with("javascript:")
        || url_lower.starts_with("data:")
        || url_lower.starts_with("vbscript:")
        || url_lower.starts_with("file:")
    {
        tracing::warn!("Rejected dangerous URL scheme: {}", url);
        return None;
    }

    // Add https:// prefix to www. URLs
    if url.starts_with("www.") {
        return Some(format!("https://{}", url));
    }

    // Return URL as-is if it already has a scheme
    Some(url.to_string())
}

/// Extract the first URL from text
///
/// Returns the first URL found in the text, or None if no URL is present.
/// This is optimized to stop at the first match without parsing the entire text.
///
/// # Examples
///
/// ```
/// use cosmic_applet_notifications::ui::url_parser::extract_first_url;
///
/// let url = extract_first_url("Check out https://example.com!");
/// assert_eq!(url, Some("https://example.com".to_string()));
/// ```
pub fn extract_first_url(text: &str) -> Option<String> {
    let regex = url_regex();

    regex
        .captures(text)
        .and_then(|cap| cap.name("url"))
        .map(|m| m.as_str())
        .and_then(validate_url)
}

/// Open a URL using the system's default handler
///
/// Uses xdg-open on Linux to open URLs in the default browser/application.
///
/// Returns:
/// - Ok(()) if the URL was opened successfully
/// - Err if the command failed
pub fn open_url(url: &str) -> Result<(), std::io::Error> {
    tracing::info!("Opening URL: {}", url);

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(url)
            .spawn()
            .map(|_| ())
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(url)
            .spawn()
            .map(|_| ())
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", url])
            .spawn()
            .map(|_| ())
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        tracing::error!("URL opening not supported on this platform");
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Platform not supported for opening URLs",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_text_no_urls() {
        let segments = parse_text("This is plain text without URLs");
        assert_eq!(segments.len(), 1);
        assert_eq!(
            segments[0],
            TextSegment::Text("This is plain text without URLs".to_string())
        );
    }

    #[test]
    fn test_parse_text_single_url() {
        let segments = parse_text("Check out https://example.com for info");

        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0], TextSegment::Text("Check out ".to_string()));
        assert!(
            matches!(&segments[1], TextSegment::Link { url, .. } if url == "https://example.com")
        );
        assert_eq!(segments[2], TextSegment::Text(" for info".to_string()));
    }

    #[test]
    fn test_parse_text_multiple_urls() {
        let segments = parse_text("Visit https://example.com or https://test.org");

        assert_eq!(segments.len(), 4);
        assert_eq!(segments[0], TextSegment::Text("Visit ".to_string()));
        assert!(
            matches!(&segments[1], TextSegment::Link { url, .. } if url == "https://example.com")
        );
        assert_eq!(segments[2], TextSegment::Text(" or ".to_string()));
        assert!(matches!(&segments[3], TextSegment::Link { url, .. } if url == "https://test.org"));
    }

    #[test]
    fn test_parse_text_www_url() {
        let segments = parse_text("Go to www.example.com");

        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0], TextSegment::Text("Go to ".to_string()));
        // www. URLs should be normalized to https://
        assert!(
            matches!(&segments[1], TextSegment::Link { url, .. } if url == "https://www.example.com")
        );
    }

    #[test]
    fn test_parse_text_mailto() {
        let segments = parse_text("Email me at mailto:user@example.com");

        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0], TextSegment::Text("Email me at ".to_string()));
        assert!(
            matches!(&segments[1], TextSegment::Link { url, .. } if url == "mailto:user@example.com")
        );
    }

    #[test]
    fn test_validate_url_dangerous_javascript() {
        let url = validate_url("javascript:alert('xss')");
        assert_eq!(url, None);
    }

    #[test]
    fn test_validate_url_dangerous_data() {
        let url = validate_url("data:text/html,<script>alert('xss')</script>");
        assert_eq!(url, None);
    }

    #[test]
    fn test_validate_url_dangerous_file() {
        let url = validate_url("file:///etc/passwd");
        assert_eq!(url, None);
    }

    #[test]
    fn test_validate_url_safe_http() {
        let url = validate_url("https://example.com");
        assert_eq!(url, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_validate_url_normalize_www() {
        let url = validate_url("www.example.com");
        assert_eq!(url, Some("https://www.example.com".to_string()));
    }

    #[test]
    fn test_parse_text_url_at_start() {
        let segments = parse_text("https://example.com is great");

        assert_eq!(segments.len(), 2);
        assert!(
            matches!(&segments[0], TextSegment::Link { url, .. } if url == "https://example.com")
        );
        assert_eq!(segments[1], TextSegment::Text(" is great".to_string()));
    }

    #[test]
    fn test_parse_text_url_at_end() {
        let segments = parse_text("Visit https://example.com");

        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0], TextSegment::Text("Visit ".to_string()));
        assert!(
            matches!(&segments[1], TextSegment::Link { url, .. } if url == "https://example.com")
        );
    }

    #[test]
    fn test_parse_text_ftp_url() {
        let segments = parse_text("Download from ftp://files.example.com/file.zip");

        assert!(segments.len() >= 2);
        assert!(segments
            .iter()
            .any(|s| matches!(s, TextSegment::Link { url, .. } if url.contains("ftp://"))));
    }
}
