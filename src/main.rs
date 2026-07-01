#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod editor_view;
mod file_io;
mod find_replace;
mod line_numbers;
mod shortcuts;
mod status_bar;
mod tab;
mod theme;
mod undo;

fn main() -> iced::Result {
    iced::application(app::App::boot, app::App::update, app::App::view)
        .title("Notepad")
        // cosmic-text maps the generic `SansSerif` family to a font named
        // "Open Sans", which isn't installed on most systems. Point the default
        // font at the Fira Sans we embed via the `fira-sans` feature so UI text
        // renders regardless of what fonts the host has.
        .default_font(iced::Font::with_name("Fira Sans"))
        .theme(app::App::theme)
        .subscription(app::App::subscription)
        .window_size((900.0, 600.0))
        .centered()
        .run()
}
