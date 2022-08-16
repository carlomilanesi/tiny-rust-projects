use crossterm::{
    cursor::{position, MoveTo, Show},
    event::{
        read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEventKind,
    },
    style::Stylize,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
    QueueableCommand, Result,
};
use std::io::{stdout, Error, ErrorKind, Write};

use crossterm::terminal::{is_raw_mode_enabled, EnterAlternateScreen, LeaveAlternateScreen};
use std::panic::catch_unwind;

fn main() -> Result<()> {
    let result = catch_unwind(run);
    cleanup_tty()?;
    match result {
        Ok(Ok(())) => Ok(()),
        Ok(err) => err,
        Err(err) => Err(Error::new(
            ErrorKind::Other,
            if let Some(&message) = err.downcast_ref::<&str>() {
                message
            } else {
                "Unexpected error"
            },
        )),
    }
}

fn cleanup_tty() -> Result<()> {
    if is_raw_mode_enabled()? {
        disable_raw_mode()?;
        stdout()
            .queue(DisableMouseCapture)?
            .queue(SetForegroundColor(Color::White))?
            .queue(SetBackgroundColor(Color::Black))?
            .queue(Clear(ClearType::All))?
            .queue(LeaveAlternateScreen)?
            .queue(Show)?
            .flush()?;
    }
    Ok(())
}

fn run() -> Result<()> {
    let mut addend_1 = String::new();
    let mut addend_2 = String::new();
    let mut sum: u64;

    let (mut n_cols, mut n_rows) = size()?;

    enable_raw_mode()?;

    let max_n_digits = 18;
    let number_width = max_n_digits as usize;
    let label_width = 10;
    let mut cursor_at_1 = true;

    let mut out = stdout();
    out.queue(EnterAlternateScreen)?;
    out.queue(EnableMouseCapture)?;
    loop {
        // Refresh the terminal.
        out.queue(Clear(ClearType::All))?;
        if n_rows >= 3 || n_cols >= label_width + max_n_digits {
            let addend_1_row = n_rows / 6;
            let addend_2_row = n_rows * 3 / 6;
            let sum_row = n_rows * 5 / 6;
            let labels_col = (n_cols - label_width - max_n_digits) / 2;
            sum = addend_1.parse::<u64>().unwrap_or_default()
                + addend_2.parse::<u64>().unwrap_or_default();
            out.queue(ResetColor)?
                .queue(MoveTo(labels_col, addend_1_row))?
                .queue(Print("Addend 1: "))?
                .queue(MoveTo(labels_col, addend_2_row))?
                .queue(Print("Addend 2: "))?
                .queue(MoveTo(labels_col, sum_row))?
                .queue(Print("Sum:      "))?
                .queue(SetForegroundColor(Color::Cyan))?
                .queue(SetBackgroundColor(Color::DarkBlue))?
                .queue(MoveTo(labels_col + label_width, addend_1_row))?
                .queue(Print(format!("{:>number_width$}", addend_1)))?
                .queue(MoveTo(labels_col + label_width, addend_2_row))?
                .queue(Print(format!("{:>number_width$}", addend_2)))?
                .queue(ResetColor)?
                .queue(MoveTo(labels_col + label_width, sum_row))?
                .queue(Print(format!("{:>number_width$}", sum).bold()))?
                .queue(MoveTo(
                    labels_col + label_width + max_n_digits,
                    if cursor_at_1 {
                        addend_1_row
                    } else {
                        addend_2_row
                    },
                ))?;
        }
        out.flush()?;
        // Interpret the command.
        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Esc => break,
                KeyCode::Tab | KeyCode::BackTab => cursor_at_1 = !cursor_at_1,
                KeyCode::Char(ch) if ch.is_ascii_digit() => {
                    if cursor_at_1 {
                        if addend_1.len() < number_width - 1 {
                            addend_1.push(ch);
                        }
                    } else {
                        if addend_2.len() < number_width - 1 {
                            addend_2.push(ch);
                        }
                    }
                }
                KeyCode::Backspace => {
                    if cursor_at_1 {
                        if addend_1.len() > 0 {
                            addend_1.pop();
                        }
                    } else {
                        if addend_2.len() > 0 {
                            addend_2.pop();
                        }
                    }
                }
                _ => {}
            },
            Event::Mouse(event) => {
                if event.kind == MouseEventKind::Down(MouseButton::Left)
                    && event.row != position().unwrap().1
                {
                    cursor_at_1 = !cursor_at_1;
                }
            }
            Event::Resize(width, height) => {
                n_cols = width;
                n_rows = height;
            }
        }
    }
    Ok(())
}
