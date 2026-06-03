use std::borrow::Cow;

use crate::render_ctx::{RenderContext, RenderMode};

/// An inline progress bar: `label  [████░░░░░░]  57%`.
pub struct Progress<'a> {
    label: Cow<'a, str>,
    fraction: f64,
    bar_width: usize,
}

impl<'a> Progress<'a> {
    pub fn new(label: impl Into<Cow<'a, str>>, fraction: f64) -> Self {
        Self {
            label: label.into(),
            fraction: fraction.clamp(0.0, 1.0),
            bar_width: 20,
        }
    }

    pub fn bar_width(mut self, w: usize) -> Self {
        self.bar_width = w.max(4);
        self
    }

    pub fn render(&self, ctx: &RenderContext) -> String {
        let filled = (self.fraction * self.bar_width as f64).round() as usize;
        let empty = self.bar_width.saturating_sub(filled);
        let pct = (self.fraction * 100.0) as u32;

        match ctx.mode {
            RenderMode::Plain => {
                let bar = format!("[{}{}]", "#".repeat(filled), ".".repeat(empty));
                format!("{} {} {}%", self.label, bar, pct)
            }
            RenderMode::Ansi => {
                use nu_ansi_term::{Color, Style};
                let filled_str = Style::new()
                    .fg(Color::Rgb(0xe0, 0x90, 0x00))
                    .paint("█".repeat(filled))
                    .to_string();
                let empty_str = Style::new().dimmed().paint("░".repeat(empty)).to_string();
                let bar = format!("[{filled_str}{empty_str}]");
                let pct_str = Style::new()
                    .fg(Color::Rgb(0xe0, 0x90, 0x00))
                    .paint(format!("{pct}%"))
                    .to_string();
                format!("{} {} {}", self.label, bar, pct_str)
            }
        }
    }
}
