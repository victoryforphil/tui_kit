use std::borrow::Cow;

use crate::render_ctx::RenderContext;
use crate::render_ctx::RenderMode;

/// A dimmed `Summary: message` footer line.
pub struct Summary<'a> {
    message: Cow<'a, str>,
}

impl<'a> Summary<'a> {
    pub fn new(message: impl Into<Cow<'a, str>>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn render(&self, ctx: &RenderContext) -> String {
        match ctx.mode {
            RenderMode::Plain => format!("Summary: {}", self.message),
            RenderMode::Ansi => {
                let prefix = nu_ansi_term::Style::new()
                    .fg(nu_ansi_term::Color::Cyan)
                    .paint("Summary:");
                format!("{prefix} {}", self.message)
            }
        }
    }
}
