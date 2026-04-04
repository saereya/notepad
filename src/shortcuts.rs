use iced::keyboard::{key::Named, Key};
use iced::widget::text_editor::{Binding, KeyPress, Status};

use crate::app::Message;

pub fn handle_key_binding(key_press: KeyPress) -> Option<Binding<Message>> {
    let focused = matches!(key_press.status, Status::Focused { .. });
    let ctrl = key_press.modifiers.command();
    let shift = key_press.modifiers.shift();

    if !focused {
        return None;
    }

    let custom = match (&key_press.key, ctrl, shift) {
        // File operations
        (Key::Character(c), true, false) if c.as_str() == "n" => {
            Some(Binding::Custom(Message::NewTab))
        }
        (Key::Character(c), true, false) if c.as_str() == "o" => {
            Some(Binding::Custom(Message::OpenFile))
        }
        (Key::Character(c), true, false) if c.as_str() == "s" => {
            Some(Binding::Custom(Message::SaveFile))
        }
        (Key::Character(c), true, true) if c.as_str() == "S" => {
            Some(Binding::Custom(Message::SaveFileAs))
        }

        // Undo/Redo
        (Key::Character(c), true, false) if c.as_str() == "z" => {
            Some(Binding::Custom(Message::Undo))
        }
        (Key::Character(c), true, true) if c.as_str() == "Z" => {
            Some(Binding::Custom(Message::Redo))
        }
        (Key::Character(c), true, false) if c.as_str() == "y" => {
            Some(Binding::Custom(Message::Redo))
        }

        // Find & Replace
        (Key::Character(c), true, false) if c.as_str() == "f" => {
            Some(Binding::Custom(Message::ToggleFindReplace))
        }
        (Key::Character(c), true, false) if c.as_str() == "h" => {
            Some(Binding::Custom(Message::ToggleFindReplace))
        }

        // Tab management
        (Key::Character(c), true, false) if c.as_str() == "w" => {
            Some(Binding::Custom(Message::CloseCurrentTab))
        }
        (Key::Named(Named::Tab), true, false) => {
            Some(Binding::Custom(Message::NextTab))
        }
        (Key::Named(Named::Tab), true, true) => {
            Some(Binding::Custom(Message::PrevTab))
        }

        // View
        (Key::Character(c), true, false) if c.as_str() == "l" => {
            Some(Binding::Custom(Message::ToggleLineNumbers))
        }

        _ => None,
    };

    // If we matched a custom shortcut, use it.
    // Otherwise, fall through to iced's default key handling.
    custom.or_else(|| Binding::from_key_press(key_press))
}
