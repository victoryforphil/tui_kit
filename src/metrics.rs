/// Layout and spacing constants for the Forge theme.
#[derive(Debug, Clone)]
pub struct Metrics {
    /// Padding inside panel borders (spaces on each side of body text).
    pub panel_padding: usize,
    /// Minimum panel width even if body is narrower.
    pub panel_min_width: usize,
    /// Maximum items shown in the picker before scrolling.
    pub picker_max_visible: usize,
}

impl Metrics {
    pub fn forge() -> Self {
        Self {
            panel_padding: 1,
            panel_min_width: 20,
            picker_max_visible: 10,
        }
    }
}
