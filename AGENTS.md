# Agent Notes

Minimal Rust binary crate (`honyaku`). Translates between Japanese and English using an OpenAI-compatible LLM API.

## Runtime environment

- `dot.honyaku-env` is the local env template. It defines:
  - `HONYAKU_API_KEY`
  - `HONYAKU_ENDPOINT`
  - `HONYAKU_MODEL`
- Load them with your preferred tool, e.g.:
  - `export $(grep -v '^#' dot.honyaku-env | xargs)`
  - or pass a file explicitly with `--env ./my.env`.
- The app does **not** auto-load a `.env` in the current working directory; it only checks `$HOME/.env` and `--env <FILE>`.
- Env lookup precedence (highest to lowest):
  1. `--env <FILE>`
  2. Shell environment variables
  3. `${HOME}/.env`

## Build / run / test

Standard Cargo workflow applies:

- `cargo build --release`
- `cargo test`

### Run examples

```bash
# Auto-detect direction
cargo run -- "こんにちは"

# Force Japanese -> English
cargo run -- --je "こんにちは"

# Force English -> Japanese
cargo run -- --ej "Hello, world!"

# Use a specific env file
cargo run -- --env ./my.env "Hello"

# Translate from stdin
echo "Hello" | cargo run
```

## Notes

- The endpoint must be OpenAI-compatible (`$HONYAKU_ENDPOINT/chat/completions`).
- Direction flags are `--ej` and `--je` (not `-ej`/`-je`).
- Auto-detection rule: ≥70% hiragana/katakana/kanji → Japanese; 0% Japanese chars → English; otherwise ask the LLM.
- There are unit tests for language detection and env loading; end-to-end tests require a running LLM. The repo's `dot.honyaku-env` points to a local LM Studio endpoint for manual verification.
