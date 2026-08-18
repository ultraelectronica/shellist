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

/// Hour-of-day distribution (0–23, UTC) of timestamped entries,
/// sorted by hour. Only hours with data appear.
pub fn compute_hourly(entries: &[HistoryEntry]) -> Vec<(String, usize)> {
    let mut counts: BTreeMap<u32, usize> = BTreeMap::new();
    for entry in entries {
        if let Some(ts) = entry.timestamp {
            *counts.entry(((ts / 3600) % 24) as u32).or_insert(0) += 1;
        }
    }
    counts
        .into_iter()
        .map(|(h, c)| (format!("{h:02}"), c))
        .collect()
}

/// Format an hour-of-day distribution table (Hour | Count | Bars).
pub fn format_hourly(entries: &[HistoryEntry]) -> String {
    let hourly = compute_hourly(entries);
    if hourly.is_empty() {
        return String::new();
    }
    let max = hourly.iter().map(|(_, c)| *c).max().unwrap_or(1);
    let rows: Vec<Vec<String>> = hourly
        .iter()
        .map(|(h, c)| vec![h.clone(), c.to_string(), "#".repeat(bar_len(*c, max))])
        .collect();
    render_table(
        &["Hour".to_string(), "Count".to_string(), "Bars".to_string()],
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

    #[test]
    fn hourly_buckets_by_utc_hour() {
        // 2020-01-01T00:00:00Z and 2020-01-02T05:00:00Z.
        let e = vec![
            HistoryEntry::new("ls", "ls").with_timestamp(1_577_836_800),
            HistoryEntry::new("ls", "ls").with_timestamp(1_577_836_900),
            HistoryEntry::new("git", "git").with_timestamp(1_577_836_800 + 3600 * 5),
        ];
        let h = compute_hourly(&e);
        assert_eq!(h, vec![("00".to_string(), 2), ("05".to_string(), 1)]);
    }

    #[test]
    fn hourly_format_has_headers() {
        let e = vec![HistoryEntry::new("ls", "ls").with_timestamp(1_577_840_000)];
        let out = format_hourly(&e);
        assert!(out.contains("Hour"));
        assert!(out.contains("Count"));
        assert!(out.contains("Bars"));
    }

    #[test]
    fn hourly_empty_when_no_timestamps() {
        let e = vec![HistoryEntry::new("ls", "ls")];
        assert!(format_hourly(&e).is_empty());
        assert!(compute_hourly(&e).is_empty());
    }
}
