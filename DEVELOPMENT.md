# Shellist — Development Phases

## Phase 1: Core Parsing Engine

**Goal:** Read `.bash_history` and extract commands cleanly.

- [X] Create `HistoryEntry` struct
  ```rust
  pub struct HistoryEntry {
      pub raw: String,
      pub command: String,
  }
  ```
- [X] Implement `parse_history(input: &str) -> Vec<HistoryEntry>`
- [X] Handle empty lines, leading/trailing whitespace
- [X] Extract first token as the command (e.g. `git commit -m "msg"` → `git`)

---

## Phase 2: Command Aggregation

**Goal:** Count command usage.

- [X] Implement `count_commands(entries: &[HistoryEntry]) -> HashMap<String, usize>`
- [X] Normalize: lowercase commands, trim spaces

---

## Phase 3: Ranking System

**Goal:** Sort commands by frequency.

- [X] Implement `rank_commands(map: HashMap<String, usize>) -> Vec<(String, usize)>`
- [X] Sort descending by count
- [X] Tie-break alphabetically (optional)

---

## Phase 4: Filtering & Options

**Goal:** Make the library flexible.

- [X] `top_n(commands, n)` — return top N results
- [X] `filter_commands(commands, ignore)` — ignore list
- [X] Min frequency threshold

---

## Phase 5: File Input Support

**Goal:** Make it practical.

- [X] `load_history_file(path: &str) -> io::Result<String>`
- [X] Default path support (`~/.bash_history`)

---

## Phase 6: Public API Design

**Goal:** Clean, composable interface.

- [X] `analyze(input: &str) -> Vec<(String, usize)>` — full pipeline entry point

---

## Phase 7: Testing

**Goal:** Correctness.

- [X] Unit tests for parsing
- [X] Unit tests for counting
- [X] Unit tests for sorting
- [X] Integration test: `analyze("ls\nls\ngit\n")` → `ls` ranked first

---

## Phase 8: Performance

**Goal:** Handle large history files.

- [X] Avoid unnecessary allocations
- [X] Use iterators over cloning
- [X] Benchmark with large inputs

---

## Phase 9: Documentation

**Goal:** Usable and readable.

- [X] `///` doc comments on all public items
- [X] Example usage in doc comments
- [X] Crate-level documentation

---

## Phase 10: CLI Layer

**Goal:** Standalone binary.

- [X] Argument parsing (`std::env::args`)
- [X] Pretty output:
  ```
  1. ls     120
  2. git    95
  3. cd     80
  ```
- [X] Depend on `shellist` library via path
