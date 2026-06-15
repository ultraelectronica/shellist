use std::collections::HashMap;

/// Sort commands by frequency descending, with alphabetical tie-breaking.
///
/// ```rust
/// let mut map = std::collections::HashMap::new();
/// map.insert("git".into(), 3);
/// map.insert("ls".into(), 5);
/// map.insert("cd".into(), 1);
/// let ranked = shellist::rank_commands(map);
/// assert_eq!(ranked[0], ("ls".to_string(), 5));
/// assert_eq!(ranked[1], ("git".to_string(), 3));
/// assert_eq!(ranked[2], ("cd".to_string(), 1));
/// ```
pub fn rank_commands(map: HashMap<String, usize>) -> Vec<(String, usize)> {
    let mut ranked: Vec<(String, usize)> = map.into_iter().collect();

    ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    ranked
}

/// Like [`rank_commands`] but ascending: lowest count first, alphabetical tie-break.
///
/// ```rust
/// let mut map = std::collections::HashMap::new();
/// map.insert("git".into(), 3);
/// map.insert("ls".into(), 5);
/// let ranked = shellist::rank_commands_ascending(map);
/// assert_eq!(ranked[0], ("git".to_string(), 3));
/// ```
pub fn rank_commands_ascending(map: HashMap<String, usize>) -> Vec<(String, usize)> {
    let mut ranked: Vec<(String, usize)> = map.into_iter().collect();

    ranked.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));

    ranked
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranks_by_frequency_descending() {
        let mut map = HashMap::new();
        map.insert("git".to_string(), 3);
        map.insert("ls".to_string(), 5);
        map.insert("cd".to_string(), 1);

        let ranked = rank_commands(map);

        assert_eq!(ranked[0], ("ls".to_string(), 5));
        assert_eq!(ranked[1], ("git".to_string(), 3));
        assert_eq!(ranked[2], ("cd".to_string(), 1));
    }

    #[test]
    fn tie_breaks_alphabetically() {
        let mut map = HashMap::new();
        map.insert("zebra".to_string(), 2);
        map.insert("alpha".to_string(), 2);
        map.insert("middle".to_string(), 2);

        let ranked = rank_commands(map);

        assert_eq!(ranked[0], ("alpha".to_string(), 2));
        assert_eq!(ranked[1], ("middle".to_string(), 2));
        assert_eq!(ranked[2], ("zebra".to_string(), 2));
    }

    #[test]
    fn handles_single_command() {
        let mut map = HashMap::new();
        map.insert("ls".to_string(), 42);

        let ranked = rank_commands(map);

        assert_eq!(ranked.len(), 1);
        assert_eq!(ranked[0], ("ls".to_string(), 42));
    }

    #[test]
    fn handles_empty_map() {
        let map: HashMap<String, usize> = HashMap::new();
        let ranked = rank_commands(map);
        assert!(ranked.is_empty());
    }

    #[test]
    fn mixed_frequencies_and_ties() {
        let mut map = HashMap::new();
        map.insert("vim".to_string(), 10);
        map.insert("emacs".to_string(), 10);
        map.insert("nano".to_string(), 5);
        map.insert("cat".to_string(), 20);

        let ranked = rank_commands(map);

        assert_eq!(ranked[0], ("cat".to_string(), 20));
        assert_eq!(ranked[1], ("emacs".to_string(), 10));
        assert_eq!(ranked[2], ("vim".to_string(), 10));
        assert_eq!(ranked[3], ("nano".to_string(), 5));
    }

    #[test]
    fn ascending_orders_lowest_first() {
        let mut map = HashMap::new();
        map.insert("git".to_string(), 3);
        map.insert("ls".to_string(), 5);
        map.insert("cd".to_string(), 1);

        let ranked = rank_commands_ascending(map);

        assert_eq!(ranked[0], ("cd".to_string(), 1));
        assert_eq!(ranked[1], ("git".to_string(), 3));
        assert_eq!(ranked[2], ("ls".to_string(), 5));
    }

    #[test]
    fn ascending_tie_breaks_alphabetically() {
        let mut map = HashMap::new();
        map.insert("zebra".to_string(), 2);
        map.insert("alpha".to_string(), 2);

        let ranked = rank_commands_ascending(map);

        assert_eq!(ranked[0], ("alpha".to_string(), 2));
        assert_eq!(ranked[1], ("zebra".to_string(), 2));
    }
}
