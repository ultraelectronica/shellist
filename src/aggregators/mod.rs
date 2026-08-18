//! Command aggregation: counting, ranking, and filtering.

pub mod command_counter;
pub mod command_filter;
pub mod command_ranker;

pub use command_counter::{
    command_key, count_commands, count_commands_at_depth, last_used_at_depth,
};
pub use command_filter::{filter_by_min_frequency, filter_commands, top_n};
pub use command_ranker::{rank_commands, rank_commands_ascending};
