use crate::models::HistoryEntry;
use crate::parsers::HistoryParser;

/// Parser for zsh history files, including extended-history timestamps.
///
/// Extended format lines look like `: 1577836800:0;git push`, where the first
/// number is the start timestamp, the second is elapsed seconds, and the text
/// after `;` is the command. Lines without that prefix are treated as plain
/// commands.
///
/// ```rust
/// use shellist::{ZshHistoryParser, HistoryParser};
/// let entries = ZshHistoryParser::new().parse(": 1577836800:0;git push\nls\n");
/// assert_eq!(entries.len(), 2);
/// assert_eq!(entries[0].command, "git");
/// assert_eq!(entries[0].timestamp, Some(1577836800));
/// assert_eq!(entries[1].command, "ls");
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct ZshHistoryParser;

impl ZshHistoryParser {
    /// Create a new parser instance.
    pub const fn new() -> Self {
        Self
    }
}

impl HistoryParser for ZshHistoryParser {
    fn parse(&self, input: &str) -> Vec<HistoryEntry> {
        let mut entries = Vec::new();
        for line in input.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Some((command_line, ts)) = parse_zsh_extended(trimmed) {
                if command_line.is_empty() {
                    continue;
                }
                let command = command_line
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .to_string();
                entries.push(HistoryEntry::new(command_line, command).with_timestamp(ts));
            } else {
                let command = trimmed.split_whitespace().next().unwrap_or("").to_string();
                entries.push(HistoryEntry::new(trimmed, command));
            }
        }
        entries
    }
}

/// Parse a `: <epoch>:<elapsed>;<command>` line into (command_line, timestamp).
fn parse_zsh_extended(line: &str) -> Option<(String, u64)> {
    let rest = line.strip_prefix(':')?;
    let rest = rest.trim_start();
    let digit_end = rest
        .char_indices()
        .find(|(_, c)| !c.is_ascii_digit())
        .map(|(i, _)| i)
        .unwrap_or(rest.len());
    if digit_end == 0 {
        return None;
    }
    let ts: u64 = rest[..digit_end].parse().ok()?;
    let rest = rest[digit_end..].strip_prefix(':')?;
    // Skip elapsed seconds up to ';'.
    let command = match rest.find(';') {
        Some(idx) => rest[idx + 1..].trim_start().to_string(),
        None => return None,
    };
    Some((command, ts))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_extended_history() {
        let entries = ZshHistoryParser::new().parse(": 1577836800:0;git push\n");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].command, "git");
        assert_eq!(entries[0].raw, "git push");
        assert_eq!(entries[0].timestamp, Some(1577836800));
    }

    #[test]
    fn parses_plain_lines() {
        let entries = ZshHistoryParser::new().parse("ls\ncd ..\n");
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].command, "ls");
        assert_eq!(entries[0].timestamp, None);
    }

    #[test]
    fn handles_multiline_extended() {
        let input = ": 1:0;ls\n: 2:0;git status\n: 3:0;cargo build\n";
        let entries = ZshHistoryParser::new().parse(input);
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[2].command, "cargo");
        assert_eq!(entries[2].timestamp, Some(3));
    }

    #[test]
    fn skips_empty_lines() {
        let entries = ZshHistoryParser::new().parse(": 1:0;ls\n\n  \n: 2:0;git\n");
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn handles_empty_command_after_separator() {
        let entries = ZshHistoryParser::new().parse(": 1:0;\nls\n");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].command, "ls");
    }

    #[test]
    fn handles_empty_input() {
        assert!(ZshHistoryParser::new().parse("").is_empty());
    }

    #[test]
    fn extended_without_separator_is_plain() {
        // ": 123" has no ';' so it's not valid extended; treat as plain command.
        let entries = ZshHistoryParser::new().parse(": 123\n");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].command, ":");
    }

    #[test]
    fn parse_zsh_extended_helper() {
        assert_eq!(
            parse_zsh_extended(": 1577836800:0;git push"),
            Some(("git push".to_string(), 1577836800))
        );
        assert_eq!(parse_zsh_extended("git push"), None);
    }
}
