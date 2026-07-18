//! Shell detection and per-shell history paths.

use std::path::PathBuf;

/// Supported shells whose history formats shellist understands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl Shell {
    /// Parse a shell name (`bash`/`zsh`/`fish`, case-insensitive).
    pub fn from_name(name: &str) -> Option<Self> {
        match name.trim().to_ascii_lowercase().as_str() {
            "bash" => Some(Shell::Bash),
            "zsh" => Some(Shell::Zsh),
            "fish" => Some(Shell::Fish),
            _ => None,
        }
    }

    /// Lowercase name usable in output.
    pub const fn as_str(self) -> &'static str {
        match self {
            Shell::Bash => "bash",
            Shell::Zsh => "zsh",
            Shell::Fish => "fish",
        }
    }

    /// Default history file location for this shell.
    ///
    /// - Bash: `~/.bash_history`
    /// - Zsh: `~/.zsh_history`
    /// - Fish: `$XDG_DATA_HOME/fish/fish_history` (default `~/.local/share/fish/fish_history`)
    pub fn default_history_path(self) -> Option<PathBuf> {
        let home = home_dir()?;
        Some(match self {
            Shell::Bash => home.join(".bash_history"),
            Shell::Zsh => home.join(".zsh_history"),
            Shell::Fish => {
                if let Some(xdg) = std::env::var_os("XDG_DATA_HOME") {
                    PathBuf::from(xdg).join("fish").join("fish_history")
                } else {
                    home.join(".local")
                        .join("share")
                        .join("fish")
                        .join("fish_history")
                }
            }
        })
    }
}

fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

/// Detect the shell format from history file contents.
///
/// - Lines like `: 1577836800:0;cmd` → zsh extended history.
/// - Lines like `- cmd: ...` → fish history.
/// - Anything else → bash.
pub fn detect_shell(input: &str) -> Shell {
    for line in input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .take(10)
    {
        if line.starts_with("- cmd:") {
            return Shell::Fish;
        }
        if looks_like_zsh_extended(line) {
            return Shell::Zsh;
        }
    }
    Shell::Bash
}

fn looks_like_zsh_extended(line: &str) -> bool {
    crate::parsers::zsh_history_parser::parse_zsh_extended(line).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_from_name() {
        assert_eq!(Shell::from_name("bash"), Some(Shell::Bash));
        assert_eq!(Shell::from_name("ZSH"), Some(Shell::Zsh));
        assert_eq!(Shell::from_name("Fish"), Some(Shell::Fish));
        assert_eq!(Shell::from_name("powershell"), None);
    }

    #[test]
    fn shell_as_str() {
        assert_eq!(Shell::Bash.as_str(), "bash");
        assert_eq!(Shell::Zsh.as_str(), "zsh");
        assert_eq!(Shell::Fish.as_str(), "fish");
    }

    #[test]
    fn bash_default_path() {
        let path = Shell::Bash.default_history_path().unwrap();
        assert_eq!(path.file_name().unwrap(), ".bash_history");
    }

    #[test]
    fn zsh_default_path() {
        let path = Shell::Zsh.default_history_path().unwrap();
        assert_eq!(path.file_name().unwrap(), ".zsh_history");
    }

    #[test]
    fn fish_default_path() {
        let path = Shell::Fish.default_history_path().unwrap();
        assert_eq!(path.file_name().unwrap(), "fish_history");
    }

    #[test]
    fn detect_bash() {
        assert_eq!(detect_shell("ls\ngit status\ncd .."), Shell::Bash);
    }

    #[test]
    fn detect_zsh_extended() {
        let input = ": 1577836800:0;ls\n: 1577836801:0;git status\n";
        assert_eq!(detect_shell(input), Shell::Zsh);
    }

    #[test]
    fn detect_fish() {
        let input = "- cmd: ls\n  when: 1577836800\n- cmd: git status\n  when: 1577836801\n";
        assert_eq!(detect_shell(input), Shell::Fish);
    }

    #[test]
    fn detect_empty_defaults_to_bash() {
        assert_eq!(detect_shell(""), Shell::Bash);
    }

    #[test]
    fn zsh_extended_detection_helper() {
        assert!(looks_like_zsh_extended(": 1577836800:0;git push"));
        assert!(looks_like_zsh_extended(":1234567890:0;ls"));
        assert!(!looks_like_zsh_extended("git push"));
        assert!(!looks_like_zsh_extended("- cmd: ls"));
        assert!(!looks_like_zsh_extended(":not zsh"));
    }

    #[test]
    fn detect_mixed_falls_to_first_signal() {
        // fish marker wins on its line
        let input = "ls\n- cmd: git status\n  when: 1\n";
        assert_eq!(detect_shell(input), Shell::Fish);
    }
}
