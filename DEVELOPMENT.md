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

- [ ] `top_n(commands, n)` — return top N results
- [ ] `filter_commands(commands, ignore)` — ignore list
- [ ] Min frequency threshold

---

## Phase 5: File Input Support

**Goal:** Make it practical.

- [ ] `load_history_file(path: &str) -> io::Result<String>`
- [ ] Default path support (`~/.bash_history`)

---

## Phase 6: Public API Design

**Goal:** Clean, composable interface.

- [ ] `analyze(input: &str) -> Vec<(String, usize)>` — full pipeline entry point

---

## Phase 7: Testing

**Goal:** Correctness.

- [ ] Unit tests for parsing
- [ ] Unit tests for counting
- [ ] Unit tests for sorting
- [ ] Integration test: `analyze("ls\nls\ngit\n")` → `ls` ranked first

---

## Phase 8: Performance

**Goal:** Handle large history files.

- [ ] Avoid unnecessary allocations
- [ ] Use iterators over cloning
- [ ] Benchmark with large inputs

---

## Phase 9: Documentation

**Goal:** Usable and readable.

- [ ] `///` doc comments on all public items
- [ ] Example usage in doc comments
- [ ] Crate-level documentation

---

## Phase 10: CLI Layer (Optional)

**Goal:** Standalone binary.

- [ ] Argument parsing (`std::env::args`)
- [ ] Pretty output:
  ```
  1. ls     120
  2. git    95
  3. cd     80
  ```
- [ ] Depend on `shellist` library via path
