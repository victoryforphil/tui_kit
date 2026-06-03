---
name: vfp-tuikit
description: Use when working on or integrating the vfp_tuikit crate, demo binary, or picker output helpers.
---

# vfp_tuikit

Use this skill when editing, reviewing, or integrating the standalone `vfp_tuikit` crate.

## What it is

- `cli` provides pure `String` renderers for scripts and pipes.
- `tui` adds the fullscreen picker built on `crossterm`.
- `src/bin/demo.rs` is the example CLI entrypoint.

## Install and use

```bash
cargo add vfp_tuikit
cargo run --bin demo
cargo run --features tui --bin demo -- --picker
```

For local checkout installs:

```bash
cargo install --path . --bin demo --features tui
```

## Verify

```bash
cargo fmt
cargo clippy --all-features --all-targets -- -D warnings
cargo test --all-features
cargo doc --all-features --no-deps
```

## Notes

- Keep renderers pure and return `String`.
- Use `anyhow` in the demo binary and `thiserror` in the library.
- The picker requires a real TTY.
