use iced::widget::{button, column, container, row, scrollable, text, text_editor};
use iced::{Color, Element, Length, Task};

use crate::editor_view;
use crate::file_io::{self, OpenedFile};
use crate::find_replace::{FindReplaceMessage, FindReplaceState};
use crate::line_numbers;
use crate::status_bar;
use crate::tab::Tab;
use crate::theme::dialog::{ThemeDialog, ThemeDialogMessage};
use crate::theme::ThemeConfig;

#[derive(Debug, Clone)]
pub enum Message {
    // Editor
    EditorAction(text_editor::Action),

    // Tabs
    NewTab,
    CloseTab(usize),
    CloseCurrentTab,
    SwitchTab(usize),
    NextTab,
    PrevTab,

    // File I/O
    OpenFile,
    FileOpened(Result<OpenedFile, String>),
    SaveFile,
    SaveFileAs,
    FileSaved(Result<std::path::PathBuf, String>),

    // Edit
    Undo,
    Redo,

    // Find & Replace
    ToggleFindReplace,
    FindReplace(FindReplaceMessage),

    // View
    ToggleWordWrap,
    ToggleLineNumbers,

    // Theme
    OpenThemeDialog,
    ThemeDialog(ThemeDialogMessage),
}

pub struct App {
    tabs: Vec<Tab>,
    active_tab: usize,
    theme_config: ThemeConfig,
    show_theme_dialog: bool,
    theme_dialog: Option<ThemeDialog>,
    show_find_replace: bool,
    find_replace: FindReplaceState,
    word_wrap: bool,
    show_line_numbers: bool,
}

impl App {
    pub fn boot() -> (Self, Task<Message>) {
        let theme_config = crate::theme::config::load_config();
        let app = Self {
            tabs: vec![Tab::new()],
            active_tab: 0,
            theme_config,
            show_theme_dialog: false,
            theme_dialog: None,
            show_find_replace: false,
            find_replace: FindReplaceState::new(),
            word_wrap: true,
            show_line_numbers: true,
        };
        (app, Task::none())
    }

    pub fn theme(&self) -> iced::Theme {
        self.theme_config.to_iced_theme()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EditorAction(action) => {
                if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                    let is_edit = action.is_edit();
                    tab.content.perform(action);
                    if is_edit {
                        tab.record_edit();
                    }
                }
                Task::none()
            }

            // --- Tabs ---
            Message::NewTab => {
                self.tabs.push(Tab::new());
                self.active_tab = self.tabs.len() - 1;
                Task::none()
            }
            Message::CloseTab(idx) => {
                self.close_tab(idx);
                Task::none()
            }
            Message::CloseCurrentTab => {
                let idx = self.active_tab;
                self.close_tab(idx);
                Task::none()
            }
            Message::SwitchTab(idx) => {
                if idx < self.tabs.len() {
                    self.active_tab = idx;
                }
                Task::none()
            }
            Message::NextTab => {
                if !self.tabs.is_empty() {
                    self.active_tab = (self.active_tab + 1) % self.tabs.len();
                }
                Task::none()
            }
            Message::PrevTab => {
                if !self.tabs.is_empty() {
                    self.active_tab = if self.active_tab == 0 {
                        self.tabs.len() - 1
                    } else {
                        self.active_tab - 1
                    };
                }
                Task::none()
            }

            // --- File I/O ---
            Message::OpenFile => {
                Task::perform(file_io::open_file_dialog(), Message::FileOpened)
            }
            Message::FileOpened(Ok(opened)) => {
                let tab = Tab::from_file(
                    opened.path,
                    opened.content,
                    opened.encoding,
                    opened.line_ending,
                );
                self.tabs.push(tab);
                self.active_tab = self.tabs.len() - 1;
                Task::none()
            }
            Message::FileOpened(Err(e)) => {
                eprintln!("Error opening file: {e}");
                Task::none()
            }
            Message::SaveFile => {
                if let Some(tab) = self.tabs.get(self.active_tab) {
                    if let Some(path) = tab.file_path.clone() {
                        let content = tab.text();
                        let encoding = tab.encoding.clone();
                        let line_ending = tab.line_ending;
                        return Task::perform(
                            file_io::save_file(path, content, encoding, line_ending),
                            Message::FileSaved,
                        );
                    } else {
                        return self.update(Message::SaveFileAs);
                    }
                }
                Task::none()
            }
            Message::SaveFileAs => {
                if let Some(tab) = self.tabs.get(self.active_tab) {
                    let content = tab.text();
                    let encoding = tab.encoding.clone();
                    let line_ending = tab.line_ending;
                    return Task::perform(
                        file_io::save_file_as_dialog(content, encoding, line_ending),
                        Message::FileSaved,
                    );
                }
                Task::none()
            }
            Message::FileSaved(Ok(path)) => {
                if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                    tab.title = path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| "Untitled".to_string());
                    tab.file_path = Some(path);
                    tab.mark_saved();
                }
                Task::none()
            }
            Message::FileSaved(Err(e)) => {
                eprintln!("Error saving file: {e}");
                Task::none()
            }

            // --- Undo/Redo ---
            Message::Undo => {
                if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                    tab.force_snapshot(); // snapshot current state before undoing
                    tab.undo();
                }
                Task::none()
            }
            Message::Redo => {
                if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                    tab.redo();
                }
                Task::none()
            }

            // --- Find & Replace ---
            Message::ToggleFindReplace => {
                self.show_find_replace = !self.show_find_replace;
                Task::none()
            }
            Message::FindReplace(msg) => {
                match msg {
                    FindReplaceMessage::SearchTermChanged(term) => {
                        self.find_replace.search_term = term;
                        if let Some(tab) = self.tabs.get(self.active_tab) {
                            self.find_replace.find_all(&tab.text());
                        }
                    }
                    FindReplaceMessage::ReplaceTermChanged(term) => {
                        self.find_replace.replace_term = term;
                    }
                    FindReplaceMessage::FindNext => {
                        self.find_replace.find_next();
                    }
                    FindReplaceMessage::FindPrev => {
                        self.find_replace.find_prev();
                    }
                    FindReplaceMessage::ReplaceCurrent => {
                        if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                            if let Some(new_text) =
                                self.find_replace.replace_current_in_text(&tab.text())
                            {
                                tab.force_snapshot();
                                tab.set_content(&new_text);
                                self.find_replace.find_all(&tab.text());
                            }
                        }
                    }
                    FindReplaceMessage::ReplaceAll => {
                        if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                            let new_text = self.find_replace.replace_all_in_text(&tab.text());
                            tab.force_snapshot();
                            tab.set_content(&new_text);
                            self.find_replace.find_all(&tab.text());
                        }
                    }
                    FindReplaceMessage::ToggleCaseSensitive(v) => {
                        self.find_replace.case_sensitive = v;
                        if let Some(tab) = self.tabs.get(self.active_tab) {
                            self.find_replace.find_all(&tab.text());
                        }
                    }
                    FindReplaceMessage::Close => {
                        self.show_find_replace = false;
                    }
                }
                Task::none()
            }

            // --- View ---
            Message::ToggleWordWrap => {
                self.word_wrap = !self.word_wrap;
                Task::none()
            }
            Message::ToggleLineNumbers => {
                self.show_line_numbers = !self.show_line_numbers;
                Task::none()
            }

            // --- Theme ---
            Message::OpenThemeDialog => {
                self.show_theme_dialog = true;
                self.theme_dialog = Some(ThemeDialog::new(&self.theme_config));
                Task::none()
            }
            Message::ThemeDialog(msg) => {
                if let Some(dialog) = &mut self.theme_dialog {
                    let close = dialog.update(msg, &mut self.theme_config);
                    if close {
                        self.show_theme_dialog = false;
                        self.theme_dialog = None;
                    }
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        // If theme dialog is open, show it as overlay
        if self.show_theme_dialog {
            if let Some(dialog) = &self.theme_dialog {
                return dialog.view(&self.theme_config).map(Message::ThemeDialog);
            }
        }

        let preset = self.theme_config.active_preset();

        // --- Menu bar ---
        let menu_bar = self.view_menu_bar(preset);

        // --- Tab bar ---
        let tab_bar = self.view_tab_bar(preset);

        // --- Find & Replace bar ---
        let find_replace_bar: Option<Element<Message>> = if self.show_find_replace {
            Some(self.find_replace.view().map(Message::FindReplace))
        } else {
            None
        };

        // --- Editor area ---
        let editor_area: Element<Message> = if let Some(tab) = self.tabs.get(self.active_tab) {
            let editor = editor_view::view(tab, preset, self.word_wrap);

            if self.show_line_numbers {
                let gutter = line_numbers::view(tab.line_count(), preset.font_size, preset);
                row![gutter, editor].height(Length::Fill).into()
            } else {
                editor
            }
        } else {
            container(text("No open tabs"))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        };

        // --- Status bar ---
        let status_bar: Element<Message> = if let Some(tab) = self.tabs.get(self.active_tab) {
            let (line, col) = tab.cursor_position();
            status_bar::view(line, col, &tab.encoding, &tab.line_ending, preset)
        } else {
            container(text("")).into()
        };

        // --- Compose layout ---
        let mut layout = column![menu_bar, tab_bar]
            .width(Length::Fill)
            .height(Length::Fill);
        if let Some(fr) = find_replace_bar {
            layout = layout.push(fr);
        }
        layout = layout.push(editor_area).push(status_bar);

        layout.into()
    }

    fn view_menu_bar<'a>(&'a self, preset: &'a crate::theme::ThemePreset) -> Element<'a, Message> {
        let bg = preset.status_bar_background.to_iced();

        container(
            row![
                menu_button("New", Message::NewTab),
                menu_button("Open", Message::OpenFile),
                menu_button("Save", Message::SaveFile),
                menu_button("Save As", Message::SaveFileAs),
                text("  |  ").size(13),
                menu_button("Find", Message::ToggleFindReplace),
                text("  |  ").size(13),
                menu_button(
                    if self.word_wrap { "Wrap: On" } else { "Wrap: Off" },
                    Message::ToggleWordWrap,
                ),
                menu_button(
                    if self.show_line_numbers { "Ln#: On" } else { "Ln#: Off" },
                    Message::ToggleLineNumbers,
                ),
                text("  |  ").size(13),
                menu_button("Theme", Message::OpenThemeDialog),
            ]
            .spacing(2)
            .align_y(iced::Alignment::Center),
        )
        .width(Length::Fill)
        .padding([4, 8])
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(bg)),
            ..Default::default()
        })
        .into()
    }

    fn view_tab_bar<'a>(&'a self, preset: &'a crate::theme::ThemePreset) -> Element<'a, Message> {
        let active_bg = preset.tab_active.to_iced();
        let inactive_bg = preset.tab_inactive.to_iced();
        let fg = preset.foreground.to_iced();
        let close_color = preset.tab_close_button.to_iced();

        let tabs: Vec<Element<Message>> = self
            .tabs
            .iter()
            .enumerate()
            .map(|(i, tab)| {
                let is_active = i == self.active_tab;
                let bg = if is_active { active_bg } else { inactive_bg };

                let tab_label = button(text(tab.display_title()).size(13).color(fg))
                    .on_press(Message::SwitchTab(i))
                    .style(move |_theme: &iced::Theme, _status| button::Style {
                        background: Some(iced::Background::Color(bg)),
                        text_color: fg,
                        border: iced::Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: 2.0.into(),
                        },
                        ..Default::default()
                    })
                    .padding([4, 8]);

                let close_btn = button(text("x").size(11).color(close_color))
                    .on_press(Message::CloseTab(i))
                    .style(move |_theme: &iced::Theme, _status| button::Style {
                        background: None,
                        text_color: close_color,
                        ..Default::default()
                    })
                    .padding([2, 4]);

                container(row![tab_label, close_btn].spacing(2).align_y(iced::Alignment::Center))
                    .style(move |_theme: &iced::Theme| container::Style {
                        background: Some(iced::Background::Color(bg)),
                        border: iced::Border {
                            color: if is_active { fg } else { Color::TRANSPARENT },
                            width: if is_active { 0.0 } else { 0.0 },
                            radius: 3.0.into(),
                        },
                        ..Default::default()
                    })
                    .padding([0, 2])
                    .into()
            })
            .collect();

        container(scrollable(row(tabs).spacing(4).align_y(iced::Alignment::Center)).direction(
            scrollable::Direction::Horizontal(scrollable::Scrollbar::default()),
        ))
        .width(Length::Fill)
        .padding([4, 8])
        .into()
    }

    fn close_tab(&mut self, idx: usize) {
        if idx >= self.tabs.len() {
            return;
        }
        // TODO: prompt for unsaved changes
        self.tabs.remove(idx);
        if self.tabs.is_empty() {
            self.tabs.push(Tab::new());
            self.active_tab = 0;
        } else if self.active_tab >= self.tabs.len() {
            self.active_tab = self.tabs.len() - 1;
        } else if self.active_tab > idx {
            self.active_tab -= 1;
        }
    }
}

fn menu_button(label: &str, msg: Message) -> Element<'_, Message> {
    button(text(label).size(13))
        .on_press(msg)
        .padding([3, 8])
        .into()
}
