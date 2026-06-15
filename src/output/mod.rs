//! Output formatting: tables, JSON, CSV, stats, and trend charts.

pub mod csv;
pub mod json;
pub mod stats;
pub mod table;
pub mod trend;

pub use csv::format_csv;
pub use json::format_json;
pub use stats::{Stats, compute_stats, format_stats};
pub use table::{TableOptions, format_table};
pub use trend::{compute_trend, format_trend};

/// Column alignment for the shared table renderer.
#[derive(Clone, Copy)]
pub(crate) enum Align {
    Left,
    Right,
}

/// Render headers + rows into an aligned, column-padded text table.
pub(crate) fn render_table(headers: &[String], rows: &[Vec<String>], aligns: &[Align]) -> String {
    let cols = headers.len();
    let mut widths = vec![0usize; cols];
    for (i, h) in headers.iter().enumerate() {
        widths[i] = widths[i].max(h.chars().count());
    }
    for row in rows {
        for (i, width) in widths.iter_mut().enumerate() {
            let len = row.get(i).map(|s| s.chars().count()).unwrap_or(0);
            *width = (*width).max(len);
        }
    }

    let pad = |cells: &[String], fill: char| -> String {
        let mut line = String::new();
        for i in 0..cols {
            if i > 0 {
                line.push_str("  ");
            }
            let cell = cells.get(i).map(String::as_str).unwrap_or("");
            if fill == '-' {
                for _ in 0..widths[i] {
                    line.push('-');
                }
            } else {
                let pad = widths[i].saturating_sub(cell.chars().count());
                match aligns[i] {
                    Align::Left => {
                        line.push_str(cell);
                        for _ in 0..pad {
                            line.push(' ');
                        }
                    }
                    Align::Right => {
                        for _ in 0..pad {
                            line.push(' ');
                        }
                        line.push_str(cell);
                    }
                }
            }
        }
        line
    };

    let mut out = String::new();
    out.push_str(&pad(headers, ' '));
    out.push('\n');
    out.push_str(&pad(&vec![String::new(); cols], '-'));
    out.push('\n');
    for row in rows {
        out.push_str(&pad(row, ' '));
        out.push('\n');
    }
    out
}
