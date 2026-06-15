//! Search / regex filtering over ranked commands.

use regex::Regex;

/// Keep only ranked commands whose name matches the regex.
pub fn grep_filter(ranked: &[(String, usize)], pattern: &Regex) -> Vec<(String, usize)> {
    ranked
        .iter()
        .filter(|(cmd, _)| pattern.is_match(cmd))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ranked() -> Vec<(String, usize)> {
        vec![
            ("git".to_string(), 5),
            ("gita".to_string(), 2),
            ("ls".to_string(), 9),
            ("grep".to_string(), 1),
        ]
    }

    #[test]
    fn filters_by_pattern() {
        let re = Regex::new("^git").unwrap();
        let got = grep_filter(&ranked(), &re);
        let names: Vec<&str> = got.iter().map(|(c, _)| c.as_str()).collect();
        assert_eq!(names, vec!["git", "gita"]);
    }

    #[test]
    fn no_match_returns_empty() {
        let re = Regex::new("^zzz").unwrap();
        assert!(grep_filter(&ranked(), &re).is_empty());
    }

    #[test]
    fn anchored_vs_unanchored() {
        let re = Regex::new("git").unwrap();
        assert_eq!(grep_filter(&ranked(), &re).len(), 2);
    }
}
