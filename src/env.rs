use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};

const ENV_API_KEY: &str = "HONYAKU_API_KEY";
const ENV_ENDPOINT: &str = "HONYAKU_ENDPOINT";
const ENV_MODEL: &str = "HONYAKU_MODEL";

/// Runtime configuration loaded from environment variables and optional `.env` files.
#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub endpoint: String,
    pub model: String,
}

/// Load configuration.
///
/// Precedence (highest to lowest):
/// 1. The file given by `env_file` (`--env`)
/// 2. Actual shell environment variables
/// 3. `${HOME}/.env`
pub fn load(env_file: Option<&Path>) -> Result<Config> {
    let mut vars = HashMap::new();

    // Lowest priority: ${HOME}/.env
    if let Some(home) = dirs::home_dir() {
        let path = home.join(".env");
        if path.exists() {
            load_into(&path, &mut vars)?;
        }
    }

    // Middle priority: shell environment variables.
    for key in [ENV_API_KEY, ENV_ENDPOINT, ENV_MODEL] {
        if let Ok(value) = std::env::var(key) {
            vars.insert(key.to_string(), value);
        }
    }

    // Highest priority: the file passed via --env.
    if let Some(file) = env_file {
        load_into(file, &mut vars)?;
    }

    Ok(Config {
        api_key: take_required(&mut vars, ENV_API_KEY)?,
        endpoint: take_required(&mut vars, ENV_ENDPOINT)?,
        model: take_required(&mut vars, ENV_MODEL)?,
    })
}

fn load_into(path: &Path, target: &mut HashMap<String, String>) -> Result<()> {
    let iter = dotenvy::from_path_iter(path)
        .with_context(|| format!("failed to read env file: {}", path.display()))?;

    for item in iter {
        let (key, value) = item.with_context(|| format!("failed to parse env file: {}", path.display()))?;
        target.insert(key, value);
    }

    Ok(())
}

fn take_required(vars: &mut HashMap<String, String>, key: &str) -> Result<String> {
    vars.remove(key)
        .filter(|v| !v.is_empty())
        .with_context(|| format!("{} is not set", key))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn temp_env_file(contents: &str) -> (tempfile::NamedTempFile, String) {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(contents.as_bytes()).unwrap();
        file.flush().unwrap();
        let path = file.path().to_path_buf();
        (file, path.to_string_lossy().to_string())
    }

    #[test]
    fn loads_from_env_file() {
        let (_file, path) = temp_env_file(
            "HONYAKU_API_KEY=key\nHONYAKU_ENDPOINT=http://localhost/v1\nHONYAKU_MODEL=m\n",
        );
        let config = load(Some(Path::new(&path))).unwrap();
        assert_eq!(config.api_key, "key");
        assert_eq!(config.endpoint, "http://localhost/v1");
        assert_eq!(config.model, "m");
    }

    #[test]
    fn env_file_overrides_home_env() {
        // This test does not set real home env vars; it only verifies that the --env file
        // takes precedence over values loaded from a simulated home .env file.
        let (home_file, home_path) = temp_env_file(
            "HONYAKU_API_KEY=home-key\nHONYAKU_ENDPOINT=http://home/v1\nHONYAKU_MODEL=home-model\n",
        );

        let (override_file, override_path) = temp_env_file(
            "HONYAKU_API_KEY=override-key\n",
        );

        // Since load() always looks at ${HOME}/.env, we can only test precedence by temporarily
        // swapping the mechanism. Instead, we verify load_into overrides existing keys.
        let mut vars = HashMap::new();
        load_into(home_file.path(), &mut vars).unwrap();
        assert_eq!(vars["HONYAKU_API_KEY"], "home-key");

        load_into(override_file.path(), &mut vars).unwrap();
        assert_eq!(vars["HONYAKU_API_KEY"], "override-key");

        // Keep paths used to avoid unused warnings (they are tied to the temp file lifetimes).
        let _ = home_path;
        let _ = override_path;
    }

    #[test]
    fn missing_required_key_errors() {
        let (_file, path) = temp_env_file("HONYAKU_API_KEY=key\nHONYAKU_ENDPOINT=http://localhost/v1\n");
        let err = load(Some(Path::new(&path))).unwrap_err();
        assert!(err.to_string().contains("HONYAKU_MODEL is not set"));
    }
}
