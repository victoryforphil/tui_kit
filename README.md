# vfp_tuikit

`vfp_tuikit` is a small Rust library for terminal UI primitives.

- `cli` is the default feature set for inline output in scripts and pipes.
- `tui` adds the fullscreen picker built on `crossterm`.

Version: `0.1.0`

## Features

- `ActionLine` for `[ LABEL ] message` status lines.
- `Badge` for inline pills.
- `Panel` for bordered sections.
- `Progress` for simple progress bars.
- `Spacer` and `Summary` for layout.
- `render_table()` for compact tables.
- `CliPicker` for interactive selection.

## Example

```rust
use vfp_tuikit::cli::{ActionLine, Panel, Progress, render_table};
use vfp_tuikit::{RenderContext, Tone};

let ctx = RenderContext::default_stdout();
println!("{}", ActionLine::new("INFO", "hello", Tone::Info).render(&ctx));
println!("{}", Panel::new("body").title("Title").render(&ctx));
println!("{}", Progress::new("Docs", 0.58).render(&ctx));

let rows = vec![vec!["cli".into(), "inline".into()]];
println!("{}", render_table(&["Feature", "Notes"], &rows, &ctx));
```

## Demo binary

```bash
cargo run --bin demo
cargo run --features tui --bin demo -- --picker
```

## Development

```bash
cargo fmt
cargo clippy --all-features --all-targets -- -D warnings
cargo test --all-features
cargo doc --all-features --no-deps
```
