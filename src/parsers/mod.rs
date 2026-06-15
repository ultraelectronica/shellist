//! Parsers turn raw history text into structured entries.
//!
//! Each shell stores history differently; implement [`HistoryParser`] to add
//! support for a new format.

pub mod default_history_parser;
pub mod fish_history_parser;
pub mod zsh_history_parser;

pub use default_history_parser::{DefaultHistoryParser, parse_history};
pub use fish_history_parser::FishHistoryParser;
pub use zsh_history_parser::ZshHistoryParser;

use crate::models::HistoryEntry;

/// Swappable parser trait. Implement this for zsh, fish, etc.
pub trait HistoryParser {
    /// Parse raw history text into a list of [`HistoryEntry`]s.
    fn parse(&self, input: &str) -> Vec<HistoryEntry>;
}
