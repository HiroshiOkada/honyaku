use std::io::{BufRead, Write};

use anyhow::{Context, Result};

use crate::cli::{Direction, StdinEncoding};
use crate::env::Config;
use crate::input::decode_stdin;
use crate::translate::translate;

#[derive(Debug, Default)]
struct ReplBuffer {
    lines: Vec<String>,
    blank_lines: usize,
}

impl ReplBuffer {
    fn push_line(&mut self, line: &str) -> Option<String> {
        let line = line.trim_end_matches(['\r', '\n']);
        if line.trim().is_empty() {
            self.blank_lines += 1;
            if self.blank_lines >= 2 {
                return self.take_text();
            }
        } else {
            self.blank_lines = 0;
            self.lines.push(line.to_string());
        }

        None
    }

    fn take_text(&mut self) -> Option<String> {
        self.blank_lines = 0;
        let text = self.lines.join("\n");
        self.lines.clear();

        if text.trim().is_empty() {
            None
        } else {
            Some(text)
        }
    }
}

pub async fn run_repl<R, W>(
    config: &Config,
    reader: R,
    mut writer: W,
    stdin_encoding: StdinEncoding,
    direction: Option<Direction>,
) -> Result<()>
where
    R: BufRead,
    W: Write,
{
    let mut repl_buffer = ReplBuffer::default();
    let mut reader = reader;
    let mut line_bytes = Vec::new();

    loop {
        line_bytes.clear();
        let bytes_read = reader
            .read_until(b'\n', &mut line_bytes)
            .context("failed to read REPL input")?;
        if bytes_read == 0 {
            if let Some(text) = repl_buffer.take_text() {
                translate_and_write(config, &mut writer, &text, direction).await?;
            }
            return Ok(());
        }

        let line = decode_stdin(&line_bytes, stdin_encoding)?;
        if let Some(text) = repl_buffer.push_line(&line) {
            translate_and_write(config, &mut writer, &text, direction).await?;
        }
    }
}

async fn translate_and_write<W: Write>(
    config: &Config,
    writer: &mut W,
    text: &str,
    direction: Option<Direction>,
) -> Result<()> {
    let translated = translate(config, text, direction).await?;
    writeln!(writer, "{}", translated).context("failed to write translation")?;
    writer.flush().context("failed to flush translation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emits_text_after_two_blank_lines() {
        let mut buffer = ReplBuffer::default();

        assert_eq!(buffer.push_line("Hello\n"), None);
        assert_eq!(buffer.push_line("world\n"), None);
        assert_eq!(buffer.push_line("\n"), None);
        assert_eq!(buffer.push_line("\n"), Some("Hello\nworld".to_string()));
    }

    #[test]
    fn ignores_empty_chunks() {
        let mut buffer = ReplBuffer::default();

        assert_eq!(buffer.push_line("\n"), None);
        assert_eq!(buffer.push_line("\n"), None);
        assert_eq!(buffer.push_line("Hello\n"), None);
        assert_eq!(buffer.push_line("\n"), None);
        assert_eq!(buffer.push_line("\n"), Some("Hello".to_string()));
    }

    #[test]
    fn flushes_pending_text_at_eof() {
        let mut buffer = ReplBuffer::default();

        assert_eq!(buffer.push_line("Hello\n"), None);
        assert_eq!(buffer.take_text(), Some("Hello".to_string()));
    }
}
