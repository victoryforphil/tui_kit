use std::borrow::Cow;

use crate::palette::{Palette, Tone};
use crate::render_ctx::{RenderContext, RenderMode};

/// An inline colored pill: `[ text ]`.
pub struct Badge<'a> {
    text: Cow<'a, str>,
    tone: Tone,
}

impl<'a> Badge<'a> {
    pub fn new(text: impl Into<Cow<'a, str>>, tone: Tone) -> Self {
        Self {
            text: text.into(),
            tone,
        }
    }

    pub fn render(&self, ctx: &RenderContext) -> String {
        match ctx.mode {
            RenderMode::Plain => format!("[{}]", self.text),
            RenderMode::Ansi => {
                let p = Palette::forge();
                let badge = p.badge_style(self.tone).paint(self.text.as_ref());
                format!("[{badge}]")
            }
        }
    }
}
