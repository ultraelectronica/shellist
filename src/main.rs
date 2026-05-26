//! Shellist — shell history analysis CLI.
//!
//! Reads `.bash_history`, ranks commands by frequency, prints a table.
//!
//! ```text
//! $ shellist
//! Rank  Command  Count
//! 1     ls       120
//! 2     git      95
//! 3     cd       80
//! ```
//!
//! Options:
//! - `--top N`       Show only the top N commands
//! - `--ignore X,Y`  Exclude commands (comma-separated)
//! - `--min N`       Only commands with count >= N
//! - `--path P`      Read from `P` instead of `~/.bash_history`
//! - `--help`        Print this

use std::env;
use std::process;

use shellist::{
    analyze, default_history_path, filter_by_min_frequency, filter_commands, load_history_file,
    top_n,
};

struct Args {
    top: Option<usize>,
    ignore: Vec<String>,
    min_freq: Option<usize>,
    path: Option<String>,
}

fn parse_args() -> Args {
    let mut args = Args {
        top: None,
        ignore: Vec::new(),
        min_freq: None,
        path: None,
    };

    let mut iter = env::args().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--help" => {
                print_help();
                process::exit(0);
            }
            "--top" => {
                let val = iter.next().unwrap_or_else(|| {
                    eprintln!("shellist: --top requires a number");
                    process::exit(1);
                });
                args.top = Some(val.parse().unwrap_or_else(|_| {
                    eprintln!("shellist: --top expects a number, got '{val}'");
                    process::exit(1);
                }));
            }
            "--ignore" => {
                let val = iter.next().unwrap_or_else(|| {
                    eprintln!("shellist: --ignore requires a comma-separated list");
                    process::exit(1);
                });
                args.ignore = val.split(',').map(|s| s.trim().to_string()).collect();
            }
            "--min" => {
                let val = iter.next().unwrap_or_else(|| {
                    eprintln!("shellist: --min requires a number");
                    process::exit(1);
                });
                args.min_freq = Some(val.parse().unwrap_or_else(|_| {
                    eprintln!("shellist: --min expects a number, got '{val}'");
                    process::exit(1);
                }));
            }
            "--path" => {
                let val = iter.next().unwrap_or_else(|| {
                    eprintln!("shellist: --path requires a file path");
                    process::exit(1);
                });
                args.path = Some(val);
            }
            other => {
                eprintln!("shellist: unknown flag '{other}'");
                process::exit(1);
            }
        }
    }

    args
}

fn print_help() {
    eprintln!(
        "shellist — shell history analysis

USAGE:
    shellist [OPTIONS]

OPTIONS:
    --top N        Show only the top N commands
    --ignore X,Y   Exclude commands (comma-separated)
    --min N        Only commands used at least N times
    --path PATH    Read from PATH instead of ~/.bash_history
    --help         Print this help"
    );
}

fn print_table(commands: &[(String, usize)]) {
    if commands.is_empty() {
        return;
    }

    let rank_width = commands.len().to_string().len().max(4);
    let cmd_width = commands
        .iter()
        .map(|(c, _)| c.len())
        .max()
        .unwrap_or(7)
        .max(7);
    let count_width = commands
        .iter()
        .map(|(_, n)| n.to_string().len())
        .max()
        .unwrap_or(5)
        .max(5);

    println!(
        "{:>rw$}  {:<cw$}  {:>nw$}",
        "Rank",
        "Command",
        "Count",
        rw = rank_width,
        cw = cmd_width,
        nw = count_width
    );
    println!(
        "{:-<rw$}  {:-<cw$}  {:-<nw$}",
        "",
        "",
        "",
        rw = rank_width,
        cw = cmd_width,
        nw = count_width
    );

    for (i, (cmd, count)) in commands.iter().enumerate() {
        println!(
            "{:>rw$}  {:<cw$}  {:>nw$}",
            i + 1,
            cmd,
            count,
            rw = rank_width,
            cw = cmd_width,
            nw = count_width
        );
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args();

    let content = if let Some(ref path) = args.path {
        load_history_file(path)?
    } else {
        let path = default_history_path();
        load_history_file(path.to_str().unwrap())?
    };

    let mut ranked = analyze(&content);

    if !args.ignore.is_empty() {
        ranked = filter_commands(ranked, &args.ignore);
    }
    if let Some(min) = args.min_freq {
        ranked = filter_by_min_frequency(ranked, min);
    }
    if let Some(n) = args.top {
        ranked = top_n(ranked, n);
    }

    print_table(&ranked);
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("shellist: {e}");
        process::exit(1);
    }
}
