use nu_ansi_term::Color as NuColor;
use nu_ansi_term::Style;

/// Semantic tone for coloring output elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Info,
    Success,
    Warning,
    Error,
}

/// Forge dark palette. Single theme - black/gray/orange-gold.
/// Accent: #e09000 (warm orange-gold, terminal colour208).
/// Tone semantic colors: Info=cyan, Success=green, Warning=gold, Error=red.
#[derive(Debug, Clone)]
pub struct Palette {
    pub canvas: NuColor,
    pub shell: NuColor,
    pub panel: NuColor,
    pub widget: NuColor,
    pub hover: NuColor,
    pub active: NuColor,
    pub accent: NuColor,
    pub accent_hover: NuColor,
    pub accent_dim: NuColor,
    pub text: NuColor,
    pub text_secondary: NuColor,
    pub text_muted: NuColor,
    pub separator: NuColor,
    pub popup_border: NuColor,
}

impl Palette {
    /// Forge - the single built-in theme.
    pub fn forge() -> Self {
        Self {
            canvas: NuColor::Rgb(0x12, 0x12, 0x12),
            shell: NuColor::Rgb(0x1c, 0x1c, 0x1c),
            panel: NuColor::Rgb(0x26, 0x26, 0x26),
            widget: NuColor::Rgb(0x33, 0x33, 0x33),
            hover: NuColor::Rgb(0x44, 0x44, 0x44),
            active: NuColor::Rgb(0x55, 0x55, 0x55),
            accent: NuColor::Rgb(0xe0, 0x90, 0x00),
            accent_hover: NuColor::Rgb(0xff, 0xaa, 0x00),
            accent_dim: NuColor::Rgb(0x7a, 0x4e, 0x00),
            text: NuColor::Rgb(0xe4, 0xe4, 0xe4),
            text_secondary: NuColor::Rgb(0xbc, 0xbc, 0xbc),
            text_muted: NuColor::Rgb(0x76, 0x76, 0x76),
            separator: NuColor::Rgb(0x44, 0x44, 0x44),
            popup_border: NuColor::Rgb(0xe0, 0x90, 0x00),
        }
    }

    pub fn badge_style(&self, tone: Tone) -> Style {
        match tone {
            Tone::Info => Style::new().on(NuColor::Cyan).fg(NuColor::Black).bold(),
            Tone::Success => Style::new().on(NuColor::Green).fg(NuColor::Black).bold(),
            Tone::Warning => Style::new().on(self.accent).fg(NuColor::Black).bold(),
            Tone::Error => Style::new().on(NuColor::Red).fg(NuColor::White).bold(),
        }
    }

    pub fn text_style(&self, tone: Tone) -> Style {
        match tone {
            Tone::Info => Style::new().fg(NuColor::Cyan),
            Tone::Success => Style::new().fg(NuColor::Green),
            Tone::Warning => Style::new().fg(self.accent),
            Tone::Error => Style::new().fg(NuColor::Red).bold(),
        }
    }

    pub fn border_style(&self, tone: Option<Tone>) -> Style {
        let color = match tone.unwrap_or(Tone::Info) {
            Tone::Info => NuColor::Cyan,
            Tone::Success => NuColor::Green,
            Tone::Warning => self.accent,
            Tone::Error => NuColor::Red,
        };
        Style::new().fg(color).bold()
    }

    pub fn table_border_style(&self) -> Style {
        Style::new().fg(self.accent).bold()
    }
}
