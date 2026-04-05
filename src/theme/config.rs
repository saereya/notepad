use super::{dark_preset, light_preset, solarized_preset, ThemeConfig, ThemePreset};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// File format: only stores active preset and user customizations
#[derive(Serialize, Deserialize)]
struct ThemeConfigFile {
    active_preset: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    custom_presets: HashMap<String, ThemePreset>,
}

fn builtin_presets() -> HashMap<String, ThemePreset> {
    let mut presets = HashMap::new();
    presets.insert("light".to_string(), light_preset());
    presets.insert("dark".to_string(), dark_preset());
    presets.insert("solarized".to_string(), solarized_preset());
    presets
}

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
            Ok(contents) => match toml::from_str::<ThemeConfigFile>(&contents) {
                Ok(file_config) => {
                    let custom = file_config.custom_presets;
                    let mut presets = builtin_presets();
                    for (k, v) in &custom {
                        presets.insert(k.clone(), v.clone());
                    }
                    return ThemeConfig {
                        active_preset: file_config.active_preset,
                        presets,
                        custom_presets: custom,
                    };
                }
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
    let file_config = ThemeConfigFile {
        active_preset: config.active_preset.clone(),
        custom_presets: config.custom_presets.clone(),
    };
    let contents = toml::to_string_pretty(&file_config)?;
    std::fs::write(&path, contents)?;
    Ok(())
}
