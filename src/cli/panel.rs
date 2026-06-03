use std::borrow::Cow;

use crate::ansi_width::{truncate, visible_width};
use crate::cli::action_line::styled;
use crate::palette::{Palette, Tone};
use crate::render_ctx::{RenderContext, RenderMode};

/// A bordered box with optional title and tone-colored border.
pub struct Panel<'a> {
    body: Cow<'a, str>,
    title: Option<Cow<'a, str>>,
    tone: Option<Tone>,
}

impl<'a> Panel<'a> {
    pub fn new(body: impl Into<Cow<'a, str>>) -> Self {
        Self {
            body: body.into(),
            title: None,
            tone: None,
        }
    }

    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = Some(tone);
        self
    }

    pub fn render(&self, ctx: &RenderContext) -> String {
        let body_lines: Vec<&str> = self.body.lines().collect();
        let body_w = body_lines
            .iter()
            .map(|l| visible_width(l))
            .max()
            .unwrap_or(0);
        let title_w = self
            .title
            .as_ref()
            .map(|t| visible_width(t) + 3)
            .unwrap_or(0);

        let mut inner_w = body_w.max(title_w).max(1);
        if let Some(cols) = ctx.width {
            inner_w = inner_w.min(cols.saturating_sub(4).max(1));
        }

        let (tl, tr, bl, br, h, v) = match ctx.mode {
            RenderMode::Ansi => ('╭', '╮', '╰', '╯', '─', '│'),
            RenderMode::Plain => ('+', '+', '+', '+', '-', '|'),
        };

        let p = Palette::forge();
        let border = p.border_style(self.tone);

        let top_fill = build_top(self.title.as_deref(), inner_w + 2, h);
        let top = styled(&format!("{tl}{top_fill}{tr}"), ctx, border);

        let mut lines = vec![top];
        let body_iter: &[&str] = if body_lines.is_empty() {
            &[""]
        } else {
            &body_lines
        };
        for &line in body_iter {
            let pad = inner_w.saturating_sub(visible_width(line));
            let b = styled(&v.to_string(), ctx, border);
            lines.push(format!("{b} {line}{} {b}", " ".repeat(pad)));
        }

        let bottom = styled(
            &format!("{bl}{}{br}", h.to_string().repeat(inner_w + 2)),
            ctx,
            border,
        );
        lines.push(bottom);
        lines.join("\n")
    }
}

fn build_top(title: Option<&str>, width: usize, h: char) -> String {
    match title {
        None => h.to_string().repeat(width),
        Some(t) => {
            let max = width.saturating_sub(3);
            let t = truncate(t, max);
            let prefix = format!("{h} {t} ");
            let fill = width.saturating_sub(prefix.chars().count());
            format!("{prefix}{}", h.to_string().repeat(fill))
        }
    }
}
