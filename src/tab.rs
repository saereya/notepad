use iced::widget::text_editor;
use std::path::PathBuf;
use uuid::Uuid;

use crate::file_io::{FileEncoding, LineEnding};
use crate::undo::{CursorPos, UndoStack};

pub struct Tab {
    pub id: Uuid,
    pub title: String,
    pub file_path: Option<PathBuf>,
    pub content: text_editor::Content,
    pub undo_stack: UndoStack,
    pub is_dirty: bool,
    pub encoding: FileEncoding,
    pub line_ending: LineEnding,
}

impl Tab {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: "Untitled".to_string(),
            file_path: None,
            content: text_editor::Content::new(),
            undo_stack: UndoStack::new(),
            is_dirty: false,
            encoding: FileEncoding::Utf8,
            line_ending: if cfg!(windows) {
                LineEnding::CrLf
            } else {
                LineEnding::Lf
            },
        }
    }

    pub fn from_file(path: PathBuf, content: String, encoding: FileEncoding, line_ending: LineEnding) -> Self {
        let title = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Untitled".to_string());

        Self {
            id: Uuid::new_v4(),
            title,
            file_path: Some(path),
            content: text_editor::Content::with_text(&content),
            undo_stack: UndoStack::from_text(&content),
            is_dirty: false,
            encoding,
            line_ending,
        }
    }

    pub fn display_title(&self) -> String {
        if self.is_dirty {
            format!("*{}", self.title)
        } else {
            self.title.clone()
        }
    }

    pub fn cursor_position(&self) -> (usize, usize) {
        let cursor = self.content.cursor();
        (cursor.position.line + 1, cursor.position.column + 1)
    }

    pub fn text(&self) -> String {
        self.content.text()
    }

    pub fn record_edit(&mut self) {
        let text = self.content.text();
        let cursor = self.content.cursor();
        self.undo_stack.record_edit(
            &text,
            CursorPos {
                line: cursor.position.line,
                col: cursor.position.column,
            },
        );
        self.is_dirty = true;
    }

    pub fn force_snapshot(&mut self) {
        let text = self.content.text();
        let cursor = self.content.cursor();
        self.undo_stack.push(
            &text,
            CursorPos {
                line: cursor.position.line,
                col: cursor.position.column,
            },
        );
    }

    pub fn undo(&mut self) {
        if let Some((text, _cursor)) = self.undo_stack.undo() {
            let text = text.to_string();
            self.content = text_editor::Content::with_text(&text);
            self.is_dirty = true;
        }
    }

    pub fn redo(&mut self) {
        if let Some((text, _cursor)) = self.undo_stack.redo() {
            let text = text.to_string();
            self.content = text_editor::Content::with_text(&text);
            self.is_dirty = true;
        }
    }

    pub fn set_content(&mut self, text: &str) {
        self.content = text_editor::Content::with_text(text);
        self.is_dirty = true;
    }

    pub fn mark_saved(&mut self) {
        self.is_dirty = false;
    }

    pub fn line_count(&self) -> usize {
        self.content.line_count()
    }
}
