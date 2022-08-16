use crossterm::{
    cursor::{Hide, MoveTo, Show},
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::size,
    QueueableCommand, Result,
};
use std::fs::File;
use std::io::{stdin, stdout, Error, ErrorKind, Read, Write};

struct Model {
    contents: String,
    lines: Vec<usize>,
    n_cols: u16,
    n_rows: u16,
}

fn load_file() -> Result<Model> {
    let mut model = Model {
        contents: "".to_string(),
        lines: Vec::<usize>::new(),
        n_cols: 9,
        n_rows: 0,
    };
    let filepath = std::env::args().nth(1).ok_or(Error::new(
        ErrorKind::Other,
        "Missing first command-line argument",
    ))?;
    let mut file = File::open(&filepath)?;
    model.contents = String::new();
    file.read_to_string(&mut model.contents)?;
    model.lines = model
        .contents
        .match_indices('\n')
        .map(|(index, _)| index)
        .collect();
    (model.n_cols, model.n_rows) = size()?;
    Ok(model)
}

fn refresh_screen(model: &Model) -> Result<()> {
    let mut out = stdout();
    for row in 0..model.n_rows {
        let current_line_index = row as usize;
        out.queue(MoveTo(0, row))?
            .queue(SetBackgroundColor(Color::DarkBlue))?;
        if current_line_index >= model.lines.len() {
            let width = model.n_cols as usize;
            out.queue(Print(format!("{:width$}", "")))?;
        } else {
            out.queue(Print(' '))?;
            out.queue(SetForegroundColor(Color::Black))?;
            out.queue(SetBackgroundColor(Color::Grey))?;
            let text_width = model.n_cols as usize - 1;
            let line_begin = if current_line_index == 0 {
                0
            } else {
                model.lines[current_line_index - 1] + 1
            };
            let line_end = model.lines[current_line_index];
            out.queue(Print(format!(
                "{:text_width$}",
                model.contents[line_begin..line_end]
                    .chars()
                    .take(text_width)
                    .collect::<String>(),
            )))?;
        }
    }
    out.flush()?;
    Ok(())
}

fn main() -> Result<()> {
    stdout().queue(Hide)?.flush()?;
    let model = load_file()?;
    refresh_screen(&model)?;
    stdin().read(&mut [0u8; 0])?;
    stdout().queue(Show)?;
    Ok(())
}
