# Shellist

A Rust library that parses `.bash_history`, shows your most used commands, and ranks them by frequency.

## What it does

- Parses `.bash_history` files
- Extracts and counts command usage
- Ranks commands by frequency
- Supports filtering, top-N, and thresholds

## Usage

```rust
use shellist::analyze;

let result = analyze("ls\ngit\nls\ncd\ngit\nls");
// [("ls", 3), ("git", 2), ("cd", 1)]
```

## License

MIT
