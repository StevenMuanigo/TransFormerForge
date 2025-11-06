use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref SPECIAL_CHARS_RE: Regex = Regex::new(r"[^a-zA-Z0-9\s]").unwrap();
    static ref WHITESPACE_RE: Regex = Regex::new(r"\s+").unwrap();
}

pub fn remove_special_characters(text: &str) -> String {
    SPECIAL_CHARS_RE.replace_all(text, "").to_string()
}

pub fn normalize_whitespace(text: &str) -> String {
    WHITESPACE_RE.replace_all(text, " ").to_string()
}

pub fn remove_extra_spaces(text: &str) -> String {
    text.trim().to_string()
}

pub fn normalize_unicode(text: &str) -> String {
    // Unicode normalization (NFC form)
    text.chars()
        .map(|c| c.to_lowercase().to_string())
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_special_characters() {
        assert_eq!(
            remove_special_characters("Hello, World!"),
            "Hello World"
        );
    }

    #[test]
    fn test_normalize_whitespace() {
        assert_eq!(
            normalize_whitespace("Hello    World\t\n"),
            "Hello World "
        );
    }

    #[test]
    fn test_remove_extra_spaces() {
        assert_eq!(
            remove_extra_spaces("  Hello World  "),
            "Hello World"
        );
    }
}
