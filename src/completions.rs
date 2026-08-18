//! Shell completion script generation (bash, zsh, fish).
//!
//! The `FLAGS` list must be updated whenever new CLI flags are added to `main.rs`.

use crate::shell::Shell;

/// All flags the CLI accepts, for embedding in completion scripts.
const FLAGS: &[&str] = &[
    "--path",
    "--shell",
    "--top",
    "--ignore",
    "--no-default-ignore",
    "--no-strip",
    "--collapse",
    "--min",
    "--grep",
    "--depth",
    "--since",
    "--until",
    "--asc",
    "--bars",
    "--percent",
    "--json",
    "--csv",
    "--stats",
    "--trend",
    "--trend-bucket",
    "--hourly",
    "--last-used",
    "--output",
    "--completions",
    "--man",
    "--help",
    "--version",
];

fn opts_line() -> String {
    FLAGS.join(" ")
}

/// Generate a completion script for the given shell.
pub fn completions(shell: Shell) -> String {
    match shell {
        Shell::Bash => bash(),
        Shell::Zsh => zsh(),
        Shell::Fish => fish(),
    }
}

fn bash() -> String {
    format!(
        r#"_shellist() {{
    local cur prev opts
    cur="${{COMP_WORDS[COMP_CWORD]}}"
    prev="${{COMP_WORDS[COMP_CWORD-1]}}"
    opts="{opts}"

    case "$prev" in
        --shell|--completions)
            COMPREPLY=( $(compgen -W "bash zsh fish" -- "$cur") )
            return 0 ;;
        --trend-bucket)
            COMPREPLY=( $(compgen -W "day week month" -- "$cur") )
            return 0 ;;
        --path|--output)
            COMPREPLY=( $(compgen -f -- "$cur") )
            return 0 ;;
        --top|--min|--depth|--ignore|--grep|--since|--until)
            return 0 ;;
    esac

    if [[ "$cur" == -* ]]; then
        COMPREPLY=( $(compgen -W "$opts" -- "$cur") )
        return 0
    fi
}}
complete -F _shellist shellist
"#,
        opts = opts_line()
    )
}

fn zsh() -> String {
    String::from(
        r#"#compdef shellist

_shellist() {
    local -a opts
    opts=(
        '--top[Show top N commands]:N:'
        '--ignore[Exclude commands (comma-separated)]:list:'
        '--no-default-ignore[Do not filter bash internals (set, shopt)]'
        '--no-strip[Do not strip leading sudo / VAR=val prefixes]'
        '--collapse[Merge adjacent identical lines before counting]'
        '--min[Only commands with count >= N]:N:'
        '--path[History file path]:file:_files'
        '--shell[Force shell parser]:shell:(bash zsh fish)'
        '--depth[Subcommand depth (tokens)]:N:'
        '--json[Output as JSON]'
        '--csv[Output as CSV]'
        '--bars[Show ASCII bar chart]'
        '--percent[Show percentage column]'
        '--stats[Show summary statistics]'
        '--grep[Regex filter on command names]:pattern:'
        '--asc[Sort ascending]'
        '--since[Start date YYYY-MM-DD or Nd/Nw/Nm]:date:'
        '--until[End date YYYY-MM-DD or Nd/Nw/Nm]:date:'
        '--trend[Show usage over time]'
        '--trend-bucket[Trend bucket]:bucket:(day week month)'
        '--hourly[Show hour-of-day distribution]'
        '--last-used[Add last-run date column]'
        '--output[Write to file]:file:_files'
        '--completions[Print completion script]:shell:(bash zsh fish)'
        '--man[Print man page]'
        '--help[Print help]'
        '--version[Print version]'
    )
    _arguments -s $opts
}

_shellist "$@"
"#,
    )
}

fn fish() -> String {
    let mut s = String::new();
    for flag in FLAGS {
        let desc = description(flag);
        s.push_str(&format!(
            "complete -c shellist -l {} -d \"{}\"",
            flag.trim_start_matches('-'),
            desc
        ));
        match *flag {
            "--path" | "--output" => s.push_str(" -r"),
            "--shell" | "--completions" => s.push_str(" -x -a \"bash zsh fish\""),
            "--trend-bucket" => s.push_str(" -x -a \"day week month\""),
            _ => {}
        }
        s.push('\n');
    }
    s
}

fn description(flag: &str) -> &'static str {
    match flag {
        "--top" => "Show top N commands",
        "--ignore" => "Exclude commands (comma-separated)",
        "--no-default-ignore" => "Do not filter bash internals (set, shopt)",
        "--no-strip" => "Do not strip leading sudo / VAR=val prefixes",
        "--collapse" => "Merge adjacent identical lines before counting",
        "--min" => "Only commands with count >= N",
        "--path" => "History file path",
        "--shell" => "Force shell parser",
        "--depth" => "Subcommand depth (tokens)",
        "--json" => "Output as JSON",
        "--csv" => "Output as CSV",
        "--bars" => "Show ASCII bar chart",
        "--percent" => "Show percentage column",
        "--stats" => "Show summary statistics",
        "--grep" => "Regex filter on command names",
        "--asc" => "Sort ascending",
        "--since" => "Start date YYYY-MM-DD or Nd/Nw/Nm",
        "--until" => "End date YYYY-MM-DD or Nd/Nw/Nm",
        "--trend" => "Show usage over time",
        "--trend-bucket" => "Trend bucket",
        "--hourly" => "Show hour-of-day distribution",
        "--last-used" => "Add last-run date column",
        "--output" => "Write to file",
        "--completions" => "Print completion script",
        "--man" => "Print man page",
        "--help" => "Print help",
        "--version" => "Print version",
        _ => "option",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bash_script_has_complete_directive() {
        let s = completions(Shell::Bash);
        assert!(s.contains("complete -F _shellist shellist"));
        assert!(s.contains("--json"));
    }

    #[test]
    fn zsh_script_has_compdef() {
        let s = completions(Shell::Zsh);
        assert!(s.contains("#compdef shellist"));
        assert!(s.contains("--csv"));
    }

    #[test]
    fn fish_script_has_complete_lines() {
        let s = completions(Shell::Fish);
        assert!(s.contains("complete -c shellist"));
        assert!(s.contains("-l grep"));
    }
}
