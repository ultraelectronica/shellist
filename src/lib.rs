// Shell history analysis lib. Parses, counts, ranks commands.

pub mod aggregators;
pub mod models;
pub mod parsers;

pub use aggregators::{count_commands, filter_by_min_frequency, filter_commands, rank_commands, top_n};
pub use models::HistoryEntry;
pub use parsers::{parse_history, DefaultHistoryParser, HistoryParser};
