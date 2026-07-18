//! Date math and parsing with zero dependencies.
//!
//! Uses Howard Hinnant's `days_from_civil` / `civil_from_days` algorithms,
//! valid proleptic Gregorian for any year.

/// Time bucket granularity for trend reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bucket {
    Day,
    Week,
    Month,
}

impl Bucket {
    /// Parse a bucket name (`day`/`week`/`month`, case-insensitive).
    pub fn from_name(name: &str) -> Option<Self> {
        match name.trim().to_ascii_lowercase().as_str() {
            "day" | "daily" => Some(Bucket::Day),
            "week" | "weekly" => Some(Bucket::Week),
            "month" | "monthly" => Some(Bucket::Month),
            _ => None,
        }
    }
}

/// Days since the Unix epoch (1970-01-01) for a (year, month, day).
#[allow(clippy::cast_possible_truncation)]
pub fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = (if y >= 0 { y } else { y - 399 }) / 400;
    let yoe = (y - era * 400) as u64;
    let mp = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * mp as u64 + 2) / 5 + d as u64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe as i64 - 719_468
}

/// Inverse of [`days_from_civil`]: days since epoch → (year, month, day).
pub fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = (if z >= 0 { z } else { z - 146_096 }) / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

/// Parse a `YYYY-MM-DD` date into a Unix timestamp (start of day, UTC).
pub fn parse_date_to_unix(s: &str) -> Option<i64> {
    let s = s.trim();
    let mut parts = s.split('-');
    let y: i64 = parts.next()?.trim().parse().ok()?;
    let m: u32 = parts.next()?.trim().parse().ok()?;
    let d: u32 = parts.next()?.trim().parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    if !(1..=12).contains(&m) {
        return None;
    }
    let days = days_from_civil(y, m, d);
    let (yr, mo, dy) = civil_from_days(days);
    if (yr, mo, dy) != (y, m, d) {
        return None;
    }
    Some(days * 86_400)
}

/// A sortable, human-readable bucket key for a Unix timestamp.
pub fn bucket_key(ts: u64, bucket: Bucket) -> String {
    let days = (ts / 86_400) as i64;
    let (y, m, d) = civil_from_days(days);
    match bucket {
        Bucket::Day | Bucket::Week => {
            if matches!(bucket, Bucket::Week) {
                // 1970-01-01 was Thursday; Monday-based week: (days+3) % 7 == 0 on Monday.
                let monday = days - (days + 3).rem_euclid(7);
                let (wy, wm, wd) = civil_from_days(monday);
                format!("{wy:04}-{wm:02}-{wd:02}")
            } else {
                format!("{y:04}-{m:02}-{d:02}")
            }
        }
        Bucket::Month => format!("{y:04}-{m:02}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_is_zero_days() {
        assert_eq!(days_from_civil(1970, 1, 1), 0);
    }

    #[test]
    fn y2k_known_offset() {
        // 2000-01-01 is 10957 days after 1970-01-01.
        assert_eq!(days_from_civil(2000, 1, 1), 10_957);
    }

    #[test]
    fn civil_round_trip() {
        for (y, m, d) in [
            (1970, 1, 1),
            (2000, 1, 1),
            (1999, 12, 31),
            (2024, 2, 29),
            (2099, 6, 15),
            (1601, 1, 1),
        ] {
            let days = days_from_civil(y, m, d);
            assert_eq!(civil_from_days(days), (y, m, d), "{y}-{m}-{d}");
        }
    }

    #[test]
    fn parse_date_valid() {
        assert_eq!(parse_date_to_unix("2020-01-01"), Some(1_577_836_800));
        assert_eq!(parse_date_to_unix("1970-01-01"), Some(0));
    }

    #[test]
    fn parse_date_invalid() {
        assert_eq!(parse_date_to_unix("not-a-date"), None);
        assert_eq!(parse_date_to_unix("2020-13-01"), None);
        assert_eq!(parse_date_to_unix("2020-00-40"), None);
        assert_eq!(parse_date_to_unix("2020-02-30"), None);
        assert_eq!(parse_date_to_unix("2023-04-31"), None);
        assert_eq!(parse_date_to_unix("2023-02-29"), None);
        assert_eq!(
            parse_date_to_unix("2020-1-1"),
            Some(days_from_civil(2020, 1, 1) * 86400)
        );
    }

    #[test]
    fn bucket_day_key() {
        assert_eq!(bucket_key(1_577_836_800, Bucket::Day), "2020-01-01");
    }

    #[test]
    fn bucket_month_key() {
        assert_eq!(bucket_key(1_577_836_800, Bucket::Month), "2020-01");
    }

    #[test]
    fn bucket_week_aligns_to_monday() {
        let key = bucket_key(1_577_923_200, Bucket::Week); // 2020-01-02 (Thursday)
        // ISO week 1 of 2020 starts Monday 2019-12-30.
        assert_eq!(key, "2019-12-30");
    }

    #[test]
    fn bucket_from_name() {
        assert_eq!(Bucket::from_name("day"), Some(Bucket::Day));
        assert_eq!(Bucket::from_name("WEEKLY"), Some(Bucket::Week));
        assert_eq!(Bucket::from_name("month"), Some(Bucket::Month));
        assert_eq!(Bucket::from_name("hour"), None);
    }
}
