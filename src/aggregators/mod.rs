// Aggregation shit. Counts, groups, tallies commands.

pub mod command_counter;
pub mod command_ranker;

pub use command_counter::count_commands;
pub use command_ranker::rank_commands;
