use super::ThemeConfig;
use std::path::PathBuf;

pub fn config_dir() -> PathBuf {
    let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("notepad")
}

pub fn config_path() -> PathBuf {
    config_dir().join("theme.toml")
}

pub fn load_config() -> ThemeConfig {
    let path = config_path();
    if path.exists() {
        match std::fs::read_to_string(&path) {
            Ok(contents) => match toml::from_str(&contents) {
                Ok(config) => return config,
                Err(e) => eprintln!("Failed to parse theme config: {e}"),
            },
            Err(e) => eprintln!("Failed to read theme config: {e}"),
        }
    }
    ThemeConfig::default()
}

pub fn save_config(config: &ThemeConfig) -> Result<(), Box<dyn std::error::Error>> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let contents = toml::to_string_pretty(config)?;
    std::fs::write(&path, contents)?;
    Ok(())
}
