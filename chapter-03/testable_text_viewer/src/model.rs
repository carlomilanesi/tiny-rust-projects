use crossterm::{
    cursor::{Hide, MoveTo},
    event::{Event, KeyCode, KeyModifiers},
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::size,
    tty::IsTty,
    QueueableCommand, Result,
};
use std::cmp::{max, min};
use std::env::args;
use std::fs::File;
use std::io::{stdin, stdout, Error, ErrorKind, Read, Write};

pub enum EventOutcome {
    Continue,
    Terminate,
}

pub struct Model {
    contents: String,
    lines: Vec<usize>,
    first_line_index: usize,
    save_events_stream: Option<File>,
    n_cols: u16,
    n_rows: u16,
    n_digits: usize,
}

impl Model {
    pub fn new() -> Self {
        Self {
            contents: String::new(),
            lines: vec![],
            first_line_index: 0,
            save_events_stream: None,
            n_cols: 0,
            n_rows: 0,
            n_digits: 0,
        }
    }

    pub fn interpret_arguments(&mut self) -> Result<()> {
        let filepath = args()
            .nth(1)
            .ok_or_else(|| Error::new(ErrorKind::Other, "Missing first command-line argument"))?;
        let mut file = File::open(&filepath)?;
        file.read_to_string(&mut self.contents)?;
        self.lines = self
            .contents
            .match_indices('\n')
            .map(|(index, _)| index)
            .collect();
        self.n_digits = format!("{}", self.lines.len()).len();
        if let Some(arg2) = args().nth(2) {
            if &arg2 == "--save-events" {
                self.save_events_stream =
                    Some(File::create(args().nth(3).ok_or_else(|| {
                        Error::new(ErrorKind::Other, "Missing save-file argument")
                    })?)?);
            }
        }
        Ok(())
    }

    pub fn set_initial_size(
        &mut self,
        command_processor: &dyn Fn() -> Result<Event>,
    ) -> Result<()> {
        if is_input_tty() {
            (self.n_cols, self.n_rows) = size()?;
        } else {
            let event = command_processor()?;
            self.interpret_event(event)?;
        }
        if self.save_events_stream.is_some() {
            write_resize_command(&self.save_events_stream, self.n_cols, self.n_rows)?;
        }
        if self.n_rows == 0 {
            Err(Error::new(ErrorKind::Other, "The terminal has no rows"))
        } else if (self.n_cols as usize) < self.n_digits {
            Err(Error::new(
                ErrorKind::Other,
                "The terminal has not enough columns",
            ))
        } else {
            Ok(())
        }
    }

    pub fn refresh(&mut self) -> Result<()> {
        let mut out = stdout();
        for row in 0..self.n_rows {
            let current_line_index = self.first_line_index + row as usize;
            out.queue(MoveTo(0, row))?
                .queue(SetForegroundColor(Color::Cyan))?
                .queue(SetBackgroundColor(Color::DarkBlue))?;
            if current_line_index >= self.lines.len() {
                let width = self.n_cols as usize;
                out.queue(Print(format!("{:width$}", "")))?;
            } else {
                let n_digits = self.n_digits;
                out.queue(Print(format!("{:>n_digits$} ", current_line_index + 1)))?
                    .queue(SetForegroundColor(Color::Black))?
                    .queue(SetBackgroundColor(Color::Grey))?;
                let text_width = self.n_cols as usize - self.n_digits - 1;
                let line_begin = if current_line_index == 0 {
                    0
                } else {
                    self.lines[current_line_index - 1] + 1
                };
                let line_end = self.lines[current_line_index];
                out.queue(Print(format!(
                    "{:text_width$}",
                    self.contents[line_begin..line_end]
                        .chars()
                        .take(text_width)
                        .collect::<String>(),
                )))?;
            }
        }
        out.queue(Hide)?.flush()?;
        Ok(())
    }

    pub fn interpret_event(&mut self, event: Event) -> Result<EventOutcome> {
        match event {
            Event::Key(event) => match event.code {
                KeyCode::Down => self.handle_down(),
                KeyCode::Up => self.handle_up(),
                KeyCode::PageDown => self.handle_page_down(),
                KeyCode::PageUp => self.handle_page_up(),
                KeyCode::Esc => self.handle_esc(),
                KeyCode::Char('q') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.handle_ctrl_q()
                }
                _ => Ok(EventOutcome::Continue),
            },
            Event::Resize(cols, rows) => self.handle_resize(cols, rows),
            _ => Ok(EventOutcome::Continue),
        }
    }

    fn handle_down(&mut self) -> Result<EventOutcome> {
        write_command(&self.save_events_stream, "down")?;
        if self.lines.len() > self.first_line_index + self.n_rows as usize {
            self.first_line_index += 1;
        }
        Ok(EventOutcome::Continue)
    }

    fn handle_up(&mut self) -> Result<EventOutcome> {
        write_command(&self.save_events_stream, "up")?;
        self.first_line_index -= min(1, self.first_line_index);
        Ok(EventOutcome::Continue)
    }

    fn handle_page_down(&mut self) -> Result<EventOutcome> {
        write_command(&self.save_events_stream, "page_down")?;
        if self.lines.len() > self.first_line_index + self.n_rows as usize {
            self.first_line_index += min(
                self.n_rows as usize,
                self.lines.len() - (self.first_line_index + self.n_rows as usize),
            );
        }
        Ok(EventOutcome::Continue)
    }

    fn handle_page_up(&mut self) -> Result<EventOutcome> {
        write_command(&self.save_events_stream, "page_up")?;
        self.first_line_index -= min(self.n_rows as usize, self.first_line_index);
        Ok(EventOutcome::Continue)
    }

    fn handle_esc(&self) -> Result<EventOutcome> {
        write_command(&self.save_events_stream, "esc")?;
        Ok(EventOutcome::Terminate)
    }

    fn handle_ctrl_q(&self) -> Result<EventOutcome> {
        write_command(&self.save_events_stream, "C q")?;
        Ok(EventOutcome::Terminate)
    }

    fn handle_resize(&mut self, cols: u16, rows: u16) -> Result<EventOutcome> {
        if is_input_tty() {
            (self.n_cols, self.n_rows) = size()?;
        } else {
            self.n_cols = cols;
            self.n_rows = rows;
        }
        write_resize_command(&self.save_events_stream, self.n_cols, self.n_rows)?;
        self.first_line_index = min(
            self.first_line_index,
            max(0, self.lines.len() as isize - self.n_rows as isize) as usize,
        );
        Ok(EventOutcome::Continue)
    }
}

fn write_resize_command(save_events_stream: &Option<File>, n_cols: u16, n_rows: u16) -> Result<()> {
    write_command(save_events_stream, &format!("cols{n_cols} rows{n_rows}"))
}

fn write_command(save_events_stream: &Option<File>, command: &str) -> Result<()> {
    if save_events_stream.is_some() {
        let mut stream = save_events_stream.as_ref().unwrap();
        stream.write_all(command.as_bytes())?;
        stream.write_all(b"\n")?;
    }
    Ok(())
}

pub fn is_input_tty() -> bool {
    stdin().is_tty()
}
