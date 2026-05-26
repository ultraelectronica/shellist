//! Shell history analysis. Parses `.bash_history`, counts commands, and ranks by frequency.
//!
//! ```rust
//! let ranked = shellist::analyze("ls\ngit\nls\ncd\ngit\nls");
//! assert_eq!(ranked, vec![
//!     ("ls".to_string(), 3),
//!     ("git".to_string(), 2),
//!     ("cd".to_string(), 1),
//! ]);
//! ```

pub mod aggregators;
pub mod io;
pub mod models;
pub mod parsers;

pub use aggregators::{
    count_commands, filter_by_min_frequency, filter_commands, rank_commands, top_n,
};
pub use io::{default_history_path, load_history_file};
pub use models::HistoryEntry;
pub use parsers::{DefaultHistoryParser, HistoryParser, parse_history};

/// Full pipeline: raw history text in, ranked commands out.
///
/// Parses input into entries, counts each command (lowercased), then ranks by frequency
/// with alphabetical tie-breaking.
///
/// ```rust
/// let result = shellist::analyze("git push\ngit commit\ncargo test\ncd ..\ncd /\n");
/// assert_eq!(result.first(), Some(&("cd".to_string(), 2)));
/// ```
pub fn analyze(input: &str) -> Vec<(String, usize)> {
    let entries = parse_history(input);
    let counts = count_commands(&entries);
    rank_commands(counts)
}
