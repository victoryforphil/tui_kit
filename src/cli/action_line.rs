use std::borrow::Cow;

use nu_ansi_term::Style;

use crate::palette::{Palette, Tone};
use crate::render_ctx::{RenderContext, RenderMode};

/// A `[ LABEL ] message` line with a tone-colored filled badge.
pub struct ActionLine<'a> {
    label: Cow<'a, str>,
    message: Cow<'a, str>,
    tone: Tone,
}

impl<'a> ActionLine<'a> {
    pub fn new(
        label: impl Into<Cow<'a, str>>,
        message: impl Into<Cow<'a, str>>,
        tone: Tone,
    ) -> Self {
        Self {
            label: label.into(),
            message: message.into(),
            tone,
        }
    }

    pub fn render(&self, ctx: &RenderContext) -> String {
        let badge_text = format!("{:<7}", self.label);
        match ctx.mode {
            RenderMode::Plain => format!("[{badge_text}] {}", self.message),
            RenderMode::Ansi => {
                let p = Palette::forge();
                let badge = p.badge_style(self.tone).paint(&badge_text);
                let msg = p.text_style(self.tone).paint(self.message.as_ref());
                format!("[{badge}] {msg}")
            }
        }
    }
}

pub(crate) fn styled(value: &str, ctx: &RenderContext, style: Style) -> String {
    match ctx.mode {
        RenderMode::Plain => value.to_owned(),
        RenderMode::Ansi => style.paint(value).to_string(),
    }
}
