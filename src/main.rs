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

use std::path::PathBuf;

const USAGE: &str = "\
Usage: notepad [OPTIONS] [FILE]...

Opens each FILE in its own tab. A FILE that doesn't exist yet starts an
empty buffer that will be written there on save.

Options:
  -h, --help       Print this help
  -V, --version    Print version
  --               Treat every remaining argument as a file name
";

fn main() -> iced::Result {
    let Some(paths) = parse_args() else {
        return Ok(());
    };

    iced::application(
        move || app::App::boot(paths.clone()),
        app::App::update,
        app::App::view,
    )
    .title(app::App::title)
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

/// The files to open, taken from the command line. Both `notepad a.txt b.txt`
/// and the desktop entry's `%F` (used by "Open With" in file managers) arrive
/// here. Returns `None` when the arguments asked for output only, in which
/// case no window should be opened.
fn parse_args() -> Option<Vec<PathBuf>> {
    let mut paths = Vec::new();
    let mut only_files = false;

    for arg in std::env::args_os().skip(1) {
        if !only_files {
            match arg.to_str() {
                Some("--") => {
                    only_files = true;
                    continue;
                }
                Some("-h" | "--help") => {
                    print!("{USAGE}");
                    return None;
                }
                Some("-V" | "--version") => {
                    println!("notepad {}", env!("CARGO_PKG_VERSION"));
                    return None;
                }
                // A lone "-" is a legal file name, anything longer is a typo'd
                // flag; guessing it's a file would create a junk buffer.
                Some(flag) if flag.starts_with('-') && flag.len() > 1 => {
                    eprint!("notepad: unrecognized option '{flag}'\n\n{USAGE}");
                    std::process::exit(2);
                }
                _ => {}
            }
        }

        paths.push(PathBuf::from(arg));
    }

    Some(paths)
}
