// Integration tests for URL parsing
//
// Tests URL detection, validation, and text segment parsing.

use cosmic_applet_notifications::ui::url_parser::{parse_text, TextSegment};

#[test]
fn test_parse_text_no_urls() {
    let text = "This is plain text with no URLs";
    let segments = parse_text(text);

    assert_eq!(segments.len(), 1);
    match &segments[0] {
        TextSegment::Text(t) => assert_eq!(t, text),
        _ => panic!("Expected Text segment"),
    }
}

#[test]
fn test_parse_text_single_url() {
    let text = "Check out https://example.com for more info";
    let segments = parse_text(text);

    assert_eq!(segments.len(), 3);
    assert!(matches!(segments[0], TextSegment::Text(_)));
    assert!(matches!(segments[1], TextSegment::Link { .. }));
    assert!(matches!(segments[2], TextSegment::Text(_)));

    if let TextSegment::Link { url, .. } = &segments[1] {
        assert_eq!(url, "https://example.com");
    }
}

#[test]
fn test_parse_text_multiple_urls() {
    let text = "Visit https://example.com or http://test.org for details";
    let segments = parse_text(text);

    let url_count = segments
        .iter()
        .filter(|s| matches!(s, TextSegment::Link { .. }))
        .count();
    assert_eq!(url_count, 2);
}

#[test]
fn test_parse_text_url_at_start() {
    let text = "https://example.com is a great site";
    let segments = parse_text(text);

    assert!(matches!(segments[0], TextSegment::Link { .. }));

    if let TextSegment::Link { url, .. } = &segments[0] {
        assert_eq!(url, "https://example.com");
    }
}

#[test]
fn test_parse_text_url_at_end() {
    let text = "Visit our website at https://example.com";
    let segments = parse_text(text);

    assert!(matches!(segments.last().unwrap(), TextSegment::Link { .. }));
}

#[test]
fn test_parse_text_www_url() {
    let text = "Check www.example.com for updates";
    let segments = parse_text(text);

    let link_segment = segments.iter().find(|s| matches!(s, TextSegment::Link { .. }));
    assert!(link_segment.is_some());

    if let Some(TextSegment::Link { url, .. }) = link_segment {
        // www URLs should be prefixed with https://
        assert_eq!(url, "https://www.example.com");
    }
}

#[test]
fn test_parse_text_mailto_url() {
    let text = "Email us at mailto:test@example.com";
    let segments = parse_text(text);

    let link_segment = segments.iter().find(|s| matches!(s, TextSegment::Link { .. }));
    assert!(link_segment.is_some());

    if let Some(TextSegment::Link { url, .. }) = link_segment {
        assert_eq!(url, "mailto:test@example.com");
    }
}

#[test]
fn test_parse_text_ftp_url() {
    let text = "Download from ftp://ftp.example.com/file.txt";
    let segments = parse_text(text);

    let link_segment = segments.iter().find(|s| matches!(s, TextSegment::Link { .. }));
    assert!(link_segment.is_some());

    if let Some(TextSegment::Link { url, .. }) = link_segment {
        assert_eq!(url, "ftp://ftp.example.com/file.txt");
    }
}

#[test]
fn test_parse_text_dangerous_javascript_url_blocked() {
    let text = "Don't click javascript:alert('xss')";
    let segments = parse_text(text);

    // javascript: URLs should be rejected
    let has_link = segments.iter().any(|s| matches!(s, TextSegment::Link { .. }));
    assert!(!has_link);
}

#[test]
fn test_parse_text_dangerous_data_url_blocked() {
    let text = "Avoid data:text/html,<script>alert('xss')</script>";
    let segments = parse_text(text);

    // data: URLs should be rejected
    let has_link = segments.iter().any(|s| matches!(s, TextSegment::Link { .. }));
    assert!(!has_link);
}

#[test]
fn test_parse_text_file_url_blocked() {
    let text = "Local file: file:///etc/passwd";
    let segments = parse_text(text);

    // file: URLs should be rejected for security
    let has_link = segments.iter().any(|s| matches!(s, TextSegment::Link { .. }));
    assert!(!has_link);
}

#[test]
fn test_parse_text_url_with_query_params() {
    let text = "Search at https://example.com/search?q=test&lang=en";
    let segments = parse_text(text);

    let link_segment = segments.iter().find(|s| matches!(s, TextSegment::Link { .. }));
    assert!(link_segment.is_some());

    if let Some(TextSegment::Link { url, .. }) = link_segment {
        assert!(url.contains("?q=test&lang=en"));
    }
}

#[test]
fn test_parse_text_url_with_fragment() {
    let text = "Go to https://example.com/page#section";
    let segments = parse_text(text);

    let link_segment = segments.iter().find(|s| matches!(s, TextSegment::Link { .. }));
    assert!(link_segment.is_some());

    if let Some(TextSegment::Link { url, .. }) = link_segment {
        assert!(url.contains("#section"));
    }
}

#[test]
fn test_parse_text_url_with_port() {
    let text = "Server at https://example.com:8080/api";
    let segments = parse_text(text);

    let link_segment = segments.iter().find(|s| matches!(s, TextSegment::Link { .. }));
    assert!(link_segment.is_some());

    if let Some(TextSegment::Link { url, .. }) = link_segment {
        assert!(url.contains(":8080"));
    }
}

#[test]
fn test_parse_text_url_with_path() {
    let text = "Visit https://example.com/path/to/page.html";
    let segments = parse_text(text);

    let link_segment = segments.iter().find(|s| matches!(s, TextSegment::Link { .. }));
    assert!(link_segment.is_some());

    if let Some(TextSegment::Link { url, .. }) = link_segment {
        assert!(url.contains("/path/to/page.html"));
    }
}

#[test]
fn test_parse_text_consecutive_urls() {
    let text = "https://example.com https://test.org";
    let segments = parse_text(text);

    let url_count = segments
        .iter()
        .filter(|s| matches!(s, TextSegment::Link { .. }))
        .count();
    assert_eq!(url_count, 2);
}

#[test]
fn test_parse_text_url_in_parentheses() {
    let text = "See (https://example.com) for details";
    let segments = parse_text(text);

    let link_segment = segments.iter().find(|s| matches!(s, TextSegment::Link { .. }));
    assert!(link_segment.is_some());
}

#[test]
fn test_parse_text_url_with_special_chars() {
    let text = "API: https://api.example.com/v1/users?filter[name]=test";
    let segments = parse_text(text);

    let link_segment = segments.iter().find(|s| matches!(s, TextSegment::Link { .. }));
    assert!(link_segment.is_some());
}

#[test]
fn test_parse_text_empty_string() {
    let text = "";
    let segments = parse_text(text);

    assert_eq!(segments.len(), 1);
    match &segments[0] {
        TextSegment::Text(t) => assert_eq!(t, ""),
        _ => panic!("Expected Text segment"),
    }
}

#[test]
fn test_parse_text_only_url() {
    let text = "https://example.com";
    let segments = parse_text(text);

    assert_eq!(segments.len(), 1);
    assert!(matches!(segments[0], TextSegment::Link { .. }));
}

#[test]
fn test_text_segment_clone() {
    let segment = TextSegment::Link {
        text: "Example".to_string(),
        url: "https://example.com".to_string(),
    };

    let cloned = segment.clone();

    if let (
        TextSegment::Link {
            text: text1,
            url: url1,
        },
        TextSegment::Link {
            text: text2,
            url: url2,
        },
    ) = (&segment, &cloned)
    {
        assert_eq!(text1, text2);
        assert_eq!(url1, url2);
    } else {
        panic!("Expected Link segments");
    }
}

#[test]
fn test_text_segment_equality() {
    let seg1 = TextSegment::Text("test".to_string());
    let seg2 = TextSegment::Text("test".to_string());

    assert_eq!(seg1, seg2);

    let seg3 = TextSegment::Link {
        text: "Link".to_string(),
        url: "https://example.com".to_string(),
    };
    let seg4 = TextSegment::Link {
        text: "Link".to_string(),
        url: "https://example.com".to_string(),
    };

    assert_eq!(seg3, seg4);
}

#[test]
fn test_parse_text_mixed_content() {
    let text = "Hello https://example.com, email mailto:test@example.com, and www.test.org!";
    let segments = parse_text(text);

    let url_count = segments
        .iter()
        .filter(|s| matches!(s, TextSegment::Link { .. }))
        .count();

    // Should find 3 URLs
    assert_eq!(url_count, 3);
}
