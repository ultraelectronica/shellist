/// Summary statistics for a ranked command list.
#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
    /// Total command occurrences (sum of counts).
    pub total_commands: usize,
    /// Number of distinct commands.
    pub unique_commands: usize,
    /// Most-used command and its count, if any.
    pub most_used: Option<(String, usize)>,
    /// Least-used command and its count, if any.
    pub least_used: Option<(String, usize)>,
    /// Average uses per unique command.
    pub average: f64,
}

/// Compute statistics from a ranked list. Order-independent.
pub fn compute_stats(ranked: &[(String, usize)]) -> Stats {
    let total: usize = ranked.iter().map(|(_, c)| *c).sum();
    let unique = ranked.len();
    let most_used = ranked.iter().max_by_key(|(_, c)| *c).cloned();
    let least_used = ranked.iter().min_by_key(|(_, c)| *c).cloned();
    let average = if unique == 0 {
        0.0
    } else {
        total as f64 / unique as f64
    };
    Stats {
        total_commands: total,
        unique_commands: unique,
        most_used,
        least_used,
        average,
    }
}

/// Format statistics as a human-readable block.
///
/// ```rust
/// let ranked = vec![("ls".to_string(), 10), ("git".to_string(), 5)];
/// let out = shellist::format_stats(&ranked);
/// assert!(out.contains("Total commands:   15"));
/// ```
pub fn format_stats(ranked: &[(String, usize)]) -> String {
    let s = compute_stats(ranked);
    let mut out = String::new();
    out.push_str(&format!("Total commands:   {}\n", s.total_commands));
    out.push_str(&format!("Unique commands:  {}\n", s.unique_commands));
    if let Some((m, c)) = &s.most_used {
        out.push_str(&format!("Most used:        {} ({})\n", m, c));
    }
    if let Some((l, c)) = &s.least_used {
        out.push_str(&format!("Least used:       {} ({})\n", l, c));
    }
    out.push_str(&format!("Average per cmd:  {:.1}\n", s.average));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stats_basic() {
        let ranked = vec![
            ("ls".to_string(), 10),
            ("git".to_string(), 5),
            ("cd".to_string(), 2),
        ];
        let s = compute_stats(&ranked);
        assert_eq!(s.total_commands, 17);
        assert_eq!(s.unique_commands, 3);
        assert_eq!(s.most_used, Some(("ls".to_string(), 10)));
        assert_eq!(s.least_used, Some(("cd".to_string(), 2)));
        assert!((s.average - 5.6666).abs() < 0.01);
    }

    #[test]
    fn stats_empty() {
        let s = compute_stats(&[]);
        assert_eq!(s.total_commands, 0);
        assert_eq!(s.unique_commands, 0);
        assert_eq!(s.most_used, None);
        assert_eq!(s.average, 0.0);
    }

    #[test]
    fn stats_order_independent() {
        let asc = vec![("a".to_string(), 1), ("b".to_string(), 9)];
        let desc = vec![("b".to_string(), 9), ("a".to_string(), 1)];
        assert_eq!(compute_stats(&asc), compute_stats(&desc));
    }

    #[test]
    fn format_stats_output() {
        let ranked = vec![("ls".to_string(), 10), ("git".to_string(), 5)];
        let out = format_stats(&ranked);
        assert!(out.contains("Total commands:   15"));
        assert!(out.contains("Most used:        ls (10)"));
    }
}
