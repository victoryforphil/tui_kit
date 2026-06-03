use anyhow::Result;

use tui_kit::cli::{render_table, ActionLine, Badge, Panel, Progress, Spacer, Summary};
use tui_kit::{RenderContext, Tone};

fn main() -> Result<()> {
    let ctx = RenderContext::default_stdout();

    println!(
        "{}",
        ActionLine::new("INFO", "tui_kit demo", Tone::Info).render(&ctx)
    );
    println!(
        "{}",
        Panel::new("Builds clean terminal output and a small picker.")
            .title("Overview")
            .tone(Tone::Success)
            .render(&ctx)
    );
    println!("{}", Badge::new("CLI", Tone::Warning).render(&ctx));
    println!("{}", Progress::new("Docs", 0.58).render(&ctx));
    println!("{}", Spacer::rule().render(&ctx));

    let rows = vec![
        vec!["cli".to_string(), "inline output".to_string()],
        vec!["tui".to_string(), "fullscreen picker".to_string()],
    ];
    println!("{}", render_table(&["Feature", "Notes"], &rows, &ctx));
    println!(
        "{}",
        Summary::new("run with --picker to try the TUI").render(&ctx)
    );

    if std::env::args().any(|arg| arg == "--picker") {
        run_picker()?;
    }

    Ok(())
}

#[cfg(feature = "tui")]
fn run_picker() -> Result<()> {
    use tui_kit::tui::picker::{CliPicker, PickerItem, PickerOutcome};

    let items = vec![
        PickerItem::new("anchor", "Anchor - receives polls"),
        PickerItem::new("tag", "Tag - initiates ranging"),
        PickerItem::new("relay", "Relay - forwards packets"),
    ];

    match CliPicker::new("Select role", items).run()? {
        PickerOutcome::Selected(key) => println!("picked={key}"),
        PickerOutcome::Cancelled => println!("cancelled"),
    }

    Ok(())
}

#[cfg(not(feature = "tui"))]
fn run_picker() -> Result<()> {
    println!("rebuild with `--features tui` to try the picker");
    Ok(())
}
