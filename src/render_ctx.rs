use std::env;
use std::io::IsTerminal;

/// Rendering mode - controls whether ANSI escape codes are emitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    Plain,
    Ansi,
}

/// Context passed to every CLI render call.
#[derive(Debug, Clone)]
pub struct RenderContext {
    pub mode: RenderMode,
    pub width: Option<usize>,
}

impl RenderContext {
    /// Auto-detect from stdout: Ansi if stdout is a TTY, plain otherwise.
    pub fn default_stdout() -> Self {
        Self {
            mode: if std::io::stdout().is_terminal() {
                RenderMode::Ansi
            } else {
                RenderMode::Plain
            },
            width: env::var("COLUMNS")
                .ok()
                .and_then(|v| v.parse::<usize>().ok())
                .filter(|&v| v > 0),
        }
    }

    pub fn plain() -> Self {
        Self {
            mode: RenderMode::Plain,
            width: None,
        }
    }

    pub fn ansi(width: usize) -> Self {
        Self {
            mode: RenderMode::Ansi,
            width: Some(width),
        }
    }

    pub fn is_ansi(&self) -> bool {
        self.mode == RenderMode::Ansi
    }
}
