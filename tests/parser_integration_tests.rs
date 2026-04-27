use shellist::{parse_history, DefaultHistoryParser, HistoryEntry, HistoryParser};

#[test]
fn integration_parse_full_pipeline() {
    let input = "ls\nls\ngit\n";
    let entries = parse_history(input);

    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0], HistoryEntry::new("ls", "ls"));
    assert_eq!(entries[1], HistoryEntry::new("ls", "ls"));
    assert_eq!(entries[2], HistoryEntry::new("git", "git"));
}

#[test]
fn integration_trait_based_parsing() {
    let parser = DefaultHistoryParser::new();
    let entries = parser.parse("docker ps\n  \n  kubectl get pods  ");

    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].command, "docker");
    assert_eq!(entries[1].command, "kubectl");
}

#[test]
fn integration_ignores_empty_and_whitespace_lines() {
    let input = "\n  \nls\n\n  git   \n\n";
    let entries = parse_history(input);

    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].command, "ls");
    assert_eq!(entries[1].command, "git");
}

#[test]
fn integration_extracts_command_from_complex_line() {
    let input = "cargo build --release --target x86_64-unknown-linux-gnu";
    let entries = parse_history(input);

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].command, "cargo");
    assert_eq!(entries[0].raw, input);
}
