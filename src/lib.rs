//! `vfp_tuikit` is a small terminal UI component library.
//!
//! It exposes two feature sets:
//! - `cli`: inline ANSI or plain-text renderers for scripts and pipes.
//! - `tui`: a full-screen picker built on `crossterm`.

pub mod ansi_width;
pub mod cli;
pub mod metrics;
pub mod palette;
pub mod render_ctx;
pub mod theme;

#[cfg(feature = "tui")]
pub mod tui;

pub use palette::{Palette, Tone};
pub use render_ctx::{RenderContext, RenderMode};
pub use theme::Theme;
