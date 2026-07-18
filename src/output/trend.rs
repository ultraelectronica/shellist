use super::{Align, bar_len, render_table};
use crate::date::{Bucket, bucket_key};
use crate::models::HistoryEntry;
use std::collections::BTreeMap;

/// Bucket timestamped entries by date and return (bucket_key, count) sorted by time.
pub fn compute_trend(entries: &[HistoryEntry], bucket: Bucket) -> Vec<(String, usize)> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for entry in entries {
        if let Some(ts) = entry.timestamp {
            let key = bucket_key(ts, bucket);
            *counts.entry(key).or_insert(0) += 1;
        }
    }
    counts.into_iter().collect()
}

/// Format a trend table (Date | Count | Bars).
pub fn format_trend(entries: &[HistoryEntry], bucket: Bucket) -> String {
    let trend = compute_trend(entries, bucket);
    if trend.is_empty() {
        return String::new();
    }
    let max = trend.iter().map(|(_, c)| *c).max().unwrap_or(1);

    let mut rows = Vec::with_capacity(trend.len());
    for (key, count) in &trend {
        rows.push(vec![
            key.clone(),
            count.to_string(),
            "#".repeat(bar_len(*count, max)),
        ]);
    }

    render_table(
        &["Date".to_string(), "Count".to_string(), "Bars".to_string()],
        &rows,
        &[Align::Right, Align::Right, Align::Left],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entries() -> Vec<HistoryEntry> {
        vec![
            HistoryEntry::new("ls", "ls").with_timestamp(1_577_836_800), // 2020-01-01
            HistoryEntry::new("ls", "ls").with_timestamp(1_577_836_800),
            HistoryEntry::new("git", "git").with_timestamp(1_577_923_200), // 2020-01-02
            HistoryEntry::new("git", "git").with_timestamp(1_583_020_800), // 2020-03-01
        ]
    }

    #[test]
    fn trend_by_day() {
        let t = compute_trend(&entries(), Bucket::Day);
        assert_eq!(t.len(), 3);
        assert!(t.iter().any(|(k, c)| k == "2020-01-01" && *c == 2));
    }

    #[test]
    fn trend_by_month() {
        let t = compute_trend(&entries(), Bucket::Month);
        assert_eq!(t.len(), 2);
        assert!(t.iter().any(|(k, _)| k == "2020-01"));
        assert!(t.iter().any(|(k, _)| k == "2020-03"));
    }

    #[test]
    fn trend_ignores_entries_without_timestamp() {
        let mut e = entries();
        e.push(HistoryEntry::new("cd", "cd")); // no timestamp
        let t = compute_trend(&e, Bucket::Day);
        // cd entry ignored
        assert!(t.iter().all(|(_, c)| *c <= 2));
    }

    #[test]
    fn trend_format_has_headers() {
        let out = format_trend(&entries(), Bucket::Day);
        assert!(out.contains("Date"));
        assert!(out.contains("Count"));
        assert!(out.contains("Bars"));
    }

    #[test]
    fn trend_empty_when_no_timestamps() {
        let e = vec![HistoryEntry::new("ls", "ls")];
        assert!(format_trend(&e, Bucket::Day).is_empty());
        assert!(compute_trend(&e, Bucket::Day).is_empty());
    }
}
