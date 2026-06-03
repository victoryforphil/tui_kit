# Agents Guide - vfp_tuikit

`vfp_tuikit` is a standalone Rust crate for small terminal UI building blocks.

## Layout

- `src/cli/` - inline renderers (`ActionLine`, `Badge`, `Panel`, `Progress`, `Summary`, `Spacer`, tables).
- `src/tui/picker/` - interactive fullscreen picker.
- `src/bin/demo.rs` - example CLI binary.

## Defaults

- Keep the crate at `0.1.0` until the public API settles.
- Prefer `anyhow` for app/demo errors and `thiserror` for library error types.
- Keep files small and focused.

## Commands

```bash
cargo fmt
cargo clippy --all-features --all-targets -- -D warnings
cargo test --all-features
cargo doc --all-features --no-deps
```

## Notes

- `cli` is the default feature set.
- `tui` adds the fullscreen picker and requires a TTY.
- The demo binary should stay minimal and dependency-light.
