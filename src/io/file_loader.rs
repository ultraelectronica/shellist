use std::fs;
use std::io;
use std::path::PathBuf;

// Read a history file and return its contents as a string.
pub fn load_history_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

// Resolve ~/.bash_history. Panics if HOME isn't set (it always is).
pub fn default_history_path() -> PathBuf {
    dirs_or_home().join(".bash_history")
}

// Try the dirs crate convention first, fall back to HOME env var.
fn dirs_or_home() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .expect("HOME environment variable not set")
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
