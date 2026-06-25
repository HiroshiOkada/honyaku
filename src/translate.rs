use anyhow::{Context, Result};

use crate::api::ask;
use crate::cli::Direction;
use crate::detect::{detect_language, language_to_direction};
use crate::env::Config;

const IDENTIFY_SYSTEM_PROMPT: &str = r#"Identify the language of the user's text.
Reply with exactly one of the following:
- "ja" if the text is primarily Japanese
- "en" if the text is primarily English
Do not output anything else."#;

const JA_TO_EN_SYSTEM_PROMPT: &str = r#"Translate the following Japanese text into natural English.
Output only the translation, with no explanations, notes, or markdown."#;

const EN_TO_JA_SYSTEM_PROMPT: &str = r#"Translate the following English text into natural Japanese.
Output only the translation, with no explanations, notes, or markdown."#;

/// Translate the input text, using an explicit direction or auto-detecting the source language.
pub async fn translate(config: &Config, text: &str, direction: Option<Direction>) -> Result<String> {
    let direction = match direction {
        Some(d) => d,
        None => determine_direction(config, text).await?,
    };

    let system_prompt = match direction {
        Direction::JapaneseToEnglish => JA_TO_EN_SYSTEM_PROMPT,
        Direction::EnglishToJapanese => EN_TO_JA_SYSTEM_PROMPT,
    };

    ask(config, system_prompt, text)
        .await
        .map(|s| s.trim().to_string())
        .context("translation failed")
}

async fn determine_direction(config: &Config, text: &str) -> Result<Direction> {
    if let Some(direction) = language_to_direction(detect_language(text)) {
        return Ok(direction);
    }

    let answer = ask(config, IDENTIFY_SYSTEM_PROMPT, text)
        .await
        .context("failed to identify language via LLM")?;

    let trimmed = answer.trim().to_lowercase();
    if trimmed.starts_with("ja") {
        Ok(Direction::JapaneseToEnglish)
    } else if trimmed.starts_with("en") {
        Ok(Direction::EnglishToJapanese)
    } else {
        anyhow::bail!(
            "could not determine source language from LLM response: {:?}",
            answer
        )
    }
}
