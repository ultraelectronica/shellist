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
}

/// Strip leading `sudo` and `VAR=value` tokens from a raw command line.
///
/// Returns a substring of the input with the prefixes removed. If the line
/// consists only of prefix tokens (e.g. bare `sudo`), the input is returned
/// unchanged so it still counts as `sudo`.
///
/// ```rust
/// use shellist::strip_command_prefixes;
/// assert_eq!(strip_command_prefixes("sudo apt install"), "apt install");
/// assert_eq!(strip_command_prefixes("FOO=bar ls"), "ls");
/// assert_eq!(strip_command_prefixes("sudo FOO=bar sudo ls"), "ls");
/// assert_eq!(strip_command_prefixes("sudo"), "sudo");
/// ```
pub fn strip_command_prefixes(raw: &str) -> &str {
    let mut rest = raw.trim_start();
    loop {
        let token = rest.split_whitespace().next().unwrap_or("");
        if token.is_empty() {
            return raw;
        }
        if !token.eq_ignore_ascii_case("sudo") && !is_env_assign(token) {
            return rest;
        }
        let remainder = rest[token.len()..].trim_start();
        if remainder.is_empty() {
            return raw;
        }
        rest = remainder;
    }
}

fn is_env_assign(token: &str) -> bool {
    let Some((name, _)) = token.split_once('=') else {
        return false;
    };
    !name.is_empty()
        && name
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
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
    fn with_timestamp_builds_full_entry() {
        let entry = HistoryEntry::new("ls", "ls").with_timestamp(7);
        assert_eq!(entry.command, "ls");
        assert_eq!(entry.timestamp, Some(7));
    }

    #[test]
    fn strip_leading_sudo() {
        assert_eq!(strip_command_prefixes("sudo apt install"), "apt install");
        assert_eq!(strip_command_prefixes("SUDO ls"), "ls");
    }

    #[test]
    fn strip_env_assignment() {
        assert_eq!(strip_command_prefixes("FOO=bar cmd"), "cmd");
        assert_eq!(strip_command_prefixes("_hidden=1 ls"), "ls");
        assert_eq!(strip_command_prefixes("A=1 B=2 cmd"), "cmd");
        assert_eq!(strip_command_prefixes("URL=http://x cmd"), "cmd");
    }

    #[test]
    fn strip_mixed_prefixes() {
        assert_eq!(strip_command_prefixes("sudo sudo FOO=bar ls -la"), "ls -la");
    }

    #[test]
    fn strip_keeps_bare_prefix_as_command() {
        assert_eq!(strip_command_prefixes("sudo"), "sudo");
        assert_eq!(strip_command_prefixes("FOO=bar"), "FOO=bar");
        assert_eq!(strip_command_prefixes("  sudo  "), "  sudo  ");
    }

    #[test]
    fn strip_plain_line_unchanged() {
        assert_eq!(strip_command_prefixes("git push"), "git push");
        assert_eq!(strip_command_prefixes("ls"), "ls");
    }

    #[test]
    fn strip_not_env_assign() {
        // '=' with empty or numeric-leading name is not an assignment.
        assert_eq!(strip_command_prefixes("=bar cmd"), "=bar cmd");
        assert_eq!(strip_command_prefixes("1=x cmd"), "1=x cmd");
    }
}
