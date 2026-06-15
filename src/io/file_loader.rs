use std::io;
use std::path::PathBuf;

/// Read a history file and return its contents.
///
/// ```rust,no_run
/// let content = shellist::load_history_file("/home/user/.bash_history").unwrap();
/// ```
pub fn load_history_file(path: &str) -> io::Result<String> {
    std::fs::read_to_string(path)
}

/// Resolve the default history file path (`~/.bash_history`).
///
/// Uses the `HOME` environment variable.
///
/// ```rust
/// let path = shellist::default_history_path();
/// assert!(path.is_absolute());
/// assert_eq!(path.file_name().unwrap(), ".bash_history");
/// ```
pub fn default_history_path() -> PathBuf {
    crate::shell::Shell::Bash.default_history_path()
}

/// Resolve the default history path for a given shell.
pub fn default_history_path_for(shell: crate::shell::Shell) -> PathBuf {
    shell.default_history_path()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn loads_existing_file() {
        let dir = std::env::temp_dir().join("shellist_test_load");
        fs::write(&dir, "ls\ngit status\n").unwrap();
        let content = load_history_file(dir.to_str().unwrap()).unwrap();
        assert_eq!(content, "ls\ngit status\n");
        fs::remove_file(&dir).unwrap();
    }

    #[test]
    fn returns_error_for_missing_file() {
        let result = load_history_file("/tmp/nonexistent_shellist_file_12345");
        assert!(result.is_err());
    }

    #[test]
    fn default_path_ends_with_bash_history() {
        let path = default_history_path();
        assert_eq!(path.file_name().unwrap(), ".bash_history");
    }

    #[test]
    fn default_path_is_absolute() {
        let path = default_history_path();
        assert!(path.is_absolute());
    }
}
