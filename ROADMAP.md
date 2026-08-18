# Roadmap

Short-term improvement candidates. Check items off when done.

## Quick wins

- [x] `--version` flag
      Print version and exit. Every published CLI needs it.
- [x] Relative dates for `--since` / `--until`
      Accept `7d`, `2w`, `3m` alongside `YYYY-MM-DD`, resolved against now (UTC).
      Day math already exists in `date.rs`.
- [x] `--grep` with `--trend`
      Trend returns early in the pipeline before grep applies. Filter entries
      before bucketing so "usage of git over time" works.
- [x] `sudo` / env-prefix stripping
      `sudo apt install` counts as `sudo`, `FOO=bar cmd` as `FOO=bar`.
      Strip leading `sudo` and `VAR=val` tokens when extracting the base
      command. Add `--no-strip` to disable.
- [x] Consecutive-duplicate collapsing (`--collapse`)
      Merge adjacent identical raw lines (arrow-key reuse) before counting.

## Analysis features

- [x] `--last-used` column
      Show date each command was last run (needs timestamps). Max ts per key.
- [x] `--hourly`
      Hour-of-day distribution (0–23 buckets). Reuses trend machinery.
- [x] Multi-file input
      `--path` accepts comma-separated paths or repeats. Merging zsh + fish
      history into one ranking is the main use case.

## Parser accuracy

- [ ] zsh multi-line commands
      Real `.zsh_history` escapes embedded newlines; each physical line is
      currently counted as a separate entry. Fix in `ZshHistoryParser`.
- [ ] `--ignore` prefix matching
      Exact-match only today, so `--ignore git` misses `git push` at
      `--depth 2`. Either support `git*` prefixes or document the limitation.

## Minor

- [ ] Error on `--json` + `--csv` together (last flag silently wins today)
- [ ] Short flags (`-t` for `--top`, `-n` for `--min`)

## Explicitly skipped (YAGNI)

Markdown output, color/TTY detection, alias expansion, session detection,
atuin import. Revisit only on real demand.
