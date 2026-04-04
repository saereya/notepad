use iced::widget::{button, column, container, pick_list, row, slider, text,
                     text_input, scrollable};
use iced::{Alignment, Color, Element, Length};

use super::{SerColor, ThemeConfig, ThemePreset};

#[derive(Debug, Clone)]
pub enum ThemeDialogMessage {
    PresetSelected(String),
    BackgroundR(f32),
    BackgroundG(f32),
    BackgroundB(f32),
    ForegroundR(f32),
    ForegroundG(f32),
    ForegroundB(f32),
    SelectionR(f32),
    SelectionG(f32),
    SelectionB(f32),
    AccentR(f32),
    AccentG(f32),
    AccentB(f32),
    TabActiveR(f32),
    TabActiveG(f32),
    TabActiveB(f32),
    TabInactiveR(f32),
    TabInactiveG(f32),
    TabInactiveB(f32),
    FontFamilyChanged(String),
    FontSizeChanged(f32),
    Save,
    Cancel,
}

pub struct ThemeDialog {
    pub editing_preset: ThemePreset,
    pub selected_preset_name: String,
}

impl ThemeDialog {
    pub fn new(config: &ThemeConfig) -> Self {
        Self {
            editing_preset: config.active_preset().clone(),
            selected_preset_name: config.active_preset.clone(),
        }
    }

    pub fn update(&mut self, message: ThemeDialogMessage, config: &mut ThemeConfig) -> bool {
        match message {
            ThemeDialogMessage::PresetSelected(name) => {
                if let Some(preset) = config.presets.get(&name) {
                    self.editing_preset = preset.clone();
                    self.selected_preset_name = name;
                }
            }
            ThemeDialogMessage::BackgroundR(v) => self.editing_preset.background.r = v,
            ThemeDialogMessage::BackgroundG(v) => self.editing_preset.background.g = v,
            ThemeDialogMessage::BackgroundB(v) => self.editing_preset.background.b = v,
            ThemeDialogMessage::ForegroundR(v) => self.editing_preset.foreground.r = v,
            ThemeDialogMessage::ForegroundG(v) => self.editing_preset.foreground.g = v,
            ThemeDialogMessage::ForegroundB(v) => self.editing_preset.foreground.b = v,
            ThemeDialogMessage::SelectionR(v) => self.editing_preset.selection.r = v,
            ThemeDialogMessage::SelectionG(v) => self.editing_preset.selection.g = v,
            ThemeDialogMessage::SelectionB(v) => self.editing_preset.selection.b = v,
            ThemeDialogMessage::AccentR(v) => self.editing_preset.accent.r = v,
            ThemeDialogMessage::AccentG(v) => self.editing_preset.accent.g = v,
            ThemeDialogMessage::AccentB(v) => self.editing_preset.accent.b = v,
            ThemeDialogMessage::TabActiveR(v) => self.editing_preset.tab_active.r = v,
            ThemeDialogMessage::TabActiveG(v) => self.editing_preset.tab_active.g = v,
            ThemeDialogMessage::TabActiveB(v) => self.editing_preset.tab_active.b = v,
            ThemeDialogMessage::TabInactiveR(v) => self.editing_preset.tab_inactive.r = v,
            ThemeDialogMessage::TabInactiveG(v) => self.editing_preset.tab_inactive.g = v,
            ThemeDialogMessage::TabInactiveB(v) => self.editing_preset.tab_inactive.b = v,
            ThemeDialogMessage::FontFamilyChanged(f) => self.editing_preset.font_family = f,
            ThemeDialogMessage::FontSizeChanged(s) => self.editing_preset.font_size = s,
            ThemeDialogMessage::Save => {
                config
                    .presets
                    .insert(self.selected_preset_name.clone(), self.editing_preset.clone());
                config.active_preset = self.selected_preset_name.clone();
                let _ = super::config::save_config(config);
                return true; // close dialog
            }
            ThemeDialogMessage::Cancel => {
                return true; // close dialog
            }
        }
        false
    }

    pub fn view<'a>(&'a self, config: &'a ThemeConfig) -> Element<'a, ThemeDialogMessage> {
        let preset_names: Vec<String> = config.presets.keys().cloned().collect();
        let p = &self.editing_preset;

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

        let content = column![
            text("Theme Settings").size(20),
            divider(),
            preset_picker,
            divider(),
            font_row,
            divider(),
            color_section("Background", &p.background,
                ThemeDialogMessage::BackgroundR,
                ThemeDialogMessage::BackgroundG,
                ThemeDialogMessage::BackgroundB,
            ),
            color_section("Foreground", &p.foreground,
                ThemeDialogMessage::ForegroundR,
                ThemeDialogMessage::ForegroundG,
                ThemeDialogMessage::ForegroundB,
            ),
            color_section("Selection", &p.selection,
                ThemeDialogMessage::SelectionR,
                ThemeDialogMessage::SelectionG,
                ThemeDialogMessage::SelectionB,
            ),
            color_section("Accent", &p.accent,
                ThemeDialogMessage::AccentR,
                ThemeDialogMessage::AccentG,
                ThemeDialogMessage::AccentB,
            ),
            divider(),
            text("Tab Bar").size(16),
            color_section("Active Tab", &p.tab_active,
                ThemeDialogMessage::TabActiveR,
                ThemeDialogMessage::TabActiveG,
                ThemeDialogMessage::TabActiveB,
            ),
            color_section("Inactive Tab", &p.tab_inactive,
                ThemeDialogMessage::TabInactiveR,
                ThemeDialogMessage::TabInactiveG,
                ThemeDialogMessage::TabInactiveB,
            ),
            divider(),
            row![
                button("Save").on_press(ThemeDialogMessage::Save),
                button("Cancel").on_press(ThemeDialogMessage::Cancel),
            ]
            .spacing(10),
        ]
        .spacing(8)
        .padding(20)
        .width(500);

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.7))),
                ..Default::default()
            })
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

fn color_section<'a>(
    label: &'a str,
    color: &SerColor,
    on_r: fn(f32) -> ThemeDialogMessage,
    on_g: fn(f32) -> ThemeDialogMessage,
    on_b: fn(f32) -> ThemeDialogMessage,
) -> Element<'a, ThemeDialogMessage> {
    let preview_color = color.to_iced();
    row![
        text(label).width(100),
        text("R").width(15),
        slider(0.0..=1.0, color.r, on_r).width(80),
        text("G").width(15),
        slider(0.0..=1.0, color.g, on_g).width(80),
        text("B").width(15),
        slider(0.0..=1.0, color.b, on_b).width(80),
        container(text(""))
            .width(24)
            .height(24)
            .style(move |_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(preview_color)),
                border: iced::Border {
                    color: Color::from_rgb(0.5, 0.5, 0.5),
                    width: 1.0,
                    radius: 2.0.into(),
                },
                ..Default::default()
            }),
    ]
    .spacing(5)
    .align_y(Alignment::Center)
    .into()
}
