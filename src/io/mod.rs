//! File I/O for loading history files from disk.

pub mod file_loader;

pub use file_loader::{default_history_path, default_history_path_for, load_history_file};
