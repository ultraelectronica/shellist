# Shellist

Parse shell history (bash, zsh, fish), count commands, and rank them by frequency. Library and CLI.

## Install

```sh
cargo install shellist
```

## CLI

```sh
shellist                      # reads ~/.bash_history, prints ranked table
shellist --top 5              # top 5 only
shellist --shell zsh          # force the zsh parser
shellist --depth 2            # rank multi-word commands (git commit, git push, ...)
shellist --bars --percent     # add a bar chart and percentage column
shellist --json               # machine-readable output
shellist --grep '^git'        # regex filter on command names
shellist --since 2024-01-01   # date range (needs timestamps)
shellist --trend --trend-bucket month  # usage over time
cat ~/.zsh_history | shellist # read from stdin
```

### Input

| Flag                          | Description                                              |
|-------------------------------|----------------------------------------------------------|
| `--path PATH`                 | Read history from PATH instead of the default file      |
| `--shell bash\|zsh\|fish`     | Force a parser (default: auto-detect from contents)     |
| `-`                           | Read from stdin (or just pipe history in)               |

By default `shellist` reads `~/.bash_history`. With `--shell zsh`/`fish` it uses
`~/.zsh_history` / `$XDG_DATA_HOME/fish/fish_history`. Otherwise the format is
auto-detected from the file contents.

### Filtering

| Flag                   | Description                                              |
|------------------------|----------------------------------------------------------|
| `--top N`              | Show only the top N commands                             |
| `--ignore X,Y`         | Exclude commands (comma-separated)                       |
| `--no-default-ignore`  | Don't filter bash internals (set, shopt)                 |
| `--min N`              | Only commands used at least N times                      |
| `--grep PATTERN`       | Keep commands matching a regex (case-insensitive)       |
| `--depth N`            | Treat the first N tokens as the command key (default 1)  |
| `--since DATE`         | Only on/after DATE (`YYYY-MM-DD`, needs timestamps)      |
| `--until DATE`         | Only on/before DATE (`YYYY-MM-DD`, needs timestamps)     |
| `--asc`                | Sort ascending                                           |

By default, `set` and `shopt` are ignored — these are bash internals that often leak into `.bash_history` from shell init scripts, not commands you actually typed. Use `--no-default-ignore` to see them.

### Output

| Flag                            | Description                                     |
|---------------------------------|-------------------------------------------------|
| `--bars`                        | Add an ASCII bar chart column                   |
| `--percent`                     | Add a percentage column                         |
| `--json`                        | Output as a JSON array                          |
| `--csv`                         | Output as CSV                                   |
| `--stats`                       | Print summary statistics                        |
| `--trend`                       | Show usage bucketed over time (UTC, needs timestamps)|
| `--trend-bucket day\|week\|month` | Bucket granularity for `--trend` (default day). Also `daily`,`weekly`,`monthly`|
| `--output FILE`                 | Write output to FILE instead of stdout          |

### Integration

| Flag                            | Description                                     |
|---------------------------------|-------------------------------------------------|
| `--completions bash\|zsh\|fish` | Print a shell completion script                 |
| `--man`                         | Print the man page                              |
| `--help`                        | Print help                                      |

Install completions, for example for bash:

```sh
shellist --completions bash > /etc/bash_completion.d/shellist
```

Example output:

```
Rank  Command  Count    Pct  Bars
----  -------  -----  -----  ------------------------------
   1  ls          120  40.0%  ##############################
   2  git          95  31.7%  #######################
   3  cd           80  26.7%  ####################
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

### Multi-shell parsing

```rust
use shellist::{ZshHistoryParser, FishHistoryParser, HistoryParser, detect_shell};

let zsh_entries = ZshHistoryParser::new().parse(": 1577836800:0;git push\n");
assert_eq!(zsh_entries[0].timestamp, Some(1577836800));

let fish_entries = FishHistoryParser::new().parse("- cmd: ls\n  when: 1577836800\n");
let shell = detect_shell("- cmd: ls\n");
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
