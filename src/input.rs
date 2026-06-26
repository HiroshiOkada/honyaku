use anyhow::{Context, Result};
use encoding_rs::{SHIFT_JIS, UTF_16BE, UTF_16LE};

use crate::cli::StdinEncoding;

pub fn decode_stdin(bytes: &[u8], encoding: StdinEncoding) -> Result<String> {
    match encoding {
        StdinEncoding::Auto => decode_auto(bytes),
        StdinEncoding::Utf8 => decode_utf8(bytes),
        StdinEncoding::Cp932 => decode_cp932(bytes),
    }
}

fn decode_auto(bytes: &[u8]) -> Result<String> {
    if let Some(text) = decode_bom(bytes)? {
        return Ok(text);
    }

    if let Ok(text) = std::str::from_utf8(bytes) {
        return Ok(text.to_string());
    }

    #[cfg(windows)]
    {
        return decode_cp932(bytes).context("stdin is neither valid UTF-8 nor CP932/Shift_JIS");
    }

    #[cfg(not(windows))]
    {
        anyhow::bail!("stdin is not valid UTF-8");
    }
}

fn decode_bom(bytes: &[u8]) -> Result<Option<String>> {
    if let Some(rest) = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]) {
        return decode_utf8(rest).map(Some);
    }

    if let Some(rest) = bytes.strip_prefix(&[0xFF, 0xFE]) {
        let (text, _, had_errors) = UTF_16LE.decode(rest);
        if had_errors {
            anyhow::bail!("stdin has a UTF-16LE BOM but contains invalid UTF-16");
        }
        return Ok(Some(text.into_owned()));
    }

    if let Some(rest) = bytes.strip_prefix(&[0xFE, 0xFF]) {
        let (text, _, had_errors) = UTF_16BE.decode(rest);
        if had_errors {
            anyhow::bail!("stdin has a UTF-16BE BOM but contains invalid UTF-16");
        }
        return Ok(Some(text.into_owned()));
    }

    Ok(None)
}

fn decode_utf8(bytes: &[u8]) -> Result<String> {
    std::str::from_utf8(bytes)
        .map(|text| text.to_string())
        .context("stdin is not valid UTF-8")
}

fn decode_cp932(bytes: &[u8]) -> Result<String> {
    let (text, _, had_errors) = SHIFT_JIS.decode(bytes);
    if had_errors {
        anyhow::bail!("stdin is not valid CP932/Shift_JIS");
    }
    Ok(text.into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_decodes_utf8() {
        assert_eq!(
            decode_stdin("こんにちは".as_bytes(), StdinEncoding::Auto).unwrap(),
            "こんにちは"
        );
    }

    #[test]
    fn auto_decodes_utf8_bom() {
        let mut bytes = vec![0xEF, 0xBB, 0xBF];
        bytes.extend_from_slice("こんにちは".as_bytes());
        assert_eq!(
            decode_stdin(&bytes, StdinEncoding::Auto).unwrap(),
            "こんにちは"
        );
    }

    #[test]
    fn auto_decodes_utf16le_bom() {
        let bytes = [
            0xFF, 0xFE, 0x53, 0x30, 0x93, 0x30, 0x6B, 0x30, 0x61, 0x30, 0x6F, 0x30,
        ];
        assert_eq!(
            decode_stdin(&bytes, StdinEncoding::Auto).unwrap(),
            "こんにちは"
        );
    }

    #[test]
    fn explicit_cp932_decodes_shift_jis() {
        let bytes = [0x82, 0xB1, 0x82, 0xF1, 0x82, 0xC9, 0x82, 0xBF, 0x82, 0xCD];
        assert_eq!(
            decode_stdin(&bytes, StdinEncoding::Cp932).unwrap(),
            "こんにちは"
        );
    }

    #[test]
    fn explicit_utf8_rejects_cp932() {
        let bytes = [0x82, 0xB1, 0x82, 0xF1];
        assert!(decode_stdin(&bytes, StdinEncoding::Utf8).is_err());
    }

    #[cfg(windows)]
    #[test]
    fn auto_falls_back_to_cp932_on_windows() {
        let bytes = [0x82, 0xB1, 0x82, 0xF1, 0x82, 0xC9, 0x82, 0xBF, 0x82, 0xCD];
        assert_eq!(
            decode_stdin(&bytes, StdinEncoding::Auto).unwrap(),
            "こんにちは"
        );
    }
}
