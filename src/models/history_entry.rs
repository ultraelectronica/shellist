/// One parsed line from a shell history file.
///
/// Holds the raw input line, the extracted base command, and an optional
/// timestamp (Unix epoch seconds) when the shell records one (zsh extended,
/// fish, or bash with `HISTTIMEFORMAT`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryEntry {
    /// The full raw command line as it appeared in the history file.
    pub raw: String,
    /// The extracted base command (first token, no args).
    pub command: String,
    /// Unix epoch seconds, when the shell records one.
    pub timestamp: Option<u64>,
}

impl HistoryEntry {
    /// Create a new entry from a raw line and its extracted command (no timestamp).
    pub fn new(raw: impl Into<String>, command: impl Into<String>) -> Self {
        Self {
            raw: raw.into(),
            command: command.into(),
            timestamp: None,
        }
    }

    /// Attach a timestamp to this entry (builder style).
    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    /// Create an entry with all fields, including an optional timestamp.
    pub fn from_parts(
        raw: impl Into<String>,
        command: impl Into<String>,
        timestamp: Option<u64>,
    ) -> Self {
        Self {
            raw: raw.into(),
            command: command.into(),
            timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn history_entry_creation() {
        let entry = HistoryEntry::new("git status", "git");
        assert_eq!(entry.raw, "git status");
        assert_eq!(entry.command, "git");
    }

    #[test]
    fn history_entry_clone_equality() {
        let entry = HistoryEntry::new("ls -la", "ls");
        let cloned = entry.clone();
        assert_eq!(entry, cloned);
    }

    #[test]
    fn new_has_no_timestamp() {
        let entry = HistoryEntry::new("git status", "git");
        assert_eq!(entry.timestamp, None);
    }

    #[test]
    fn with_timestamp_sets_it() {
        let entry = HistoryEntry::new("git status", "git").with_timestamp(123);
        assert_eq!(entry.timestamp, Some(123));
    }

    #[test]
    fn from_parts_builds_full_entry() {
        let entry = HistoryEntry::from_parts("ls", "ls", Some(7));
        assert_eq!(entry.command, "ls");
        assert_eq!(entry.timestamp, Some(7));
    }
}
