# vfp_tuikit

[![crates.io](https://img.shields.io/crates/v/vfp_tuikit.svg)](https://crates.io/crates/vfp_tuikit)
[![docs.rs](https://docs.rs/vfp_tuikit/badge.svg)](https://docs.rs/vfp_tuikit)
[![license](https://img.shields.io/crates/l/vfp_tuikit.svg)](https://github.com/victoryforphil/tui_kit/blob/master/LICENSE)

Small Rust terminal UI primitives for scripts, tools, and interactive TUI flows.

- `cli` is the default feature set for inline output in pipes.
- `tui` adds the fullscreen picker built on `crossterm`.
- Version: `0.1.0`

## Install

Use this crate as a dependency:

```toml
[dependencies]
vfp_tuikit = "0.1"
```

Install the example binary from this checkout:

```bash
cargo install vfp_tuikit
cargo install --path . --bin demo --features tui
```

Install the example binary from Git:

```bash
cargo install --git https://github.com/victoryforphil/tui_kit --bin demo --features tui
```

## Quick Start

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

## Guides

- Run the demo: `cargo run --bin demo`
- Run the TUI picker: `cargo run --features tui --bin demo -- --picker`
- Generate docs: `cargo doc --all-features --no-deps --open`
- Run checks: `cargo fmt`, `cargo clippy --all-features --all-targets -- -D warnings`, `cargo test --all-features`

## Features

- `ActionLine` for `[ LABEL ] message` status lines.
- `Badge` for inline pills.
- `Panel` for bordered sections.
- `Progress` for simple progress bars.
- `Spacer` and `Summary` for layout.
- `render_table()` for compact tables.
- `CliPicker` for interactive selection.

<details>
<summary>For agents</summary>

### Install this repo locally

Clone the repo and use the source checkout directly:

```bash
git clone https://github.com/victoryforphil/tui_kit.git
cd tui_kit
cargo test --all-features
```

### Install the bundled skills

The repo stores reusable OpenCode skills in `.agents/skills/`.

Create the target directory and copy the skill files with `curl`:

```bash
mkdir -p ~/.agents/skills/vfp-tuikit
curl -fsSL https://raw.githubusercontent.com/victoryforphil/tui_kit/master/.agents/skills/vfp-tuikit/SKILL.md \
  -o ~/.agents/skills/vfp-tuikit/SKILL.md

mkdir -p ~/.agents/skills/install-vfp-tuikit-skills
curl -fsSL https://raw.githubusercontent.com/victoryforphil/tui_kit/master/.agents/skills/install-vfp-tuikit-skills/SKILL.md \
  -o ~/.agents/skills/install-vfp-tuikit-skills/SKILL.md
```

GitHub path form works too if you prefer to inspect before copying:

```text
https://github.com/victoryforphil/tui_kit/blob/master/.agents/skills/vfp-tuikit/SKILL.md
https://github.com/victoryforphil/tui_kit/blob/master/.agents/skills/install-vfp-tuikit-skills/SKILL.md
```

### Notes for agents

- Keep the crate at `0.1.0` until the API settles.
- Prefer `anyhow` for demo/app errors and `thiserror` for library errors.
- The TUI picker requires a real TTY.

</details>
