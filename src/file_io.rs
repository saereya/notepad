use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum FileEncoding {
    Utf8,
    Utf8Bom,
    Utf16Le,
    Utf16Be,
    Other(String),
}

impl std::fmt::Display for FileEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Utf8 => write!(f, "UTF-8"),
            Self::Utf8Bom => write!(f, "UTF-8 BOM"),
            Self::Utf16Le => write!(f, "UTF-16 LE"),
            Self::Utf16Be => write!(f, "UTF-16 BE"),
            Self::Other(name) => write!(f, "{name}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineEnding {
    Lf,
    CrLf,
}

impl std::fmt::Display for LineEnding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lf => write!(f, "LF"),
            Self::CrLf => write!(f, "CRLF"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OpenedFile {
    pub path: PathBuf,
    pub content: String,
    pub encoding: FileEncoding,
    pub line_ending: LineEnding,
}

pub async fn open_file_dialog() -> Result<OpenedFile, String> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Open File")
        .add_filter("Text Files", &["txt", "rs", "toml", "md", "json", "xml", "html", "css", "js", "py", "c", "h", "cpp", "java"])
        .add_filter("All Files", &["*"])
        .pick_file()
        .await
        .ok_or_else(|| "No file selected".to_string())?;

    let path = handle.path().to_path_buf();
    let bytes = tokio::fs::read(&path)
        .await
        .map_err(|e| format!("Failed to read file: {e}"))?;

    let (content, encoding) = decode_bytes(&bytes);
    let line_ending = detect_line_ending(&content);

    Ok(OpenedFile {
        path,
        content,
        encoding,
        line_ending,
    })
}

pub async fn save_file(
    path: PathBuf,
    content: String,
    _encoding: FileEncoding,
    line_ending: LineEnding,
) -> Result<PathBuf, String> {
    let text = match line_ending {
        LineEnding::CrLf => content.replace('\n', "\r\n"),
        LineEnding::Lf => content,
    };

    tokio::fs::write(&path, text.as_bytes())
        .await
        .map_err(|e| format!("Failed to save file: {e}"))?;

    Ok(path)
}

pub async fn save_file_as_dialog(
    content: String,
    encoding: FileEncoding,
    line_ending: LineEnding,
) -> Result<PathBuf, String> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Save As")
        .add_filter("Text Files", &["txt"])
        .add_filter("All Files", &["*"])
        .save_file()
        .await
        .ok_or_else(|| "No file selected".to_string())?;

    let path = handle.path().to_path_buf();
    save_file(path, content, encoding, line_ending).await
}

fn decode_bytes(bytes: &[u8]) -> (String, FileEncoding) {
    // Check BOM
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return (
            String::from_utf8_lossy(&bytes[3..]).into_owned(),
            FileEncoding::Utf8Bom,
        );
    }
    if bytes.starts_with(&[0xFF, 0xFE]) {
        let (decoded, _, _) = encoding_rs::UTF_16LE.decode(bytes);
        return (decoded.into_owned(), FileEncoding::Utf16Le);
    }
    if bytes.starts_with(&[0xFE, 0xFF]) {
        let (decoded, _, _) = encoding_rs::UTF_16BE.decode(bytes);
        return (decoded.into_owned(), FileEncoding::Utf16Be);
    }

    // Try UTF-8 first
    if let Ok(s) = std::str::from_utf8(bytes) {
        return (s.to_string(), FileEncoding::Utf8);
    }

    // Auto-detect with chardetng
    let mut detector = chardetng::EncodingDetector::new();
    detector.feed(bytes, true);
    let encoding = detector.guess(None, true);
    let (decoded, _, _) = encoding.decode(bytes);
    (
        decoded.into_owned(),
        FileEncoding::Other(encoding.name().to_string()),
    )
}

fn detect_line_ending(text: &str) -> LineEnding {
    if text.contains("\r\n") {
        LineEnding::CrLf
    } else {
        LineEnding::Lf
    }
}
