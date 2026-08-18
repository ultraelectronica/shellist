//! Shellist — shell history analysis CLI.
//!
//! Reads shell history (bash, zsh, fish), ranks commands by frequency, and
//! prints a table. Supports multiple output formats, subcommand depth,
//! regex filtering, date ranges, trends, and shell completions.
//!
//! ```text
//! $ shellist --top 3 --bars
//! Rank  Command  Count  Bars
//! ----  -------  -----  ------------------------------
//!    1  ls         120  ##############################
//!    2  git         95  #######################
//!    3  cd          80  ####################
//! ```

use std::env;
use std::io::{IsTerminal, Read};
use std::process;

use shellist::{
    Bucket, HistoryParser, Shell, TableOptions, command_key, completions, count_commands_at_depth,
    default_history_path, detect_shell, filter_by_min_frequency, filter_commands, format_csv,
    format_hourly, format_json, format_stats, format_table, format_trend, grep_filter,
    last_used_at_depth, load_history_file, man_page, rank_commands, rank_commands_ascending,
    resolve_date_spec, strip_command_prefixes, top_n,
};

use regex::Regex;

/// Bash builtins that often leak into `.bash_history` from shell init scripts.
const DEFAULT_IGNORE: &[&str] = &["set", "shopt"];

struct Args {
    top: Option<usize>,
    ignore: Vec<String>,
    no_default_ignore: bool,
    min_freq: Option<usize>,
    paths: Vec<String>,
    shell: Option<Shell>,
    depth: Option<usize>,
    json: bool,
    csv: bool,
    bars: bool,
    percent: bool,
    last_used: bool,
    stats: bool,
    grep: Option<String>,
    asc: bool,
    since: Option<String>,
    until: Option<String>,
    trend: bool,
    hourly: bool,
    trend_bucket: Option<Bucket>,
    output: Option<String>,
    completions: Option<Shell>,
    man: bool,
    no_strip: bool,
    collapse: bool,
}

impl Args {
    const fn empty() -> Self {
        Self {
            top: None,
            ignore: Vec::new(),
            no_default_ignore: false,
            min_freq: None,
            paths: Vec::new(),
            shell: None,
            depth: None,
            json: false,
            csv: false,
            bars: false,
            percent: false,
            last_used: false,
            stats: false,
            grep: None,
            asc: false,
            since: None,
            until: None,
            trend: false,
            hourly: false,
            trend_bucket: None,
            output: None,
            completions: None,
            man: false,
            no_strip: false,
            collapse: false,
        }
    }
}

fn need_value(iter: &mut impl Iterator<Item = String>, flag: &str) -> String {
    iter.next().unwrap_or_else(|| {
        eprintln!("shellist: {flag} requires a value");
        process::exit(1);
    })
}

fn parse_usize(val: String, flag: &str) -> usize {
    val.parse().unwrap_or_else(|_| {
        eprintln!("shellist: {flag} expects a number, got '{val}'");
        process::exit(1);
    })
}

fn parse_args() -> Args {
    let mut args = Args::empty();
    let mut iter = env::args().skip(1);

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-" => args.paths.push("-".to_string()),
            "--help" => {
                print_help();
                process::exit(0);
            }
            "--version" => {
                println!("shellist {}", env!("CARGO_PKG_VERSION"));
                process::exit(0);
            }
            "--top" => args.top = Some(parse_usize(need_value(&mut iter, "--top"), "--top")),
            "--min" => args.min_freq = Some(parse_usize(need_value(&mut iter, "--min"), "--min")),
            "--depth" => {
                args.depth = Some(parse_usize(need_value(&mut iter, "--depth"), "--depth"))
            }
            "--ignore" => {
                let val = need_value(&mut iter, "--ignore");
                args.ignore = val.split(',').map(|s| s.trim().to_lowercase()).collect();
            }
            "--no-default-ignore" => args.no_default_ignore = true,
            "--no-strip" => args.no_strip = true,
            "--collapse" => args.collapse = true,
            "--path" => {
                let val = need_value(&mut iter, "--path");
                args.paths.extend(
                    val.split(',')
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(str::to_string),
                );
            }
            "--shell" => {
                let val = need_value(&mut iter, "--shell");
                args.shell = Some(parse_shell(&val, "--shell"));
            }
            "--json" => args.json = true,
            "--csv" => args.csv = true,
            "--bars" => args.bars = true,
            "--percent" => args.percent = true,
            "--last-used" => args.last_used = true,
            "--stats" => args.stats = true,
            "--grep" => args.grep = Some(need_value(&mut iter, "--grep")),
            "--asc" => args.asc = true,
            "--since" => args.since = Some(need_value(&mut iter, "--since")),
            "--until" => args.until = Some(need_value(&mut iter, "--until")),
            "--trend" => args.trend = true,
            "--hourly" => args.hourly = true,
            "--trend-bucket" => {
                let val = need_value(&mut iter, "--trend-bucket");
                args.trend_bucket = Some(parse_bucket(&val));
            }
            "--output" => args.output = Some(need_value(&mut iter, "--output")),
            "--completions" => {
                let val = need_value(&mut iter, "--completions");
                args.completions = Some(parse_shell(&val, "--completions"));
            }
            "--man" => args.man = true,
            other => {
                eprintln!("shellist: unknown flag '{other}'");
                process::exit(1);
            }
        }
    }
    args
}

fn parse_shell(val: &str, flag: &str) -> Shell {
    Shell::from_name(val).unwrap_or_else(|| {
        eprintln!("shellist: {flag} expects bash, zsh, or fish, got '{val}'");
        process::exit(1);
    })
}

fn parse_bucket(val: &str) -> Bucket {
    Bucket::from_name(val).unwrap_or_else(|| {
        eprintln!("shellist: --trend-bucket expects day, week, or month, got '{val}'");
        process::exit(1);
    })
}

fn print_help() {
    println!(
        "shellist {version} — shell history analysis

USAGE:
    shellist [OPTIONS]

INPUT:
    --path PATH[,PATH..]  Read history from one or more files (comma-separated,
                           repeatable; default: per-shell file). Each file's
                           format is detected separately, so zsh + fish merge.
    --shell bash|zsh|fish  Force a parser (default: auto-detect)
    -                    Read history from stdin (or pipe in)

FILTERING:
    --top N              Show only the top N commands
    --ignore X,Y         Exclude commands (comma-separated)
    --no-default-ignore  Don't filter bash internals ({ignores})
    --min N              Only commands used at least N times
    --grep PATTERN       Keep commands matching a regex
    --depth N            Treat first N tokens as the command key (default 1)
    --no-strip           Don't strip leading sudo / VAR=val prefixes
    --collapse           Merge adjacent identical lines before counting
    --since DATE         Only on/after DATE (YYYY-MM-DD or Nd/Nw/Nm, needs timestamps)
    --until DATE         Only on/before DATE (same formats, needs timestamps)
    --asc                Sort ascending

OUTPUT:
    --bars               Add an ASCII bar chart column
    --percent            Add a percentage column
    --last-used          Add a last-run date column (table only, needs timestamps)
    --json               Output as JSON
    --csv                Output as CSV
    --stats              Print summary statistics
    --trend              Usage bucketed over time, UTC-based (needs timestamps)
    --trend-bucket day|week|month|daily|weekly|monthly  Bucket for --trend (default: day)
    --hourly             Hour-of-day distribution, UTC-based (needs timestamps)
    --output FILE        Write output to FILE instead of stdout

INTEGRATION:
    --completions bash|zsh|fish  Print a completion script
    --man                Print the man page
    --help               Print this help
    --version            Print version and exit",
        version = env!("CARGO_PKG_VERSION"),
        ignores = DEFAULT_IGNORE.join(", ")
    );
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = parse_args();
    execute(&mut args)
}

fn execute(args: &mut Args) -> Result<(), Box<dyn std::error::Error>> {
    if args.man {
        print!("{}", man_page());
        return Ok(());
    }
    if let Some(shell) = args.completions {
        print!("{}", completions(shell));
        return Ok(());
    }

    let grep_re = match args.grep.as_deref() {
        Some(p) => Some(
            Regex::new(&format!("(?i){p}")).map_err(|e| format!("invalid --grep pattern: {e}"))?,
        ),
        None => None,
    };
    let now = now_secs();
    let since =
        match args.since.as_deref() {
            Some(d) => Some(resolve_date_spec(d, now).ok_or_else(|| {
                format!("invalid --since date '{d}' (use YYYY-MM-DD or Nd/Nw/Nm)")
            })?),
            None => None,
        };
    let until =
        match args.until.as_deref() {
            Some(d) => Some(resolve_date_spec(d, now).ok_or_else(|| {
                format!("invalid --until date '{d}' (use YYYY-MM-DD or Nd/Nw/Nm)")
            })?),
            None => None,
        };

    let contents = read_input(args)?;
    let (output, was_empty) = core_pipeline(&contents, args, grep_re, since, until)?;

    if was_empty {
        let source = source_label(args);
        eprintln!("shellist: no commands to show from {source}");
    } else {
        write_output(&output, args.output.as_deref())?;
    }
    Ok(())
}

fn core_pipeline(
    contents: &[String],
    args: &mut Args,
    grep_re: Option<Regex>,
    since: Option<i64>,
    until: Option<i64>,
) -> Result<(String, bool), Box<dyn std::error::Error>> {
    let mut entries = Vec::new();
    for content in contents {
        let shell = args.shell.unwrap_or_else(|| detect_shell(content));
        entries.extend(match shell {
            Shell::Bash => shellist::DefaultHistoryParser::new().parse(content),
            Shell::Zsh => shellist::ZshHistoryParser::new().parse(content),
            Shell::Fish => shellist::FishHistoryParser::new().parse(content),
        });
    }

    if !args.no_strip {
        for entry in &mut entries {
            let stripped = strip_command_prefixes(&entry.raw);
            if stripped.len() != entry.raw.len() {
                entry.command = stripped.split_whitespace().next().unwrap_or("").to_string();
                entry.raw = stripped.to_string();
            }
        }
    }
    if args.collapse {
        entries.dedup_by(|a, b| a.raw == b.raw);
    }

    let depth = args.depth.unwrap_or(1);

    if since.is_some() || until.is_some() {
        let had_timestamps = entries.iter().any(|e| e.timestamp.is_some());
        entries.retain(|e| match e.timestamp {
            Some(t) => since.is_none_or(|s| t as i64 >= s) && until.is_none_or(|u| t as i64 <= u),
            None => false,
        });
        if entries.is_empty() && !had_timestamps {
            eprintln!(
                "shellist: no timestamped entries for date filter \
                 (need zsh extended, fish, or timestamped history)"
            );
            return Ok((String::new(), false));
        }
        if entries.is_empty() {
            eprintln!("shellist: date range excludes all entries");
            return Ok((String::new(), false));
        }
    }

    if args.trend || args.hourly {
        if let Some(re) = &grep_re {
            entries.retain(|e| command_key(e, depth).is_some_and(|k| re.is_match(&k)));
        }
        let out = if args.hourly {
            format_hourly(&entries)
        } else {
            format_trend(&entries, args.trend_bucket.unwrap_or(Bucket::Day))
        };
        if out.is_empty() {
            eprintln!(
                "shellist: no timestamped entries for --trend/--hourly \
                 (need zsh extended, fish, or timestamped history)"
            );
            return Ok((String::new(), false));
        }
        return Ok((out, false));
    }

    let counts = count_commands_at_depth(&entries, depth);
    let mut ranked = if args.asc {
        rank_commands_ascending(counts)
    } else {
        rank_commands(counts)
    };

    if let Some(re) = grep_re {
        ranked = grep_filter(&ranked, &re);
    }

    let ignore: Vec<String> = if args.no_default_ignore {
        std::mem::take(&mut args.ignore)
    } else {
        let mut merged = DEFAULT_IGNORE
            .iter()
            .map(|s| (*s).to_string())
            .collect::<Vec<_>>();
        merged.extend(std::mem::take(&mut args.ignore));
        merged
    };
    if !ignore.is_empty() {
        ranked = filter_commands(ranked, &ignore);
    }
    if let Some(min) = args.min_freq {
        ranked = filter_by_min_frequency(ranked, min);
    }
    if let Some(n) = args.top {
        ranked = top_n(ranked, n);
    }

    let (output, was_empty) = if args.json {
        (format_json(&ranked), false)
    } else if args.csv {
        (format_csv(&ranked), false)
    } else if args.stats {
        (format_stats(&ranked), false)
    } else if ranked.is_empty() {
        (String::new(), true)
    } else {
        let last = last_used_at_depth(&entries, depth);
        let opts = TableOptions {
            percent: args.percent,
            bars: args.bars,
            last_used: args.last_used.then_some(&last),
        };
        (format_table(&ranked, &opts), false)
    };

    Ok((output, was_empty))
}

fn read_input(args: &Args) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if !args.paths.is_empty() {
        let mut contents = Vec::with_capacity(args.paths.len());
        for path in &args.paths {
            if path == "-" {
                contents.push(read_stdin()?);
            } else {
                contents.push(load_history_file(path)?);
            }
        }
        return Ok(contents);
    }
    if !std::io::stdin().is_terminal() {
        return Ok(vec![read_stdin()?]);
    }
    let path = match args.shell {
        Some(shell) => shell.default_history_path(),
        None => default_history_path(),
    };
    let path =
        path.ok_or("HOME environment variable not set — cannot resolve default history path")?;
    Ok(vec![load_history_file(path)?])
}

fn read_stdin() -> Result<String, Box<dyn std::error::Error>> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    Ok(buf)
}

fn source_label(args: &Args) -> String {
    if !args.paths.is_empty() {
        return args
            .paths
            .iter()
            .map(|p| if p == "-" { "stdin" } else { p.as_str() })
            .collect::<Vec<_>>()
            .join(", ");
    }
    if !std::io::stdin().is_terminal() {
        return "stdin".to_string();
    }
    let path = match args.shell {
        Some(shell) => shell.default_history_path(),
        None => default_history_path(),
    };
    path.map_or_else(
        || "default history path".to_string(),
        |p| p.to_string_lossy().into_owned(),
    )
}

fn write_output(content: &str, output: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    match output {
        Some(path) => std::fs::write(path, content)?,
        None => print!("{content}"),
    }
    Ok(())
}

fn now_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() as i64)
}

fn main() {
    if let Err(e) = run() {
        eprintln!("shellist: {e}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pipeline(content: &str, args: &mut Args, grep: Option<&str>) -> (String, bool) {
        let grep_re = grep.map(|p| Regex::new(&format!("(?i){p}")).unwrap());
        core_pipeline(&[content.to_string()], args, grep_re, None, None).unwrap()
    }

    fn json_output(out: &str) -> String {
        out.replace(['\n', ' '], "")
    }

    #[test]
    fn strips_sudo_and_env_prefixes_by_default() {
        let mut args = Args::empty();
        args.json = true;
        let (out, _) = pipeline(
            "sudo apt install\nFOO=bar ls\napt update\n",
            &mut args,
            None,
        );
        let json = json_output(&out);
        assert!(json.contains("\"command\":\"apt\",\"count\":2"));
        assert!(json.contains("\"command\":\"ls\",\"count\":1"));
        assert!(!json.contains("sudo"));
    }

    #[test]
    fn no_strip_keeps_prefixes() {
        let mut args = Args::empty();
        args.json = true;
        args.no_strip = true;
        let (out, _) = pipeline("sudo apt install\napt update\n", &mut args, None);
        let json = json_output(&out);
        assert!(json.contains("\"command\":\"sudo\",\"count\":1"));
        assert!(json.contains("\"command\":\"apt\",\"count\":1"));
    }

    #[test]
    fn strip_survives_bare_sudo() {
        let mut args = Args::empty();
        args.json = true;
        let (out, _) = pipeline("sudo\nsudo ls\n", &mut args, None);
        let json = json_output(&out);
        assert!(json.contains("\"command\":\"sudo\",\"count\":1"));
        assert!(json.contains("\"command\":\"ls\",\"count\":1"));
    }

    #[test]
    fn collapse_merges_adjacent_duplicates() {
        let mut args = Args::empty();
        args.json = true;
        args.collapse = true;
        let (out, _) = pipeline("ls\nls\nls\ngit\n", &mut args, None);
        let json = json_output(&out);
        assert!(json.contains("\"command\":\"git\",\"count\":1"));
        assert!(json.contains("\"command\":\"ls\",\"count\":1"));
    }

    #[test]
    fn collapse_keeps_non_adjacent_duplicates() {
        let mut args = Args::empty();
        args.json = true;
        args.collapse = true;
        let (out, _) = pipeline("ls\ngit\nls\n", &mut args, None);
        let json = json_output(&out);
        assert!(json.contains("\"command\":\"ls\",\"count\":2"));
    }

    #[test]
    fn trend_applies_grep_before_bucketing() {
        let mut args = Args::empty();
        args.trend = true;
        let content = ": 1577836800:0;git push\n: 1577836800:0;git commit\n\
                      : 1577836800:0;ls\n: 1577923200:0;ls\n";
        let (out, _) = pipeline(content, &mut args, Some("^git$"));
        assert!(out.contains("2020-01-01"));
        assert!(out.contains(" 2 "));
        assert!(!out.contains("2020-01-02"));
    }

    #[test]
    fn trend_grep_respects_depth() {
        let mut args = Args::empty();
        args.trend = true;
        args.depth = Some(2);
        let content = ": 1577836800:0;git push\n: 1577836800:0;git commit\n: 1577836800:0;ls\n";
        let (out, _) = pipeline(content, &mut args, Some("^git commit$"));
        assert!(out.contains("2020-01-01"));
        assert!(out.contains(" 1 "));
    }

    #[test]
    fn hourly_buckets_by_utc_hour() {
        let mut args = Args::empty();
        args.hourly = true;
        // 2020-01-01T00:00:00Z (x2) and 2020-01-01T05:00:00Z.
        let content = ": 1577836800:0;git push\n: 1577836800:0;git commit\n\
                      : 1577854800:0;ls\n";
        let (out, _) = pipeline(content, &mut args, None);
        assert!(out.contains("Hour"));
        assert!(out.contains(" 2 "));
        assert!(out.contains("05"));
    }

    #[test]
    fn last_used_column_shows_max_date() {
        let mut args = Args::empty();
        args.last_used = true;
        // ls on 2020-01-01 (1577836800) and 2020-01-02 (1577923200).
        let content = ": 1577836800:0;ls\n: 1577923200:0;ls\n: 1577836800:0;git\n";
        let (out, _) = pipeline(content, &mut args, None);
        assert!(out.contains("Last Used"));
        assert!(out.contains("2020-01-02"));
    }

    #[test]
    fn multi_file_merges_formats() {
        let mut args = Args::empty();
        args.json = true;
        let zsh = ": 1577836800:0;git push\n".to_string();
        let fish = "- cmd: ls\n  when: 1577836800\n".to_string();
        let grep_re = None;
        let (out, _) = core_pipeline(&[zsh, fish], &mut args, grep_re, None, None).unwrap();
        let json = json_output(&out);
        assert!(json.contains("\"command\":\"git\",\"count\":1"));
        assert!(json.contains("\"command\":\"ls\",\"count\":1"));
    }
}
