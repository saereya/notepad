/// Snapshot-based undo/redo stack.
///
/// Iced's `text_editor::Content` has no built-in undo, so we snapshot the full
/// text (plus cursor position) on each significant edit and restore from that.

#[derive(Debug, Clone)]
pub struct CursorPos {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
struct Snapshot {
    text: String,
    cursor: CursorPos,
}

pub struct UndoStack {
    snapshots: Vec<Snapshot>,
    current: usize,
    max_size: usize,
    edits_since_snapshot: usize,
    snapshot_threshold: usize,
}

impl UndoStack {
    pub fn new() -> Self {
        Self {
            snapshots: vec![Snapshot {
                text: String::new(),
                cursor: CursorPos { line: 0, col: 0 },
            }],
            current: 0,
            max_size: 100,
            edits_since_snapshot: 0,
            snapshot_threshold: 10,
        }
    }

    pub fn from_text(text: &str) -> Self {
        Self {
            snapshots: vec![Snapshot {
                text: text.to_string(),
                cursor: CursorPos { line: 0, col: 0 },
            }],
            current: 0,
            max_size: 100,
            edits_since_snapshot: 0,
            snapshot_threshold: 10,
        }
    }

    /// Call this on every edit action. A snapshot is taken every `snapshot_threshold` edits.
    /// Returns true if a snapshot was actually taken.
    pub fn record_edit(&mut self, text: &str, cursor: CursorPos) -> bool {
        self.edits_since_snapshot += 1;
        if self.edits_since_snapshot >= self.snapshot_threshold {
            self.push(text, cursor);
            true
        } else {
            false
        }
    }

    /// Force a snapshot (e.g. before save, or on paste).
    pub fn push(&mut self, text: &str, cursor: CursorPos) {
        self.edits_since_snapshot = 0;

        // Truncate any redo history
        self.snapshots.truncate(self.current + 1);

        self.snapshots.push(Snapshot {
            text: text.to_string(),
            cursor,
        });

        if self.snapshots.len() > self.max_size {
            self.snapshots.remove(0);
        }

        self.current = self.snapshots.len() - 1;
    }

    pub fn undo(&mut self) -> Option<(&str, &CursorPos)> {
        if self.current > 0 {
            self.current -= 1;
            let snap = &self.snapshots[self.current];
            Some((&snap.text, &snap.cursor))
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<(&str, &CursorPos)> {
        if self.current + 1 < self.snapshots.len() {
            self.current += 1;
            let snap = &self.snapshots[self.current];
            Some((&snap.text, &snap.cursor))
        } else {
            None
        }
    }

    pub fn reset(&mut self, text: &str) {
        self.snapshots.clear();
        self.snapshots.push(Snapshot {
            text: text.to_string(),
            cursor: CursorPos { line: 0, col: 0 },
        });
        self.current = 0;
        self.edits_since_snapshot = 0;
    }
}
