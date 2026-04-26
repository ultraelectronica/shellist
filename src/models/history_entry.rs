// One parsed line from bash history.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryEntry {
    pub raw: String,
    pub command: String,
}

impl HistoryEntry {
    // Build it. That's it.
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
