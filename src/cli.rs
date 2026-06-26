use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    EnglishToJapanese,
    JapaneseToEnglish,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StdinEncoding {
    Auto,
    Utf8,
    Cp932,
}

impl FromStr for StdinEncoding {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "auto" => Ok(Self::Auto),
            "utf-8" | "utf8" => Ok(Self::Utf8),
            "cp932" | "shift-jis" | "shift_jis" | "sjis" => Ok(Self::Cp932),
            _ => Err(format!(
                "invalid stdin encoding: {value}; expected auto, utf-8, or cp932"
            )),
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "honyaku",
    version,
    about = "Japanese/English translator via LLM"
)]
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

    /// Character encoding to use when reading stdin.
    #[arg(long, default_value = "auto", value_name = "ENCODING")]
    pub stdin_encoding: StdinEncoding,

    /// Read repeated inputs from stdin, translating after two blank lines.
    #[arg(long)]
    pub repl: bool,

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
