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
        .theme(app::App::theme)
        .window_size((900.0, 600.0))
        .centered()
        .run()
}
