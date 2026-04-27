use std::collections::HashMap;

use crate::models::HistoryEntry;

// Count how many times each command was run.
// Normalizes to lowercase so "Git" and "git" don't get counted separately.
pub fn count_commands(entries: &[HistoryEntry]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();

    for entry in entries {
        let normalized = entry.command.trim().to_lowercase();
        if normalized.is_empty() {
            continue;
        }
        *counts.entry(normalized).or_insert(0) += 1;
    }

    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_basic_commands() {
        let entries = vec![
            HistoryEntry::new("ls", "ls"),
            HistoryEntry::new("ls -la", "ls"),
            HistoryEntry::new("git status", "git"),
        ];
        let counts = count_commands(&entries);
        assert_eq!(counts["ls"], 2);
        assert_eq!(counts["git"], 1);
    }

    #[test]
    fn normalizes_to_lowercase() {
        let entries = vec![
            HistoryEntry::new("Git status", "Git"),
            HistoryEntry::new("git commit", "git"),
            HistoryEntry::new("GIT push", "GIT"),
        ];
        let counts = count_commands(&entries);
        assert_eq!(counts["git"], 3);
    }

    #[test]
    fn trims_spaces_before_counting() {
        let entries = vec![
            HistoryEntry::new("  ls  ", "  ls  "),
            HistoryEntry::new("ls", "ls"),
        ];
        let counts = count_commands(&entries);
        assert_eq!(counts["ls"], 2);
    }

    #[test]
    fn skips_empty_commands() {
        let entries = vec![
            HistoryEntry::new("", ""),
            HistoryEntry::new("   ", "   "),
            HistoryEntry::new("ls", "ls"),
        ];
        let counts = count_commands(&entries);
        assert_eq!(counts.len(), 1);
        assert_eq!(counts["ls"], 1);
    }

    #[test]
    fn empty_entries_returns_empty_map() {
        let entries: Vec<HistoryEntry> = vec![];
        let counts = count_commands(&entries);
        assert!(counts.is_empty());
    }
}
