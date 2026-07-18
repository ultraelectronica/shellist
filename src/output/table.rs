use super::{Align, bar_len, render_table};

/// Toggles for optional table columns.
#[derive(Default, Clone, Copy)]
pub struct TableOptions {
    /// Show a percentage column (share of total count).
    pub percent: bool,
    /// Show an ASCII bar chart column scaled to the top entry.
    pub bars: bool,
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
            },
        );
        assert!(out.contains("Pct"));
        assert!(out.contains("Bars"));
    }
}
