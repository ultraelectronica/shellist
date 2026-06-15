/// Format ranked commands as CSV (RFC 4180 quoting).
///
/// ```rust
/// let ranked = vec![("ls".to_string(), 10)];
/// let csv = shellist::format_csv(&ranked);
/// assert!(csv.starts_with("command,count"));
/// ```
pub fn format_csv(ranked: &[(String, usize)]) -> String {
    let mut out = String::from("command,count\n");
    for (cmd, count) in ranked {
        out.push_str(&csv_field(cmd));
        out.push(',');
        out.push_str(&count.to_string());
        out.push('\n');
    }
    out
}

fn csv_field(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        let escaped = s.replace('"', "\"\"");
        format!("\"{escaped}\"")
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_header_and_rows() {
        let ranked = vec![("ls".to_string(), 10), ("git".to_string(), 5)];
        let out = format_csv(&ranked);
        assert_eq!(out, "command,count\nls,10\ngit,5\n");
    }

    #[test]
    fn csv_quotes_commas() {
        let ranked = vec![("a,b".to_string(), 1)];
        assert_eq!(format_csv(&ranked), "command,count\n\"a,b\",1\n");
    }

    #[test]
    fn csv_doubles_quotes() {
        let ranked = vec![("say \"hi\"".to_string(), 2)];
        assert_eq!(format_csv(&ranked), "command,count\n\"say \"\"hi\"\"\",2\n");
    }

    #[test]
    fn csv_empty_input_has_header_only() {
        assert_eq!(format_csv(&[]), "command,count\n");
    }
}
