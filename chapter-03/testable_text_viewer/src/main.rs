use crossterm::{
    cursor::Show,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, Clear, ClearType,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
    QueueableCommand, Result,
};
use std::io::{stdin, stdout, Error, ErrorKind, Write};
use std::panic::catch_unwind;

mod model;

fn main() -> Result<()> {
    let result = catch_unwind(run);
    cleanup_tty()?;
    match result {
        Ok(run_value) => run_value,
        Err(err) => {
            if let Some(string) = err.downcast_ref::<String>() {
                Err(Error::new(ErrorKind::Other, string.as_str()))
            } else if let Some(string_slice) = err.downcast_ref::<&str>() {
                Err(Error::new(ErrorKind::Other, *string_slice))
            } else {
                Err(Error::new(ErrorKind::Other, format!("{:?}", err)))
            }
        }
    }
}

fn cleanup_tty() -> Result<()> {
    if is_raw_mode_enabled()? {
        disable_raw_mode()?;
        stdout()
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
    let mut model = model::Model::new();
    model.interpret_arguments()?;
    model.set_initial_size(&process_next_command)?;
    enable_raw_mode()?;
    stdout().queue(EnterAlternateScreen)?;
    loop {
        model.refresh()?;
        let event = if model::is_input_tty() {
            read()
        } else {
            process_next_command()
        }?;
        match model.interpret_event(event)? {
            model::EventOutcome::Terminate => {
                if !model::is_input_tty() {
                    check_no_more_commands()?;
                }
                break;
            }
            model::EventOutcome::Continue => {}
        }
    }
    Ok(())
}

fn process_next_command() -> Result<Event> {
    let next_command_line = get_next_command_line()?;
    let mut key_code = KeyCode::Null;
    let mut ctrl = false;
    let mut new_cols = 0;
    let mut new_rows = 0;
    for word in next_command_line.split(' ') {
        match word {
            "up" => key_code = KeyCode::Up,
            "down" => key_code = KeyCode::Down,
            "page_up" => key_code = KeyCode::PageUp,
            "page_down" => key_code = KeyCode::PageDown,
            "esc" => key_code = KeyCode::Esc,
            "C" => ctrl = true,
            "q" => key_code = KeyCode::Char('q'),
            _ => {
                if let Some(cols_str) = word.strip_prefix("cols") {
                    new_cols = cols_str.parse().unwrap();
                } else if let Some(rows_str) = word.strip_prefix("rows") {
                    new_rows = rows_str.parse().unwrap();
                } else {
                    panic!("Unexpected command: {word}");
                }
            }
        }
    }
    Ok(if new_cols > 0 && new_rows > 0 {
        Event::Resize(new_cols, new_rows)
    } else {
        let mut modifiers = KeyModifiers::empty();
        if ctrl {
            modifiers |= KeyModifiers::CONTROL;
        }
        Event::Key(KeyEvent::new(key_code, modifiers))
    })
}

fn get_next_command_line() -> Result<String> {
    let mut line = String::new();
    while line.is_empty() {
        stdin().read_line(&mut line)?;
        if line.is_empty() {
            return Err(Error::new(ErrorKind::Other, "Unexpected end of input file"));
        }
        if let Some(comment_index) = line.find('#') {
            line = line[0..comment_index].to_string();
        }
        line = line.trim_end().to_string();
    }
    Ok(line)
}

fn check_no_more_commands() -> Result<()> {
    let mut owned_line = String::new();
    stdin().read_line(&mut owned_line)?;
    if !owned_line.is_empty() {
        return Err(Error::new(
            ErrorKind::Other,
            "Input file continuing after exit command",
        ));
    }
    Ok(())
}
