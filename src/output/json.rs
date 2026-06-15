/// Format ranked commands as a JSON array.
///
/// ```rust
/// let ranked = vec![("ls".to_string(), 10)];
/// let json = shellist::format_json(&ranked);
/// assert!(json.starts_with("["));
/// assert!(json.contains("\"command\": \"ls\""));
/// assert!(json.contains("\"count\": 10"));
/// ```
pub fn format_json(ranked: &[(String, usize)]) -> String {
    let mut out = String::from("[");
    for (i, (cmd, count)) in ranked.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str("\n  {\"command\": \"");
        json_escape(cmd, &mut out);
        out.push_str("\", \"count\": ");
        out.push_str(&count.to_string());
        out.push('}');
    }
    if !ranked.is_empty() {
        out.push('\n');
    }
    out.push(']');
    out.push('\n');
    out
}

fn json_escape(s: &str, out: &mut String) {
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_basic() {
        let ranked = vec![("ls".to_string(), 10), ("git".to_string(), 5)];
        let out = format_json(&ranked);
        assert!(out.contains("\"command\": \"ls\""));
        assert!(out.contains("\"count\": 10"));
    }

    #[test]
    fn json_empty_array() {
        assert_eq!(format_json(&[]), "[]\n");
    }

    #[test]
    fn json_escapes_special_chars() {
        let ranked = vec![("a\"b\\c\nd".to_string(), 1)];
        let out = format_json(&ranked);
        assert!(out.contains("a\\\"b\\\\c\\nd"));
    }
}
