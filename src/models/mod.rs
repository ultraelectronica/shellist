//! Domain types for shell history entries.

pub mod history_entry;

pub use history_entry::{HistoryEntry, strip_command_prefixes};
