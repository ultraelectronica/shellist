use crate::models::HistoryEntry;
use crate::parsers::HistoryParser;

/// Default parser for bash history files.
///
/// Strips whitespace, skips empty lines, grabs the first token as the command.
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
        input
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| {
                let command = line.split_whitespace().next().unwrap_or("").to_string();
                HistoryEntry::new(line, command)
            })
            .collect()
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
}
