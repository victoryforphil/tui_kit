---
name: tui-kit
description: Use when working on the tui_kit crate, example binary, or picker output helpers.
---

# tui_kit

Use this skill when editing or reviewing the standalone `tui_kit` crate.

## Workflow

- Keep CLI renderers pure and return `String`.
- Use `anyhow` in `src/bin/demo.rs` and `thiserror` for library errors.
- Run `cargo fmt`, `cargo clippy --all-features --all-targets -- -D warnings`,
  `cargo test --all-features`, and `cargo doc --all-features --no-deps`.

## Layout

- `src/cli/` - inline output helpers.
- `src/tui/picker/` - fullscreen picker.
- `docs/designs/tui_kit.md` - architecture notes.
