/// One parsed line from bash history.
///
/// Holds the raw input line and the extracted base command.
/// The command is the first whitespace-delimited token (`git` from `git push -f`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryEntry {
    /// The full raw line exactly as it appeared in the history file.
    pub raw: String,
    /// The extracted base command (first token, no args).
    pub command: String,
}

impl HistoryEntry {
    /// Create a new entry from a raw line and its extracted command.
    pub fn new(raw: impl Into<String>, command: impl Into<String>) -> Self {
        Self {
            raw: raw.into(),
            command: command.into(),
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
}
