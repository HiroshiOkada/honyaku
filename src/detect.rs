use crate::cli::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Japanese,
    English,
    Unknown,
}

/// Detect the language of the input text.
///
/// - 70% or more Japanese characters -> Japanese
/// - 0% Japanese characters -> English
/// - Otherwise -> Unknown (ask the LLM)
pub fn detect_language(text: &str) -> Language {
    let chars: Vec<char> = text.chars().collect();
    if chars.is_empty() {
        return Language::Unknown;
    }

    let ja_count = chars.iter().filter(|&&c| is_japanese_char(c)).count();
    let ratio = ja_count as f64 / chars.len() as f64;

    if ratio >= 0.7 {
        Language::Japanese
    } else if ratio == 0.0 {
        Language::English
    } else {
        Language::Unknown
    }
}

/// Convert a detected language into the default translation direction.
pub fn language_to_direction(lang: Language) -> Option<Direction> {
    match lang {
        Language::Japanese => Some(Direction::JapaneseToEnglish),
        Language::English => Some(Direction::EnglishToJapanese),
        Language::Unknown => None,
    }
}

fn is_japanese_char(c: char) -> bool {
    // Hiragana, Katakana, and CJK Unified Ideographs (including extension A and compatibility).
    ('\u{3040}' <= c && c <= '\u{309F}')
        || ('\u{30A0}' <= c && c <= '\u{30FF}')
        || ('\u{4E00}' <= c && c <= '\u{9FFF}')
        || ('\u{3400}' <= c && c <= '\u{4DBF}')
        || ('\u{F900}' <= c && c <= '\u{FAFF}')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pure_japanese_is_japanese() {
        assert_eq!(detect_language("こんにちは"), Language::Japanese);
        assert_eq!(detect_language("コンニチハ"), Language::Japanese);
        assert_eq!(detect_language("今日は"), Language::Japanese);
    }

    #[test]
    fn pure_english_is_english() {
        assert_eq!(detect_language("Hello, world!"), Language::English);
    }

    #[test]
    fn seventy_percent_japanese_is_japanese() {
        // 7 Japanese characters out of 10 total -> 70%
        assert_eq!(detect_language("あいうえおかきxyz"), Language::Japanese);
    }

    #[test]
    fn below_seventy_percent_is_unknown() {
        // 6 Japanese characters out of 10 -> 60%
        assert_eq!(detect_language("あいうえおかx y z w"), Language::Unknown);
    }

    #[test]
    fn mixed_with_no_japanese_is_english() {
        assert_eq!(detect_language("Hello 123 !@#"), Language::English);
    }

    #[test]
    fn empty_is_unknown() {
        assert_eq!(detect_language(""), Language::Unknown);
    }
}
