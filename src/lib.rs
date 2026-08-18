//! Shell history analysis. Parses shell history files (bash, zsh, fish),
//! counts commands, and ranks them by frequency.
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
pub mod completions;
pub mod date;
pub mod io;
pub mod man;
pub mod models;
pub mod output;
pub mod parsers;
pub mod search;
pub mod shell;

pub use aggregators::{
    command_key, count_commands, count_commands_at_depth, filter_by_min_frequency, filter_commands,
    last_used_at_depth, rank_commands, rank_commands_ascending, top_n,
};
pub use completions::completions;
pub use date::{
    Bucket, bucket_key, civil_from_days, days_from_civil, parse_date_to_unix, resolve_date_spec,
};
pub use io::{default_history_path, load_history_file};
pub use man::man_page;
pub use models::{HistoryEntry, strip_command_prefixes};
pub use output::{
    Stats, TableOptions, compute_hourly, compute_stats, compute_trend, format_csv, format_hourly,
    format_json, format_stats, format_table, format_trend,
};
pub use parsers::{
    DefaultHistoryParser, FishHistoryParser, HistoryParser, ZshHistoryParser, parse_history,
};

/// Alias for `DefaultHistoryParser` for naming consistency with `ZshHistoryParser` / `FishHistoryParser`.
pub type BashHistoryParser = DefaultHistoryParser;
pub use search::grep_filter;
pub use shell::{Shell, detect_shell};

/// Full pipeline: raw history text in, ranked commands out.
///
/// Parses input into entries, counts each command (lowercased), then ranks by
/// frequency with alphabetical tie-breaking.
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

/// Detect shell format and parse with the appropriate parser.
///
/// Dispatches to `DefaultHistoryParser`, `ZshHistoryParser`, or
/// `FishHistoryParser` based on the content, then returns parsed entries
/// including any timestamps the format provides.
///
/// ```rust
/// let entries = shellist::parse_with_detected(": 1577836800:0;git push\nls\n");
/// assert_eq!(entries.len(), 2);
/// assert_eq!(entries[0].timestamp, Some(1577836800));
/// assert_eq!(entries[1].command, "ls");
/// ```
pub fn parse_with_detected(input: &str) -> Vec<HistoryEntry> {
    match detect_shell(input) {
        Shell::Bash => DefaultHistoryParser::new().parse(input),
        Shell::Zsh => ZshHistoryParser::new().parse(input),
        Shell::Fish => FishHistoryParser::new().parse(input),
    }
}
