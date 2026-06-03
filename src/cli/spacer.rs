use crate::render_ctx::{RenderContext, RenderMode};

/// Blank lines and horizontal rules between output sections.
pub struct Spacer {
    kind: SpacerKind,
}

enum SpacerKind {
    Blank,
    Rule,
}

impl Spacer {
    /// A single blank line.
    pub fn blank() -> Self {
        Self {
            kind: SpacerKind::Blank,
        }
    }

    /// A horizontal rule: `────────────────────────────────`.
    pub fn rule() -> Self {
        Self {
            kind: SpacerKind::Rule,
        }
    }

    pub fn render(&self, ctx: &RenderContext) -> String {
        match self.kind {
            SpacerKind::Blank => String::new(),
            SpacerKind::Rule => {
                let width = ctx.width.unwrap_or(40);
                match ctx.mode {
                    RenderMode::Plain => "-".repeat(width),
                    RenderMode::Ansi => nu_ansi_term::Style::new()
                        .dimmed()
                        .paint("─".repeat(width))
                        .to_string(),
                }
            }
        }
    }
}
