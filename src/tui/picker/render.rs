use std::io::{self, IsTerminal, Write};

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{self, ClearType};
use crossterm::{execute, queue};
use nu_ansi_term::{Color, Style};
use thiserror::Error;

use crate::cli::{ActionLine, Panel, Summary};
use crate::palette::Tone;
use crate::render_ctx::RenderContext;

use super::state::{PickerItem, PickerOutcome, PickerState};

const MAX_VISIBLE: usize = 10;

/// Full-screen interactive fuzzy picker, built on raw crossterm.
pub struct CliPicker {
    title: String,
    items: Vec<PickerItem>,
}

#[derive(Debug, Error)]
pub enum PickerError {
    #[error("CliPicker requires a terminal (TTY)")]
    NotATty,
    #[error("CliPicker: no items to select from")]
    EmptyItems,
    #[error(transparent)]
    Io(#[from] io::Error),
}

impl CliPicker {
    pub fn new(title: impl Into<String>, items: Vec<PickerItem>) -> Self {
        Self {
            title: title.into(),
            items,
        }
    }

    pub fn run(self) -> Result<PickerOutcome, PickerError> {
        if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
            return Err(PickerError::NotATty);
        }
        if self.items.is_empty() {
            return Err(PickerError::EmptyItems);
        }

        let mut state = PickerState::new(self.title, self.items);
        let mut stdout = io::stdout();

        terminal::enable_raw_mode()?;
        execute!(
            stdout,
            terminal::EnterAlternateScreen,
            terminal::Clear(ClearType::All),
            MoveTo(0, 0),
            Hide,
        )?;

        let result = event_loop(&mut state, &mut stdout);

        terminal::disable_raw_mode()?;
        let _ = execute!(stdout, Show, terminal::LeaveAlternateScreen);

        result
    }
}

fn event_loop(
    state: &mut PickerState,
    stdout: &mut io::Stdout,
) -> Result<PickerOutcome, PickerError> {
    draw_frame(state, stdout)?;

    loop {
        match event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => return Ok(PickerOutcome::Cancelled),

            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                return Ok(match state.selected_key() {
                    Some(k) => PickerOutcome::Selected(k.to_owned()),
                    None => PickerOutcome::Cancelled,
                });
            }

            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char('k'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => state.move_up(),

            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char('j'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => state.move_down(),

            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) => state.pop_char(),

            Event::Key(KeyEvent {
                code: KeyCode::Char(ch),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                ..
            }) => state.push_char(ch),

            _ => continue,
        }

        draw_frame(state, stdout)?;
    }
}

fn draw_frame(state: &PickerState, stdout: &mut io::Stdout) -> Result<(), PickerError> {
    let ctx = RenderContext::default_stdout();
    let frame = build_frame(state, &ctx);

    queue!(stdout, MoveTo(0, 0), terminal::Clear(ClearType::All), Hide)?;
    let lines: Vec<&str> = frame.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        let last = i + 1 == lines.len();
        queue!(stdout, terminal::Clear(ClearType::CurrentLine), Print(line))?;
        if !last {
            queue!(stdout, Print("\r\n"))?;
        }
    }
    stdout.flush()?;
    Ok(())
}

fn build_frame(state: &PickerState, ctx: &RenderContext) -> String {
    let mut body = Vec::new();
    body.push(filter_line(state));
    body.push(separator());
    body.extend(items(state));
    body.push(String::new());
    body.push(help_line(state));

    let action = ActionLine::new("SELECT", &state.title, Tone::Info).render(ctx);
    let panel = Panel::new(body.join("\n"))
        .title("Selection")
        .tone(Tone::Info)
        .render(ctx);

    let total = state.visible_count();
    let summary_text = if state.query.is_empty() {
        format!("{total} option(s)")
    } else {
        format!("{total}/{} matching", state.items.len())
    };
    let summary = Summary::new(summary_text).render(ctx);

    format!("{action}\n\n{panel}\n\n{summary}")
}

fn filter_line(state: &PickerState) -> String {
    let prompt = Style::new()
        .fg(Color::Rgb(0xe0, 0x90, 0x00))
        .bold()
        .paint("›");
    if state.query.is_empty() {
        let ph = Style::new().dimmed().paint("type to filter...");
        return format!("{prompt} {ph}");
    }
    format!("{prompt} {}", state.query)
}

fn separator() -> String {
    Style::new()
        .dimmed()
        .paint("──────────────────────────────")
        .to_string()
}

fn items(state: &PickerState) -> Vec<String> {
    let total = state.visible_count();
    if total == 0 {
        return vec![Style::new()
            .fg(Color::Yellow)
            .paint("No matches")
            .to_string()];
    }

    let (start, end) = scroll_window(state.cursor, total, MAX_VISIBLE);
    let dim = Style::new().dimmed();
    let mut lines = Vec::new();

    if start > 0 {
        lines.push(dim.paint(format!("↑ {start} more above")).to_string());
    }
    for vi in start..end {
        let idx = state.filtered_indices[vi];
        let label = &state.items[idx].label;
        if vi == state.cursor {
            let arrow = Style::new()
                .fg(Color::Rgb(0xe0, 0x90, 0x00))
                .bold()
                .paint("❯");
            let text = Style::new().fg(Color::White).bold().paint(label.as_str());
            lines.push(format!("{arrow} {text}"));
        } else {
            lines.push(format!("  {label}"));
        }
    }
    if end < total {
        lines.push(
            dim.paint(format!("↓ {} more below", total - end))
                .to_string(),
        );
    }
    lines
}

fn help_line(state: &PickerState) -> String {
    let base = if state.visible_count() == 1 {
        "[↑↓] navigate  [Enter] confirm  [Esc] cancel"
    } else {
        "[↑↓] navigate  [Enter] select  [Esc] cancel"
    };
    Style::new().dimmed().paint(base).to_string()
}

fn scroll_window(cursor: usize, total: usize, max: usize) -> (usize, usize) {
    if total <= max {
        return (0, total);
    }
    let half = max / 2;
    let start = if cursor <= half {
        0
    } else if cursor + half >= total {
        total.saturating_sub(max)
    } else {
        cursor - half
    };
    let end = (start + max).min(total);
    (start, end)
}
