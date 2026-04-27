// Shell history analysis lib. Parses, counts, ranks commands.

pub mod aggregators;
pub mod io;
pub mod models;
pub mod parsers;

pub use aggregators::{count_commands, filter_by_min_frequency, filter_commands, rank_commands, top_n};
pub use io::{default_history_path, load_history_file};
pub use models::HistoryEntry;
pub use parsers::{parse_history, DefaultHistoryParser, HistoryParser};

// Full pipeline: raw history text in, ranked commands out.
pub fn analyze(input: &str) -> Vec<(String, usize)> {
    let entries = parse_history(input);
    let counts = count_commands(&entries);
    rank_commands(counts)
}
