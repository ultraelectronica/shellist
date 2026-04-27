use std::collections::HashSet;

// Keep only the top N entries from a ranked list.
pub fn top_n(commands: Vec<(String, usize)>, n: usize) -> Vec<(String, usize)> {
    commands.into_iter().take(n).collect()
}

// Drop commands whose name matches anything in the ignore list.
pub fn filter_commands<S: AsRef<str>>(
    commands: Vec<(String, usize)>,
    ignore: &[S],
) -> Vec<(String, usize)> {
    let ignore_set: HashSet<&str> = ignore.iter().map(|s| s.as_ref()).collect();
    commands
        .into_iter()
        .filter(|(cmd, _)| !ignore_set.contains(cmd.as_str()))
        .collect()
}

// Drop commands below a minimum frequency threshold.
pub fn filter_by_min_frequency(
    commands: Vec<(String, usize)>,
    min: usize,
) -> Vec<(String, usize)> {
    commands
        .into_iter()
        .filter(|(_, count)| *count >= min)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_n_returns_first_n() {
        let commands = vec![
            ("ls".into(), 10),
            ("git".into(), 5),
            ("cd".into(), 3),
        ];
        let result = top_n(commands, 2);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], ("ls".into(), 10));
        assert_eq!(result[1], ("git".into(), 5));
    }

    #[test]
    fn top_n_with_n_larger_than_list() {
        let commands = vec![("ls".into(), 10)];
        let result = top_n(commands, 100);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn top_n_zero_returns_empty() {
        let commands = vec![("ls".into(), 10)];
        let result = top_n(commands, 0);
        assert!(result.is_empty());
    }

    #[test]
    fn top_n_empty_input() {
        let commands: Vec<(String, usize)> = vec![];
        let result = top_n(commands, 5);
        assert!(result.is_empty());
    }

    #[test]
    fn filter_commands_removes_matching() {
        let commands = vec![
            ("ls".into(), 10),
            ("git".into(), 5),
            ("cd".into(), 3),
        ];
        let result = filter_commands(commands, &["git".to_string()]);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, "ls");
        assert_eq!(result[1].0, "cd");
    }

    #[test]
    fn filter_commands_with_str_slices() {
        let commands = vec![
            ("ls".into(), 10),
            ("git".into(), 5),
            ("cd".into(), 3),
        ];
        let result = filter_commands(commands, &["git", "cd"]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "ls");
    }

    #[test]
    fn filter_commands_nothing_ignored() {
        let commands = vec![
            ("ls".into(), 10),
            ("git".into(), 5),
        ];
        let result: Vec<(String, usize)> = filter_commands(commands, &[] as &[&str]);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn filter_commands_all_ignored() {
        let commands = vec![("ls".into(), 10)];
        let result = filter_commands(commands, &["ls"]);
        assert!(result.is_empty());
    }

    #[test]
    fn filter_commands_empty_input() {
        let commands: Vec<(String, usize)> = vec![];
        let result = filter_commands(commands, &["git"]);
        assert!(result.is_empty());
    }

    #[test]
    fn min_frequency_filters_below_threshold() {
        let commands = vec![
            ("ls".into(), 10),
            ("git".into(), 5),
            ("cd".into(), 1),
        ];
        let result = filter_by_min_frequency(commands, 2);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, "ls");
        assert_eq!(result[1].0, "git");
    }

    #[test]
    fn min_frequency_exact_threshold() {
        let commands = vec![
            ("ls".into(), 3),
            ("git".into(), 2),
        ];
        let result = filter_by_min_frequency(commands, 3);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "ls");
    }

    #[test]
    fn min_frequency_zero_passes_all() {
        let commands = vec![("ls".into(), 1)];
        let result = filter_by_min_frequency(commands, 0);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn min_frequency_higher_than_any_count() {
        let commands = vec![("ls".into(), 5)];
        let result = filter_by_min_frequency(commands, 100);
        assert!(result.is_empty());
    }

    #[test]
    fn min_frequency_empty_input() {
        let commands: Vec<(String, usize)> = vec![];
        let result = filter_by_min_frequency(commands, 1);
        assert!(result.is_empty());
    }
}