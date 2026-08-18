use std::collections::HashMap;

use crate::models::HistoryEntry;

/// Count how many times each command appears.
///
/// Normalizes to lowercase so `Git` and `git` merge into one count.
/// Skips empty commands.
///
/// ```rust
/// use shellist::{HistoryEntry, count_commands};
/// let entries = [
///     HistoryEntry::new("ls -la", "ls"),
///     HistoryEntry::new("ls", "ls"),
///     HistoryEntry::new("Git status", "Git"),
/// ];
/// let counts = count_commands(&entries);
/// assert_eq!(counts.get("ls"), Some(&2));
/// assert_eq!(counts.get("git"), Some(&1));
/// ```
pub fn count_commands(entries: &[HistoryEntry]) -> HashMap<String, usize> {
    count_commands_at_depth(entries, 1)
}

/// The lowercased count key for an entry at the given `depth`, if any.
///
/// `depth = 1` uses the base command; deeper keys take the first `depth`
/// whitespace tokens of the raw line.
pub fn command_key(entry: &HistoryEntry, depth: usize) -> Option<String> {
    if depth == 0 {
        return None;
    }
    if depth == 1 {
        let key = entry.command.trim().to_lowercase();
        return (!key.is_empty()).then_some(key);
    }
    let mut tokens = entry.raw.split_whitespace().take(depth);
    let mut s = String::new();
    match tokens.next() {
        Some(first) => s.push_str(first),
        None => return None,
    }
    for t in tokens {
        s.push(' ');
        s.push_str(t);
    }
    (!s.is_empty()).then(|| s.to_lowercase())
}

/// Count commands treating the first `depth` whitespace tokens as the key.
///
/// `depth = 1` is the default (first token only). `depth = 2` ranks
/// multi-word commands like `git commit` and `git push` separately.
///
/// ```rust
/// use shellist::{HistoryEntry, count_commands_at_depth};
/// let entries = [
///     HistoryEntry::new("git commit", "git"),
///     HistoryEntry::new("git push", "git"),
///     HistoryEntry::new("git commit", "git"),
/// ];
/// let counts = count_commands_at_depth(&entries, 2);
/// assert_eq!(counts.get("git commit"), Some(&2));
/// assert_eq!(counts.get("git push"), Some(&1));
/// ```
pub fn count_commands_at_depth(entries: &[HistoryEntry], depth: usize) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for entry in entries {
        if let Some(key) = command_key(entry, depth) {
            *counts.entry(key).or_insert(0) += 1;
        }
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

    #[test]
    fn depth_one_matches_count_commands() {
        let entries = vec![
            HistoryEntry::new("ls -la", "ls"),
            HistoryEntry::new("git status", "git"),
            HistoryEntry::new("ls", "ls"),
        ];
        let d1 = count_commands_at_depth(&entries, 1);
        assert_eq!(d1["ls"], 2);
        assert_eq!(d1["git"], 1);
    }

    #[test]
    fn depth_two_separates_subcommands() {
        let entries = vec![
            HistoryEntry::new("git commit", "git"),
            HistoryEntry::new("git push", "git"),
            HistoryEntry::new("git commit -m x", "git"),
            HistoryEntry::new("ls", "ls"),
        ];
        let d2 = count_commands_at_depth(&entries, 2);
        assert_eq!(d2.get("git commit"), Some(&2));
        assert_eq!(d2.get("git push"), Some(&1));
        assert_eq!(d2.get("ls"), Some(&1));
    }

    #[test]
    fn depth_zero_returns_empty() {
        let entries = vec![HistoryEntry::new("ls", "ls")];
        assert!(count_commands_at_depth(&entries, 0).is_empty());
    }

    #[test]
    fn depth_exceeding_tokens_uses_whole_line() {
        let entries = vec![HistoryEntry::new("git push", "git")];
        let d5 = count_commands_at_depth(&entries, 5);
        assert_eq!(d5.get("git push"), Some(&1));
    }

    #[test]
    fn command_key_depth_uses_base_vs_raw() {
        let entry = HistoryEntry::new("Git Push Origin", "Git");
        assert_eq!(command_key(&entry, 1).as_deref(), Some("git"));
        assert_eq!(command_key(&entry, 2).as_deref(), Some("git push"));
        assert_eq!(command_key(&entry, 0), None);
    }

    #[test]
    fn command_key_skips_empty() {
        let entry = HistoryEntry::new("   ", "");
        assert_eq!(command_key(&entry, 1), None);
        assert_eq!(command_key(&entry, 2), None);
    }
}
