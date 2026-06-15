use crate::models::HistoryEntry;
use crate::parsers::HistoryParser;

/// Parser for fish history files (`fish_history`).
///
/// Fish stores history as YAML-like pairs:
/// ```text
/// - cmd: git push
///   when: 1577836800
/// ```
/// This parser reads `cmd:` lines for the command and `when:` lines for the
/// Unix timestamp, ignoring everything else (e.g. `paths:` blocks).
///
/// ```rust
/// use shellist::{FishHistoryParser, HistoryParser};
/// let input = "- cmd: git push\n  when: 1577836800\n- cmd: ls\n  when: 1577836801\n";
/// let entries = FishHistoryParser::new().parse(input);
/// assert_eq!(entries.len(), 2);
/// assert_eq!(entries[0].command, "git");
/// assert_eq!(entries[0].timestamp, Some(1577836800));
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct FishHistoryParser;

impl FishHistoryParser {
    /// Create a new parser instance.
    pub const fn new() -> Self {
        Self
    }
}

impl HistoryParser for FishHistoryParser {
    fn parse(&self, input: &str) -> Vec<HistoryEntry> {
        let mut entries = Vec::new();
        let mut current_cmd: Option<String> = None;
        let mut current_ts: Option<u64> = None;

        for line in input.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("- cmd:") {
                flush(&mut entries, current_cmd.take(), current_ts.take());
                let cmd = rest.trim().to_string();
                if !cmd.is_empty() {
                    current_cmd = Some(cmd);
                }
            } else if let Some(rest) = trimmed.strip_prefix("cmd:") {
                // Bare "cmd:" without leading dash — also accept.
                flush(&mut entries, current_cmd.take(), current_ts.take());
                let cmd = rest.trim().to_string();
                if !cmd.is_empty() {
                    current_cmd = Some(cmd);
                }
            } else if let Some(rest) = trimmed.strip_prefix("when:")
                && let Ok(t) = rest.trim().parse::<u64>()
            {
                current_ts = Some(t);
            }
        }
        flush(&mut entries, current_cmd.take(), current_ts.take());
        entries
    }
}

fn flush(entries: &mut Vec<HistoryEntry>, cmd: Option<String>, ts: Option<u64>) {
    if let Some(cmd) = cmd {
        let command = cmd.split_whitespace().next().unwrap_or("").to_string();
        let mut entry = HistoryEntry::new(cmd, command);
        entry.timestamp = ts;
        entries.push(entry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_fish_history() {
        let input = "- cmd: git push\n  when: 1577836800\n- cmd: ls\n  when: 1577836801\n";
        let entries = FishHistoryParser::new().parse(input);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].command, "git");
        assert_eq!(entries[0].raw, "git push");
        assert_eq!(entries[0].timestamp, Some(1577836800));
        assert_eq!(entries[1].command, "ls");
        assert_eq!(entries[1].timestamp, Some(1577836801));
    }

    #[test]
    fn handles_missing_when() {
        let input = "- cmd: ls\n- cmd: git status\n";
        let entries = FishHistoryParser::new().parse(input);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].timestamp, None);
    }

    #[test]
    fn ignores_paths_block() {
        let input = "- cmd: cd\n  when: 1\n  paths:\n    - /home\n- cmd: ls\n  when: 2\n";
        let entries = FishHistoryParser::new().parse(input);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].command, "cd");
        assert_eq!(entries[1].command, "ls");
    }

    #[test]
    fn handles_empty_cmd_value() {
        let input = "- cmd:\n- cmd: ls\n  when: 1\n";
        let entries = FishHistoryParser::new().parse(input);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].command, "ls");
    }

    #[test]
    fn handles_empty_input() {
        assert!(FishHistoryParser::new().parse("").is_empty());
    }

    #[test]
    fn handles_trailing_cmd_without_when() {
        let input = "- cmd: ls\n  when: 1\n- cmd: git status\n";
        let entries = FishHistoryParser::new().parse(input);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[1].command, "git");
        assert_eq!(entries[1].timestamp, None);
    }

    #[test]
    fn ignores_garbage_lines() {
        let input = "garbage line\n- cmd: ls\n  when: 5\nrandom: value\n";
        let entries = FishHistoryParser::new().parse(input);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].timestamp, Some(5));
    }
}
