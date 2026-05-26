# Shellist

Parse `.bash_history`, count commands, rank by frequency. Library and CLI.

## Install

```sh
cargo install shellist
```

## CLI

```sh
shellist                   # reads ~/.bash_history, prints ranked table
shellist --top 5           # top 5 only
shellist --ignore ls,cd    # exclude ls and cd
shellist --min 3           # only commands used ≥ 3 times
shellist --path ./my_history.txt
```

Options:

| Flag            | Description                          |
|-----------------|--------------------------------------|
| `--top N`       | Show top N commands                  |
| `--ignore X,Y`  | Exclude commands (comma-separated)   |
| `--min N`       | Only commands with count ≥ N         |
| `--path PATH`   | Read from PATH (default: ~/.bash_history) |
| `--help`        | Print help                          |

Example output:

```
Rank  Command  Count
----  -------  -----
   1  ls          120
   2  git          95
   3  cd           80
```

## Library

```rust
use shellist::{analyze, parse_history, count_commands, rank_commands,
                top_n, filter_commands, filter_by_min_frequency,
                load_history_file, default_history_path};

// Full pipeline
let ranked = analyze("ls\ngit\nls\ncd\ngit\nls");
assert_eq!(ranked, vec![
    ("ls".to_string(), 3),
    ("git".to_string(), 2),
    ("cd".to_string(), 1),
]);

// Step by step
let entries = parse_history("git push\ncd /\ngit commit");
let counts = count_commands(&entries);
let ranked = rank_commands(counts);
let top = top_n(ranked, 2);
let filtered = filter_commands(top, &["cd"]);

// From file
let content = load_history_file("/home/user/.bash_history")?;
```

## Benchmarks

```sh
cargo bench
```

Runs criterion benchmarks on `analyze()` with 100, 1k, 10k, and 100k history entries. Results
are written to `target/criterion/` with HTML reports.

To compare against a baseline:

```sh
cargo bench -- --save-baseline before
# make changes...
cargo bench -- --baseline before
```

## License

MIT
