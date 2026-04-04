use iced::advanced::text::highlighter::{self, Highlighter};
use iced::widget::{button, container, row, text, text_input, toggler};
use iced::{Alignment, Color, Element, Length};
use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct SearchSettings {
    pub search_term: String,
    pub case_sensitive: bool,
    pub highlight_color: Color,
}

pub struct SearchHighlighter {
    settings: SearchSettings,
    current_line: usize,
}

impl Highlighter for SearchHighlighter {
    type Settings = SearchSettings;
    type Highlight = Color;
    type Iterator<'a> = std::vec::IntoIter<(Range<usize>, Color)>;

    fn new(settings: &Self::Settings) -> Self {
        Self {
            settings: settings.clone(),
            current_line: 0,
        }
    }

    fn update(&mut self, new_settings: &Self::Settings) {
        self.settings = new_settings.clone();
        self.current_line = 0;
    }

    fn change_line(&mut self, line: usize) {
        self.current_line = line;
    }

    fn highlight_line(&mut self, line: &str) -> Self::Iterator<'_> {
        let mut highlights = Vec::new();

        if !self.settings.search_term.is_empty() {
            let (haystack, needle) = if self.settings.case_sensitive {
                (line.to_string(), self.settings.search_term.clone())
            } else {
                (line.to_lowercase(), self.settings.search_term.to_lowercase())
            };

            for (i, _) in haystack.match_indices(&needle) {
                highlights.push((i..i + needle.len(), self.settings.highlight_color));
            }
        }

        self.current_line += 1;
        highlights.into_iter()
    }

    fn current_line(&self) -> usize {
        self.current_line
    }
}

pub fn format_highlight(
    color: &Color,
    _theme: &iced::Theme,
) -> highlighter::Format<iced::Font> {
    highlighter::Format {
        color: Some(*color),
        font: None,
    }
}

#[derive(Debug, Clone)]
pub enum FindReplaceMessage {
    SearchTermChanged(String),
    ReplaceTermChanged(String),
    FindNext,
    FindPrev,
    ReplaceCurrent,
    ReplaceAll,
    ToggleCaseSensitive(bool),
    Close,
}

pub struct FindReplaceState {
    pub search_term: String,
    pub replace_term: String,
    pub case_sensitive: bool,
    pub matches: Vec<(usize, usize)>, // (line, col) of each match
    pub current_match: Option<usize>,
}

impl FindReplaceState {
    pub fn new() -> Self {
        Self {
            search_term: String::new(),
            replace_term: String::new(),
            case_sensitive: false,
            matches: Vec::new(),
            current_match: None,
        }
    }

    pub fn find_all(&mut self, text: &str, cursor: Option<(usize, usize)>) {
        self.matches.clear();
        self.current_match = None;

        if self.search_term.is_empty() {
            return;
        }

        let (haystack, needle) = if self.case_sensitive {
            (text.to_string(), self.search_term.clone())
        } else {
            (text.to_lowercase(), self.search_term.to_lowercase())
        };

        let mut line = 0;
        let mut col = 0;
        let mut byte_idx = 0;

        for (i, _) in haystack.match_indices(&needle) {
            // Advance line/col tracker to position i
            while byte_idx < i {
                if haystack.as_bytes()[byte_idx] == b'\n' {
                    line += 1;
                    col = 0;
                } else {
                    col += 1;
                }
                byte_idx += 1;
            }
            self.matches.push((line, col));
        }

        if !self.matches.is_empty() {
            if let Some((cursor_line, cursor_col)) = cursor {
                // Find first match at or after cursor position
                self.current_match = self
                    .matches
                    .iter()
                    .position(|&(l, c)| l > cursor_line || (l == cursor_line && c >= cursor_col))
                    .or(Some(0));
            } else {
                self.current_match = Some(0);
            }
        }
    }

    pub fn find_next(&mut self) {
        if self.matches.is_empty() {
            return;
        }
        self.current_match = Some(match self.current_match {
            Some(i) => (i + 1) % self.matches.len(),
            None => 0,
        });
    }

    pub fn find_prev(&mut self) {
        if self.matches.is_empty() {
            return;
        }
        self.current_match = Some(match self.current_match {
            Some(0) => self.matches.len() - 1,
            Some(i) => i - 1,
            None => self.matches.len() - 1,
        });
    }

    pub fn current_match_position(&self) -> Option<(usize, usize)> {
        self.current_match.map(|i| self.matches[i])
    }

    pub fn match_count(&self) -> usize {
        self.matches.len()
    }

    pub fn replace_current_in_text(&self, text: &str) -> Option<String> {
        if self.search_term.is_empty() {
            return None;
        }

        let (line, col) = self.current_match_position()?;

        // Find the byte offset of the match
        let mut current_line = 0;
        let mut current_col = 0;
        let mut target_byte = None;

        for (i, ch) in text.char_indices() {
            if current_line == line && current_col == col {
                target_byte = Some(i);
                break;
            }
            if ch == '\n' {
                current_line += 1;
                current_col = 0;
            } else {
                current_col += 1;
            }
        }

        let byte_offset = target_byte?;
        let end = byte_offset + self.search_term.len();

        let mut result = String::with_capacity(text.len());
        result.push_str(&text[..byte_offset]);
        result.push_str(&self.replace_term);
        result.push_str(&text[end..]);

        Some(result)
    }

    pub fn replace_all_in_text(&self, text: &str) -> String {
        if self.search_term.is_empty() {
            return text.to_string();
        }

        if self.case_sensitive {
            text.replace(&self.search_term, &self.replace_term)
        } else {
            let lower = text.to_lowercase();
            let needle = self.search_term.to_lowercase();
            let mut result = String::with_capacity(text.len());
            let mut last = 0;
            for (i, _) in lower.match_indices(&needle) {
                result.push_str(&text[last..i]);
                result.push_str(&self.replace_term);
                last = i + self.search_term.len();
            }
            result.push_str(&text[last..]);
            result
        }
    }

    pub fn view(&self, preset: &crate::theme::ThemePreset) -> Element<'_, FindReplaceMessage> {
        let match_info = if self.search_term.is_empty() {
            String::new()
        } else if self.matches.is_empty() {
            "No matches".to_string()
        } else {
            format!(
                "{}/{}",
                self.current_match.map(|i| i + 1).unwrap_or(0),
                self.matches.len()
            )
        };

        let fg = preset.foreground.to_iced();
        let panel_bg = preset.status_bar_background.to_iced();
        let border_color = Color::from_rgba(fg.r, fg.g, fg.b, 0.2);

        let search_row = row![
            text_input("Search...", &self.search_term)
                .on_input(FindReplaceMessage::SearchTermChanged)
                .width(180),
            button(text("◀").size(12)).on_press(FindReplaceMessage::FindPrev),
            button(text("▶").size(12)).on_press(FindReplaceMessage::FindNext),
            text(match_info).size(12).color(fg),
        ]
        .spacing(4)
        .align_y(Alignment::Center);

        let replace_row = row![
            text_input("Replace...", &self.replace_term)
                .on_input(FindReplaceMessage::ReplaceTermChanged)
                .width(180),
            button(text("Replace").size(12)).on_press(FindReplaceMessage::ReplaceCurrent),
            button(text("All").size(12)).on_press(FindReplaceMessage::ReplaceAll),
            toggler(self.case_sensitive)
                .label("Aa")
                .on_toggle(FindReplaceMessage::ToggleCaseSensitive)
                .size(14.0)
                .width(Length::Shrink),
            button(text("✕").size(12)).on_press(FindReplaceMessage::Close),
        ]
        .spacing(4)
        .align_y(Alignment::Center);

        container(
            iced::widget::column![search_row, replace_row]
                .spacing(4)
                .padding(6),
        )
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(panel_bg)),
            border: iced::Border {
                color: border_color,
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
            ..Default::default()
        })
        .width(Length::Shrink)
        .into()
    }
}
