pub mod config;
pub mod dialog;

use iced::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl SerColor {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn to_iced(&self) -> Color {
        Color::from_rgba(self.r, self.g, self.b, self.a)
    }

    pub fn to_hex(&self) -> String {
        let r = (self.r * 255.0).round() as u8;
        let g = (self.g * 255.0).round() as u8;
        let b = (self.b * 255.0).round() as u8;
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        })
    }
}

impl From<SerColor> for Color {
    fn from(c: SerColor) -> Self {
        c.to_iced()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemePreset {
    pub name: String,
    pub background: SerColor,
    pub foreground: SerColor,
    pub selection: SerColor,
    pub accent: SerColor,
    pub font_family: String,
    pub font_size: f32,
    pub tab_active: SerColor,
    pub tab_inactive: SerColor,
    pub tab_close_button: SerColor,
    pub status_bar_background: SerColor,
    pub status_bar_foreground: SerColor,
    pub gutter_background: SerColor,
    pub gutter_foreground: SerColor,
    #[serde(default = "default_find_highlight")]
    pub find_highlight: SerColor,
}

fn default_find_highlight() -> SerColor {
    SerColor { r: 1.0, g: 0.8, b: 0.0, a: 0.45 }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub active_preset: String,
    pub presets: HashMap<String, ThemePreset>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        let mut presets = HashMap::new();
        presets.insert("light".to_string(), light_preset());
        presets.insert("dark".to_string(), dark_preset());
        presets.insert("solarized".to_string(), solarized_preset());

        Self {
            active_preset: "dark".to_string(),
            presets,
        }
    }
}

impl ThemeConfig {
    pub fn active_preset(&self) -> &ThemePreset {
        self.presets
            .get(&self.active_preset)
            .unwrap_or_else(|| self.presets.values().next().unwrap())
    }

    pub fn to_iced_theme(&self) -> iced::Theme {
        let preset = self.active_preset();
        let palette = iced::theme::Palette {
            background: preset.background.to_iced(),
            text: preset.foreground.to_iced(),
            primary: preset.accent.to_iced(),
            success: Color::from_rgb(0.3, 0.8, 0.3),
            warning: Color::from_rgb(0.9, 0.7, 0.2),
            danger: Color::from_rgb(0.8, 0.3, 0.3),
        };
        iced::Theme::custom_with_fn("Notepad".to_string(), palette, |palette| {
            iced::theme::palette::Extended::generate(palette)
        })
    }
}

pub fn light_preset() -> ThemePreset {
    ThemePreset {
        name: "Light".to_string(),
        background: SerColor::new(1.0, 1.0, 1.0),
        foreground: SerColor::new(0.1, 0.1, 0.1),
        selection: SerColor::new(0.7, 0.85, 1.0),
        accent: SerColor::new(0.2, 0.5, 0.9),
        font_family: "monospace".to_string(),
        font_size: 14.0,
        tab_active: SerColor::new(1.0, 1.0, 1.0),
        tab_inactive: SerColor::new(0.9, 0.9, 0.9),
        tab_close_button: SerColor::new(0.6, 0.6, 0.6),
        status_bar_background: SerColor::new(0.93, 0.93, 0.93),
        status_bar_foreground: SerColor::new(0.3, 0.3, 0.3),
        gutter_background: SerColor::new(0.95, 0.95, 0.95),
        gutter_foreground: SerColor::new(0.6, 0.6, 0.6),
        find_highlight: SerColor { r: 1.0, g: 0.9, b: 0.0, a: 0.4 },
    }
}

pub fn dark_preset() -> ThemePreset {
    ThemePreset {
        name: "Dark".to_string(),
        background: SerColor::new(0.15, 0.15, 0.18),
        foreground: SerColor::new(0.85, 0.85, 0.85),
        selection: SerColor::new(0.25, 0.35, 0.55),
        accent: SerColor::new(0.4, 0.6, 1.0),
        font_family: "monospace".to_string(),
        font_size: 14.0,
        tab_active: SerColor::new(0.2, 0.2, 0.25),
        tab_inactive: SerColor::new(0.12, 0.12, 0.15),
        tab_close_button: SerColor::new(0.5, 0.5, 0.5),
        status_bar_background: SerColor::new(0.1, 0.1, 0.12),
        status_bar_foreground: SerColor::new(0.6, 0.6, 0.6),
        gutter_background: SerColor::new(0.12, 0.12, 0.15),
        gutter_foreground: SerColor::new(0.4, 0.4, 0.4),
        find_highlight: SerColor { r: 1.0, g: 0.8, b: 0.0, a: 0.45 },
    }
}

pub fn solarized_preset() -> ThemePreset {
    ThemePreset {
        name: "Solarized".to_string(),
        background: SerColor::new(0.0, 0.17, 0.21),
        foreground: SerColor::new(0.51, 0.58, 0.59),
        selection: SerColor::new(0.07, 0.26, 0.30),
        accent: SerColor::new(0.15, 0.55, 0.82),
        font_family: "monospace".to_string(),
        font_size: 14.0,
        tab_active: SerColor::new(0.03, 0.21, 0.26),
        tab_inactive: SerColor::new(0.0, 0.14, 0.18),
        tab_close_button: SerColor::new(0.40, 0.48, 0.51),
        status_bar_background: SerColor::new(0.0, 0.14, 0.18),
        status_bar_foreground: SerColor::new(0.40, 0.48, 0.51),
        gutter_background: SerColor::new(0.0, 0.14, 0.18),
        gutter_foreground: SerColor::new(0.35, 0.43, 0.46),
        find_highlight: SerColor { r: 0.71, g: 0.54, b: 0.0, a: 0.45 },
    }
}
