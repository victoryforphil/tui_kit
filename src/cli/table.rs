use crate::ansi_width::visible_width;
use crate::cli::action_line::styled;
use crate::palette::Palette;
use crate::render_ctx::{RenderContext, RenderMode};

/// Render a columnar table with headers, auto-sized columns, and box borders.
pub fn render_table(headers: &[&str], rows: &[Vec<String>], ctx: &RenderContext) -> String {
    if headers.is_empty() {
        return String::new();
    }

    let col_count = headers.len();
    let mut widths: Vec<usize> = headers.iter().map(|h| visible_width(h)).collect();
    for row in rows {
        for (i, cell) in row.iter().enumerate().take(col_count) {
            widths[i] = widths[i].max(visible_width(cell));
        }
    }

    let (tl, tj, tr, ml, mj, mr, bl, bj, br, v, h) = match ctx.mode {
        RenderMode::Ansi => ('┌', '┬', '┐', '├', '┼', '┤', '└', '┴', '┘', '│', '─'),
        RenderMode::Plain => ('+', '+', '+', '+', '+', '+', '+', '+', '+', '|', '-'),
    };

    let p = Palette::forge();
    let bstyle = p.table_border_style();
    let hstyle = p.table_border_style();

    let mut out = Vec::new();
    out.push(border_row(&widths, tl, tj, tr, h, ctx, bstyle));
    out.push(data_row(&widths, headers, v, ctx, bstyle, Some(hstyle)));
    out.push(border_row(&widths, ml, mj, mr, h, ctx, bstyle));
    for row in rows {
        let cells: Vec<String> = (0..col_count)
            .map(|i| row.get(i).cloned().unwrap_or_default())
            .collect();
        let cell_refs: Vec<&str> = cells.iter().map(String::as_str).collect();
        out.push(data_row(&widths, &cell_refs, v, ctx, bstyle, None));
    }
    out.push(border_row(&widths, bl, bj, br, h, ctx, bstyle));
    out.join("\n")
}

fn border_row(
    widths: &[usize],
    left: char,
    join: char,
    right: char,
    h: char,
    ctx: &RenderContext,
    style: nu_ansi_term::Style,
) -> String {
    let mut s = String::new();
    s.push(left);
    for (i, &w) in widths.iter().enumerate() {
        if i > 0 {
            s.push(join);
        }
        s.push_str(&h.to_string().repeat(w + 2));
    }
    s.push(right);
    styled(&s, ctx, style)
}

fn data_row(
    widths: &[usize],
    cells: &[&str],
    v: char,
    ctx: &RenderContext,
    border_style: nu_ansi_term::Style,
    cell_style: Option<nu_ansi_term::Style>,
) -> String {
    let v_str = styled(&v.to_string(), ctx, border_style);
    let mut line = v_str.clone();
    for (i, &w) in widths.iter().enumerate() {
        let cell = cells.get(i).copied().unwrap_or("");
        let pad = w.saturating_sub(visible_width(cell));
        let text = format!(" {cell}{} ", " ".repeat(pad));
        match (ctx.mode, cell_style) {
            (crate::render_ctx::RenderMode::Ansi, Some(s)) => {
                line.push_str(&s.paint(&text).to_string());
            }
            _ => line.push_str(&text),
        }
        line.push_str(&v_str);
    }
    line
}
