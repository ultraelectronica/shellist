//! Man page (groff/troff) generation.

/// Generate the `shellist` man page as groff source.
pub fn man_page() -> String {
    format!(
        r#".TH SHELLIST 1 "{version}" "shellist {version}" "User Commands"
.SH NAME
shellist \- shell history analysis
.SH SYNOPSIS
.B shellist
[\fIOPTIONS\fR]
.SH DESCRIPTION
Parse shell history files (bash, zsh, fish), count commands, and rank them by
frequency. The default reads ~/.bash_history and prints a ranked table.
.PP
When no path is given and stdin is piped (or \fB-\fR is passed), shellist reads
history from stdin. Use \fB--shell\fR to force a parser; otherwise the format is
auto-detected from the contents.
.SH OPTIONS
.TP
.BR \-\-top = N
Show only the top N commands.
.TP
.BR \-\-ignore = X,Y
Exclude commands (comma-separated).
.TP
.BR \-\-no\-default\-ignore
Do not filter bash internals (set, shopt).
.TP
.BR \-\-min = N
Only show commands used at least N times.
.TP
.BR \-\-path = PATH
Read history from PATH instead of the default file.
.TP
.BR \-\-shell = bash | zsh | fish
Force a specific history parser instead of auto-detecting.
.TP
.BR \-\-depth = N
Treat the first N whitespace tokens as the command key (default 1).
.TP
.BR \-\-no\-strip
Do not strip leading \fBsudo\fR and \fBVAR=value\fR tokens from commands
(stripped by default).
.TP
.BR \-\-collapse
Merge adjacent identical raw lines (arrow-key reuse) before counting.
.TP
.BR \-\-json
Output results as a JSON array.
.TP
.BR \-\-csv
Output results as CSV.
.TP
.BR \-\-bars
Add an ASCII bar chart column.
.TP
.BR \-\-percent
Add a percentage column.
.TP
.BR \-\-stats
Print summary statistics instead of the table.
.TP
.BR \-\-grep = PATTERN
Filter command names with a regular expression (case-insensitive).
.TP
.BR \-\-asc
Sort results in ascending order.
.TP
.BR \-\-since = DATE
Only count commands on or after this date (requires timestamps).
DATE is \fIYYYY-MM-DD\fR or a relative \fINd\fR / \fINw\fR / \fINm\fR (e.g. \fB7d\fR), resolved against now (UTC).
.TP
.BR \-\-until = DATE
Only count commands on or before this date (same formats, requires timestamps).
.TP
.BR \-\-trend
Show command counts bucketed over time (UTC-based).
.TP
.BR \-\-trend-bucket = day | week | month
Bucket granularity for --trend (default: day).
Also accepts \fBdaily\fP, \fBweekly\fP, \fBmonthly\fP.
.TP
.BR \-\-output = FILE
Write output to FILE instead of stdout.
.TP
.BR \-\-completions = bash | zsh | fish
Print a shell completion script to stdout.
.TP
.BR \-\-man
Print this man page to stdout.
.TP
.BR \-\-help
Print help to stdout.
.TP
.BR \-\-version
Print version and exit.
.SH EXAMPLES
.PP
shellist --top 10
.PP
shellist --shell zsh --path ~/.zsh_history --json
.PP
shellist --depth 2 --bars --percent
.PP
cat ~/.bash_history | shellist --stats
.SH SEE ALSO
.BR bash (1),
.BR zsh (1),
.BR fish (1)
.SH BUGS
Report issues at https://github.com/ultraelectronica/shellist/issues
"#,
        version = env!("CARGO_PKG_VERSION")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn man_page_has_title_and_options() {
        let m = man_page();
        assert!(m.contains(".TH SHELLIST 1"));
        assert!(m.contains(".SH OPTIONS"));
        assert!(m.contains("\\-\\-json"));
        assert!(m.contains("\\-\\-grep"));
    }
}
