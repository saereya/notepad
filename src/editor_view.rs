use iced::widget::text::Wrapping;
use iced::widget::{container, text_editor};
use iced::{Color, Element, Length};

use crate::app::Message;
use crate::find_replace::{format_highlight, SearchHighlighter, SearchSettings};
use crate::shortcuts::handle_key_binding;
use crate::tab::Tab;
use crate::theme::ThemePreset;

pub const EDITOR_ID: &str = "main-editor";

pub fn view<'a>(
    tab: &'a Tab,
    preset: &ThemePreset,
    word_wrap: bool,
    search_settings: SearchSettings,
) -> Element<'a, Message> {
    let bg = preset.background.to_iced();
    let fg = preset.foreground.to_iced();
    let selection = preset.selection.to_iced();
    let wrapping = if word_wrap {
        Wrapping::WordOrGlyph
    } else {
        Wrapping::None
    };

    let editor = text_editor(&tab.content)
        .id(EDITOR_ID)
        .on_action(Message::EditorAction)
        .font(iced::Font::MONOSPACE)
        .size(preset.font_size)
        .height(Length::Fill)
        .wrapping(wrapping)
        .key_binding(handle_key_binding)
        .highlight_with::<SearchHighlighter>(search_settings, format_highlight)
        .style(move |_theme: &iced::Theme, _status| text_editor::Style {
            background: iced::Background::Color(bg),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            placeholder: Color::from_rgba(fg.r, fg.g, fg.b, 0.4),
            value: fg,
            selection,
        });

    container(editor)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
