// Parsers turn raw history text into actual structs.

pub mod default_history_parser;

pub use default_history_parser::{parse_history, DefaultHistoryParser};

use crate::models::HistoryEntry;

// Trait so you can swap parsers without touching caller code.
// Want zsh format? Implement this. Done.
pub trait HistoryParser {
    fn parse(&self, input: &str) -> Vec<HistoryEntry>;
}
