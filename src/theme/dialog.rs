use iced::widget::{button, column, container, pick_list, row, slider, text, text_input, scrollable};
use iced::{Alignment, Color, Element, Length};

use super::{SerColor, ThemeConfig, ThemePreset};

#[derive(Debug, Clone)]
pub enum ThemeDialogMessage {
    PresetSelected(String),
    BackgroundHex(String),
    ForegroundHex(String),
    SelectionHex(String),
    AccentHex(String),
    TabActiveHex(String),
    TabInactiveHex(String),
    TabCloseHex(String),
    StatusBarBgHex(String),
    StatusBarFgHex(String),
    GutterBgHex(String),
    GutterFgHex(String),
    FindHighlightHex(String),
    FontFamilyChanged(String),
    FontSizeChanged(f32),
    Undo,
    Redo,
    Save,
    Cancel,
}

pub struct ThemeDialog {
    pub editing_preset: ThemePreset,
    pub selected_preset_name: String,
    pub hex_fields: HexFields,
    /// Snapshot of original config so Cancel can restore it
    pub original_preset_name: String,
    pub original_preset: ThemePreset,
    /// Undo/redo stack for hex field edits
    undo_stack: Vec<(HexFields, ThemePreset)>,
    undo_pos: usize,
}

#[derive(Debug, Clone)]
pub struct HexFields {
    pub background: String,
    pub foreground: String,
    pub selection: String,
    pub accent: String,
    pub tab_active: String,
    pub tab_inactive: String,
    pub tab_close: String,
    pub status_bar_bg: String,
    pub status_bar_fg: String,
    pub gutter_bg: String,
    pub gutter_fg: String,
    pub find_highlight: String,
}

impl HexFields {
    fn from_preset(p: &ThemePreset) -> Self {
        Self {
            background: p.background.to_hex(),
            foreground: p.foreground.to_hex(),
            selection: p.selection.to_hex(),
            accent: p.accent.to_hex(),
            tab_active: p.tab_active.to_hex(),
            tab_inactive: p.tab_inactive.to_hex(),
            tab_close: p.tab_close_button.to_hex(),
            status_bar_bg: p.status_bar_background.to_hex(),
            status_bar_fg: p.status_bar_foreground.to_hex(),
            gutter_bg: p.gutter_background.to_hex(),
            gutter_fg: p.gutter_foreground.to_hex(),
            find_highlight: p.find_highlight.to_hex(),
        }
    }
}

impl ThemeDialog {
    pub fn new(config: &ThemeConfig) -> Self {
        let preset = config.active_preset().clone();
        let hex_fields = HexFields::from_preset(&preset);
        Self {
            editing_preset: preset.clone(),
            selected_preset_name: config.active_preset.clone(),
            hex_fields: hex_fields.clone(),
            original_preset_name: config.active_preset.clone(),
            original_preset: preset.clone(),
            undo_stack: vec![(hex_fields, preset)],
            undo_pos: 0,
        }
    }

    fn push_undo(&mut self) {
        // Truncate any redo history
        self.undo_stack.truncate(self.undo_pos + 1);
        self.undo_stack.push((self.hex_fields.clone(), self.editing_preset.clone()));
        if self.undo_stack.len() > 50 {
            self.undo_stack.remove(0);
        }
        self.undo_pos = self.undo_stack.len() - 1;
    }

    fn restore_snapshot(&mut self, config: &mut ThemeConfig) {
        let (hex, preset) = &self.undo_stack[self.undo_pos];
        self.hex_fields = hex.clone();
        self.editing_preset = preset.clone();
        config.presets.insert(self.selected_preset_name.clone(), self.editing_preset.clone());
        config.active_preset = self.selected_preset_name.clone();
    }

    pub fn update(&mut self, message: ThemeDialogMessage, config: &mut ThemeConfig) -> bool {
        match message {
            ThemeDialogMessage::PresetSelected(name) => {
                if let Some(preset) = config.presets.get(&name) {
                    self.push_undo();
                    self.editing_preset = preset.clone();
                    self.selected_preset_name = name.clone();
                    self.hex_fields = HexFields::from_preset(&self.editing_preset);
                    config.active_preset = name;
                }
                return false;
            }
            ThemeDialogMessage::Undo => {
                if self.undo_pos > 0 {
                    // Save current state if we're at the tip
                    if self.undo_pos == self.undo_stack.len() - 1 {
                        self.push_undo();
                        self.undo_pos -= 1; // push_undo moved us forward, go back one extra
                    }
                    self.undo_pos -= 1;
                    self.restore_snapshot(config);
                }
                return false;
            }
            ThemeDialogMessage::Redo => {
                if self.undo_pos + 1 < self.undo_stack.len() {
                    self.undo_pos += 1;
                    self.restore_snapshot(config);
                }
                return false;
            }
            ThemeDialogMessage::BackgroundHex(v) => {
                self.push_undo();
                self.hex_fields.background = v.clone();
                if let Some(c) = SerColor::from_hex(&v) {
                    self.editing_preset.background = c;
                }
            }
            ThemeDialogMessage::ForegroundHex(v) => {
                self.push_undo();
                self.hex_fields.foreground = v.clone();
                if let Some(c) = SerColor::from_hex(&v) {
                    self.editing_preset.foreground = c;
                }
            }
            ThemeDialogMessage::SelectionHex(v) => {
                self.push_undo();
                self.hex_fields.selection = v.clone();
                if let Some(c) = SerColor::from_hex(&v) {
                    self.editing_preset.selection = c;
                }
            }
            ThemeDialogMessage::AccentHex(v) => {
                self.push_undo();
                self.hex_fields.accent = v.clone();
                if let Some(c) = SerColor::from_hex(&v) {
                    self.editing_preset.accent = c;
                }
            }
            ThemeDialogMessage::TabActiveHex(v) => {
                self.push_undo();
                self.hex_fields.tab_active = v.clone();
                if let Some(c) = SerColor::from_hex(&v) {
                    self.editing_preset.tab_active = c;
                }
            }
            ThemeDialogMessage::TabInactiveHex(v) => {
                self.push_undo();
                self.hex_fields.tab_inactive = v.clone();
                if let Some(c) = SerColor::from_hex(&v) {
                    self.editing_preset.tab_inactive = c;
                }
            }
            ThemeDialogMessage::TabCloseHex(v) => {
                self.push_undo();
                self.hex_fields.tab_close = v.clone();
                if let Some(c) = SerColor::from_hex(&v) {
                    self.editing_preset.tab_close_button = c;
                }
            }
            ThemeDialogMessage::StatusBarBgHex(v) => {
                self.push_undo();
                self.hex_fields.status_bar_bg = v.clone();
                if let Some(c) = SerColor::from_hex(&v) {
                    self.editing_preset.status_bar_background = c;
                }
            }
            ThemeDialogMessage::StatusBarFgHex(v) => {
                self.push_undo();
                self.hex_fields.status_bar_fg = v.clone();
                if let Some(c) = SerColor::from_hex(&v) {
                    self.editing_preset.status_bar_foreground = c;
                }
            }
            ThemeDialogMessage::GutterBgHex(v) => {
                self.push_undo();
                self.hex_fields.gutter_bg = v.clone();
                if let Some(c) = SerColor::from_hex(&v) {
                    self.editing_preset.gutter_background = c;
                }
            }
            ThemeDialogMessage::GutterFgHex(v) => {
                self.push_undo();
                self.hex_fields.gutter_fg = v.clone();
                if let Some(c) = SerColor::from_hex(&v) {
                    self.editing_preset.gutter_foreground = c;
                }
            }
            ThemeDialogMessage::FindHighlightHex(v) => {
                self.push_undo();
                self.hex_fields.find_highlight = v.clone();
                if let Some(c) = SerColor::from_hex(&v) {
                    self.editing_preset.find_highlight = SerColor {
                        a: self.editing_preset.find_highlight.a,
                        ..c
                    };
                }
            }
            ThemeDialogMessage::FontFamilyChanged(f) => {
                self.push_undo();
                self.editing_preset.font_family = f;
            }
            ThemeDialogMessage::FontSizeChanged(s) => {
                self.push_undo();
                self.editing_preset.font_size = s;
            }
            ThemeDialogMessage::Save => {
                // Already applied live — persist to disk
                config
                    .presets
                    .insert(self.selected_preset_name.clone(), self.editing_preset.clone());
                config
                    .custom_presets
                    .insert(self.selected_preset_name.clone(), self.editing_preset.clone());
                config.active_preset = self.selected_preset_name.clone();
                let _ = super::config::save_config(config);
                return true;
            }
            ThemeDialogMessage::Cancel => {
                // Restore original state
                config
                    .presets
                    .insert(self.original_preset_name.clone(), self.original_preset.clone());
                config.active_preset = self.original_preset_name.clone();
                return true;
            }
        }
        // Apply live preview
        config
            .presets
            .insert(self.selected_preset_name.clone(), self.editing_preset.clone());
        config.active_preset = self.selected_preset_name.clone();
        false
    }

    pub fn view<'a>(&'a self, config: &'a ThemeConfig) -> Element<'a, ThemeDialogMessage> {
        let preset_names: Vec<String> = config.presets.keys().cloned().collect();
        let p = &self.editing_preset;
        let h = &self.hex_fields;

        let preset_picker = row![
            text("Preset:").width(100),
            pick_list(
                preset_names,
                Some(self.selected_preset_name.clone()),
                ThemeDialogMessage::PresetSelected,
            )
            .width(200),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let font_row = row![
            text("Font:").width(100),
            text_input("Font family", &p.font_family)
                .on_input(ThemeDialogMessage::FontFamilyChanged)
                .width(200),
            text("Size:").width(40),
            slider(8.0..=32.0, p.font_size, ThemeDialogMessage::FontSizeChanged).width(120),
            text(format!("{:.0}", p.font_size)).width(30),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let panel_bg = Color::from_rgba(0.12, 0.12, 0.15, 0.97);
        let panel_fg = Color::from_rgb(0.85, 0.85, 0.85);

        let content = column![
            text("Theme Settings").size(20).color(panel_fg),
            divider(),
            preset_picker,
            divider(),
            font_row,
            divider(),
            text("Colors").size(16).color(panel_fg),
            hex_color_row("Background", &h.background, &p.background, ThemeDialogMessage::BackgroundHex),
            hex_color_row("Foreground", &h.foreground, &p.foreground, ThemeDialogMessage::ForegroundHex),
            hex_color_row("Selection", &h.selection, &p.selection, ThemeDialogMessage::SelectionHex),
            hex_color_row("Accent", &h.accent, &p.accent, ThemeDialogMessage::AccentHex),
            hex_color_row("Find Highlight", &h.find_highlight, &p.find_highlight, ThemeDialogMessage::FindHighlightHex),
            divider(),
            text("Tab Bar").size(16).color(panel_fg),
            hex_color_row("Active Tab", &h.tab_active, &p.tab_active, ThemeDialogMessage::TabActiveHex),
            hex_color_row("Inactive Tab", &h.tab_inactive, &p.tab_inactive, ThemeDialogMessage::TabInactiveHex),
            hex_color_row("Close Button", &h.tab_close, &p.tab_close_button, ThemeDialogMessage::TabCloseHex),
            divider(),
            text("Status Bar").size(16).color(panel_fg),
            hex_color_row("Background", &h.status_bar_bg, &p.status_bar_background, ThemeDialogMessage::StatusBarBgHex),
            hex_color_row("Foreground", &h.status_bar_fg, &p.status_bar_foreground, ThemeDialogMessage::StatusBarFgHex),
            divider(),
            text("Gutter").size(16).color(panel_fg),
            hex_color_row("Background", &h.gutter_bg, &p.gutter_background, ThemeDialogMessage::GutterBgHex),
            hex_color_row("Foreground", &h.gutter_fg, &p.gutter_foreground, ThemeDialogMessage::GutterFgHex),
            divider(),
            row![
                button("Save").on_press(ThemeDialogMessage::Save),
                button("Cancel").on_press(ThemeDialogMessage::Cancel),
            ]
            .spacing(10),
        ]
        .spacing(6)
        .padding(20)
        .width(420);

        container(scrollable(
            container(content)
                .style(move |_theme: &iced::Theme| container::Style {
                    background: Some(iced::Background::Color(panel_bg)),
                    border: iced::Border {
                        color: Color::from_rgb(0.3, 0.3, 0.35),
                        width: 1.0,
                        radius: 8.0.into(),
                    },
                    shadow: iced::Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.5),
                        offset: iced::Vector::new(0.0, 4.0),
                        blur_radius: 20.0,
                    },
                    ..Default::default()
                })
        ))
        .max_height(600)
        .into()
    }
}

fn divider<'a>() -> Element<'a, ThemeDialogMessage> {
    container(text(""))
        .width(Length::Fill)
        .height(1)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.3, 0.3, 0.3))),
            ..Default::default()
        })
        .into()
}

fn hex_color_row<'a>(
    label: &'a str,
    hex_value: &str,
    color: &SerColor,
    on_change: fn(String) -> ThemeDialogMessage,
) -> Element<'a, ThemeDialogMessage> {
    let preview_color = color.to_iced();
    let hex_owned = hex_value.to_string();
    row![
        text(label).width(110).size(13),
        text_input("#RRGGBB", &hex_owned)
            .on_input(on_change)
            .width(90)
            .size(13),
        container(text(""))
            .width(20)
            .height(20)
            .style(move |_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(preview_color)),
                border: iced::Border {
                    color: Color::from_rgb(0.5, 0.5, 0.5),
                    width: 1.0,
                    radius: 3.0.into(),
                },
                ..Default::default()
            }),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}
