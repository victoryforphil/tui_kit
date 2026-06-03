---
title: tui_kit
tags: [rust, terminal, tui, cli]
status: active
---
# tui_kit

## Context

`tui_kit` packages a small set of terminal UI primitives that can be reused in
other Rust projects. The goal is to keep output styling and picker behavior in
one crate instead of reimplementing them in each tool.

## Architecture

The crate is split into two surfaces:

- `cli` for inline renderers that return `String`.
- `tui` for the fullscreen picker built on `crossterm`.

Theme data lives in `palette`, `metrics`, and `theme`. Rendering helpers share
`RenderContext` so callers can choose ANSI or plain text behavior.

## Interfaces

- `RenderContext`, `RenderMode`, `Tone`, `Palette`, `Theme`
- `cli::ActionLine`, `Badge`, `Panel`, `Progress`, `Spacer`, `Summary`
- `cli::render_table()`
- `tui::picker::CliPicker`, `PickerItem`, `PickerOutcome`, `PickerError`
- `src/bin/demo.rs` as the example CLI entrypoint

## Trade-offs

- The crate keeps its API small and composable instead of adding a command
  framework or app shell.
- The picker stays dependency-light and returns a simple enum outcome.
- `anyhow` is reserved for the demo binary; `thiserror` handles library errors.

## Current Status

The crate starts at version `0.1.0` and is ready for local use and docs
generation. Publishing still needs GitHub permissions and crates.io auth.
