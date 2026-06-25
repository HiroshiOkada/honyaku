use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    EnglishToJapanese,
    JapaneseToEnglish,
}

#[derive(Parser, Debug)]
#[command(name = "honyaku", version, about = "Japanese/English translator via LLM")]
pub struct Args {
    /// Force English -> Japanese translation.
    #[arg(long)]
    pub ej: bool,

    /// Force Japanese -> English translation.
    #[arg(long)]
    pub je: bool,

    /// Path to an environment file to load.
    #[arg(long, value_name = "FILE")]
    pub env: Option<PathBuf>,

    /// Text to translate. If omitted, stdin is read.
    pub text: Vec<String>,
}

impl Args {
    /// Resolve an explicit translation direction, or None for auto-detection.
    pub fn direction(&self) -> Option<Direction> {
        match (self.ej, self.je) {
            (true, _) => Some(Direction::EnglishToJapanese),
            (_, true) => Some(Direction::JapaneseToEnglish),
            _ => None,
        }
    }
}
