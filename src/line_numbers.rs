use iced::widget::{column, container, text};
use iced::{Element, Length};

use crate::theme::ThemePreset;

pub fn view<'a, M: 'a>(
    line_count: usize,
    font_size: f32,
    preset: &ThemePreset,
) -> Element<'a, M> {
    let bg = preset.gutter_background.to_iced();
    let fg = preset.gutter_foreground.to_iced();

    let width = digit_width(line_count);

    let lines: Vec<Element<'a, M>> = (1..=line_count)
        .map(|n| {
            text(format!("{n:>width$} "))
                .size(font_size)
                .color(fg)
                .font(iced::Font::MONOSPACE)
                .into()
        })
        .collect();

    container(column(lines))
        .width(Length::Shrink)
        .height(Length::Fill)
        .padding([0, 4])
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(bg)),
            ..Default::default()
        })
        .into()
}

fn digit_width(line_count: usize) -> usize {
    if line_count == 0 {
        1
    } else {
        ((line_count as f64).log10().floor() as usize) + 1
    }
}
