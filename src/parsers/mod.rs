//! Parsers turn raw history text into structured entries.

pub mod default_history_parser;

pub use default_history_parser::{DefaultHistoryParser, parse_history};

use crate::models::HistoryEntry;

/// Swappable parser trait. Implement this for zsh, fish, etc.
pub trait HistoryParser {
    /// Parse raw history text into a list of [`HistoryEntry`]s.
    fn parse(&self, input: &str) -> Vec<HistoryEntry>;
}
