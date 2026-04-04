# Notepad

A fast, lightweight tabbed text editor built with Rust and [iced](https://iced.rs).

## Features

- **Tabbed editing** - Open and edit multiple files at once
- **Find & Replace** - Search with case-sensitive toggle, replace one or all matches
- **Undo/Redo** - Snapshot-based undo history per tab
- **Encoding support** - Auto-detects and preserves UTF-8, UTF-8 BOM, UTF-16 LE/BE
- **Line endings** - Detects and preserves LF/CRLF
- **Theming** - Fully customizable colors and fonts via a built-in theme editor, stored as TOML
- **Line numbers** - Toggleable gutter
- **Word wrap** - Toggleable word wrapping

## Keyboard Shortcuts

| Shortcut | Action |
|---|---|
| Ctrl+N | New tab |
| Ctrl+O | Open file |
| Ctrl+S | Save |
| Ctrl+Shift+S | Save as |
| Ctrl+W | Close tab |
| Ctrl+Z | Undo |
| Ctrl+Shift+Z / Ctrl+Y | Redo |
| Ctrl+F / Ctrl+H | Find & Replace |
| Ctrl+Tab | Next tab |
| Ctrl+Shift+Tab | Previous tab |
| Ctrl+L | Toggle line numbers |

## Building

Requires Rust 1.85+ (edition 2024).

```sh
make
```

## Installing

Installs to `/usr/local/bin` by default:

```sh
sudo make install
```

To install somewhere else:

```sh
make install PREFIX=~/.local
```

To uninstall:

```sh
sudo make uninstall
```

## Themes

Themes are stored at `~/.config/notepad/theme.toml` (Linux/macOS) or the platform equivalent. Three presets are included: Light, Dark, and Solarized. Open the theme editor from the menu bar to customize.

## License

MIT
