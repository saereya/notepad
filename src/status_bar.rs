use iced::widget::{container, row, text};
use iced::{Alignment, Element, Length};

use crate::file_io::{FileEncoding, LineEnding};
use crate::theme::ThemePreset;

pub fn view<'a, M: 'a>(
    cursor_line: usize,
    cursor_col: usize,
    encoding: &FileEncoding,
    line_ending: &LineEnding,
    preset: &ThemePreset,
) -> Element<'a, M> {
    let bg = preset.status_bar_background.to_iced();
    let fg = preset.status_bar_foreground.to_iced();

    let position = text(format!("Ln {cursor_line}, Col {cursor_col}"))
        .size(13)
        .color(fg);
    let enc = text(encoding.to_string()).size(13).color(fg);
    let le = text(line_ending.to_string()).size(13).color(fg);

    container(
        row![position, text("    ").size(13), enc, text("    ").size(13), le]
            .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .padding([4, 12])
    .style(move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(bg)),
        ..Default::default()
    })
    .into()
}
