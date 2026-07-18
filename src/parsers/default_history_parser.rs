use crate::models::HistoryEntry;
use crate::parsers::HistoryParser;

/// Default parser for bash history files.
///
/// Strips whitespace, skips empty lines, grabs the first token as the command.
/// Lines starting with `#<digits>` are treated as timestamps
/// (bash `HISTTIMEFORMAT`) and attached to the following command line.
///
/// ```rust
/// use shellist::{DefaultHistoryParser, HistoryParser};
/// let entries = DefaultHistoryParser::new().parse("ls -la\n\n  git push  ");
/// assert_eq!(entries.len(), 2);
/// assert_eq!(entries[0].command, "ls");
/// assert_eq!(entries[1].command, "git");
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultHistoryParser;

impl DefaultHistoryParser {
    /// Create a new parser instance.
    pub const fn new() -> Self {
        Self
    }
}

impl HistoryParser for DefaultHistoryParser {
    fn parse(&self, input: &str) -> Vec<HistoryEntry> {
        let mut entries = Vec::new();
        let mut pending_ts: Option<u64> = None;
        for line in input.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix('#')
                && rest.chars().all(|c| c.is_ascii_digit())
                && !rest.is_empty()
            {
                pending_ts = rest.parse().ok();
                continue;
            }
            let command = trimmed.split_whitespace().next().unwrap_or("").to_string();
            let mut entry = HistoryEntry::new(trimmed, command);
            if let Some(ts) = pending_ts.take() {
                entry.timestamp = Some(ts);
            }
            entries.push(entry);
        }
        entries
    }
}

/// Convenience function: parse history using [`DefaultHistoryParser`].
///
/// ```rust
/// let entries = shellist::parse_history("echo hello\ncargo build");
/// assert_eq!(entries.len(), 2);
/// ```
pub fn parse_history(input: &str) -> Vec<HistoryEntry> {
    DefaultHistoryParser::new().parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_parses_basic_commands() {
        let parser = DefaultHistoryParser::new();
        let entries = parser.parse("ls\ngit commit\ncd /home");
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].command, "ls");
        assert_eq!(entries[1].command, "git");
        assert_eq!(entries[2].command, "cd");
    }

    #[test]
    fn parser_ignores_empty_lines() {
        let parser = DefaultHistoryParser::new();
        let entries = parser.parse("ls\n\n\ngit");
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn parser_trims_whitespace() {
        let parser = DefaultHistoryParser::new();
        let entries = parser.parse("  ls  \n  git status  ");
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].raw, "ls");
        assert_eq!(entries[0].command, "ls");
        assert_eq!(entries[1].raw, "git status");
        assert_eq!(entries[1].command, "git");
    }

    #[test]
    fn parser_handles_empty_input() {
        let parser = DefaultHistoryParser::new();
        let entries = parser.parse("");
        assert!(entries.is_empty());
    }

    #[test]
    fn parser_extracts_first_token() {
        let parser = DefaultHistoryParser::new();
        let entries = parser.parse("git commit -m \"hello world\"");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].command, "git");
    }

    #[test]
    fn parse_history_convenience_function() {
        let entries = parse_history("echo hello");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].command, "echo");
    }

    #[test]
    fn parses_bash_timestamp_prefix() {
        let input = "#1577836800\nls\n#1577836801\ngit status\n";
        let entries = DefaultHistoryParser::new().parse(input);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].timestamp, Some(1577836800));
        assert_eq!(entries[0].command, "ls");
        assert_eq!(entries[1].timestamp, Some(1577836801));
        assert_eq!(entries[1].command, "git");
    }

    #[test]
    fn bash_timestamp_mixed_with_plain() {
        let input = "ls\n#1577836800\ncd ..\n";
        let entries = DefaultHistoryParser::new().parse(input);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].timestamp, None);
        assert_eq!(entries[0].command, "ls");
        assert_eq!(entries[1].timestamp, Some(1577836800));
        assert_eq!(entries[1].command, "cd");
    }

    #[test]
    fn bash_consecutive_timestamps_use_last() {
        let input = "#1\n#2\nls\n";
        let entries = DefaultHistoryParser::new().parse(input);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].timestamp, Some(2));
    }

    #[test]
    fn bash_hash_not_digits_is_plain() {
        let input = "# comment\nls\n";
        let entries = DefaultHistoryParser::new().parse(input);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].command, "#");
        assert_eq!(entries[1].command, "ls");
    }
}
