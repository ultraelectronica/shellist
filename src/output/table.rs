use std::collections::HashMap;

use super::{Align, bar_len, render_table};
use crate::date::{Bucket, bucket_key};

/// Toggles for optional table columns.
#[derive(Default, Clone, Copy)]
pub struct TableOptions<'a> {
    /// Show a percentage column (share of total count).
    pub percent: bool,
    /// Show an ASCII bar chart column scaled to the top entry.
    pub bars: bool,
    /// Show a "Last Used" date column from max-timestamp-per-command
    /// (see [`crate::last_used_at_depth`]). Un timestamped commands show `-`.
    pub last_used: Option<&'a HashMap<String, u64>>,
}

/// Format ranked commands as an aligned text table.
///
/// ```rust
/// use shellist::{format_table, TableOptions};
/// let ranked = vec![("ls".to_string(), 10), ("git".to_string(), 5)];
/// let out = format_table(&ranked, &TableOptions::default());
/// assert!(out.contains("Rank"));
/// assert!(out.contains("Command"));
/// ```
pub fn format_table(ranked: &[(String, usize)], opts: &TableOptions) -> String {
    if ranked.is_empty() {
        return String::new();
    }
    let total: usize = ranked.iter().map(|(_, c)| *c).sum();
    let max_count = ranked.iter().map(|(_, c)| *c).max().unwrap_or(1);

    let mut headers = vec![
        "Rank".to_string(),
        "Command".to_string(),
        "Count".to_string(),
    ];
    let mut aligns = vec![Align::Right, Align::Left, Align::Right];
    if opts.last_used.is_some() {
        headers.push("Last Used".to_string());
        aligns.push(Align::Right);
    }
    if opts.percent {
        headers.push("Pct".to_string());
        aligns.push(Align::Right);
    }
    if opts.bars {
        headers.push("Bars".to_string());
        aligns.push(Align::Left);
    }

    let mut rows = Vec::with_capacity(ranked.len());
    for (i, (cmd, count)) in ranked.iter().enumerate() {
        let mut row = vec![(i + 1).to_string(), cmd.clone(), count.to_string()];
        if let Some(last) = opts.last_used {
            let cell = last
                .get(cmd)
                .map_or_else(|| "-".to_string(), |ts| bucket_key(*ts, Bucket::Day));
            row.push(cell);
        }
        if opts.percent {
            let pct = if total == 0 {
                0.0
            } else {
                *count as f64 * 100.0 / total as f64
            };
            row.push(format!("{pct:.1}%"));
        }
        if opts.bars {
            row.push("#".repeat(bar_len(*count, max_count)));
        }
        rows.push(row);
    }

    render_table(&headers, &rows, &aligns)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ranked() -> Vec<(String, usize)> {
        vec![
            ("ls".to_string(), 10),
            ("git".to_string(), 5),
            ("cd".to_string(), 2),
        ]
    }

    #[test]
    fn table_has_headers_and_separator() {
        let out = format_table(&ranked(), &TableOptions::default());
        assert!(out.contains("Rank"));
        assert!(out.contains("Command"));
        assert!(out.contains("Count"));
        assert!(out.contains("----"));
    }

    #[test]
    fn table_empty_returns_empty_string() {
        assert!(format_table(&[], &TableOptions::default()).is_empty());
    }

    #[test]
    fn percent_column_present() {
        let out = format_table(
            &ranked(),
            &TableOptions {
                percent: true,
                bars: false,
                last_used: None,
            },
        );
        assert!(out.contains("Pct"));
        assert!(out.contains("58.8%")); // 10/17
    }

    #[test]
    fn bars_column_present_and_top_is_full() {
        let out = format_table(
            &ranked(),
            &TableOptions {
                percent: false,
                bars: true,
                last_used: None,
            },
        );
        assert!(out.contains("Bars"));
        // Top entry (10) is the max → 30 '#'s.
        assert!(out.contains(&"#".repeat(30)));
    }

    #[test]
    fn percent_and_bars_together() {
        let out = format_table(
            &ranked(),
            &TableOptions {
                percent: true,
                bars: true,
                last_used: None,
            },
        );
        assert!(out.contains("Pct"));
        assert!(out.contains("Bars"));
    }

    #[test]
    fn last_used_column_shows_date_and_dash() {
        let mut last = HashMap::new();
        last.insert("ls".to_string(), 1_577_836_800); // 2020-01-01
        let out = format_table(
            &ranked(),
            &TableOptions {
                percent: false,
                bars: false,
                last_used: Some(&last),
            },
        );
        assert!(out.contains("Last Used"));
        assert!(out.contains("2020-01-01"));
        // git/cd have no timestamp → dash.
        assert!(out.lines().any(|l| l.contains("git") && l.contains('-')));
    }
}
