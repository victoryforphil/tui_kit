use crate::metrics::Metrics;
use crate::palette::Palette;

/// Combined theme: palette + metrics.
#[derive(Debug, Clone)]
pub struct Theme {
    pub palette: Palette,
    pub metrics: Metrics,
}

impl Theme {
    pub fn forge() -> Self {
        Self {
            palette: Palette::forge(),
            metrics: Metrics::forge(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::forge()
    }
}
