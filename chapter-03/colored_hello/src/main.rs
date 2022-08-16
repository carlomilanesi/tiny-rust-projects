use crossterm::{
    cursor::{Hide, MoveTo, Show},
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand, Result,
};
use std::io::{stdin, stdout, Write};

fn main() -> Result<()> {
    stdout()
        .queue(Hide)?
        .queue(SetBackgroundColor(Color::Rgb {
            r: 210,
            g: 224,
            b: 232,
        }))?
        .queue(Clear(ClearType::All))?
        .queue(MoveTo(0, 0))?
        .queue(Print("X"))?
        .queue(Print("Y"))?
        .queue(MoveTo(30, 3))?
        .queue(SetForegroundColor(Color::Yellow))?
        .queue(SetBackgroundColor(Color::Blue))?
        .queue(Print("Hello"))?
        .queue(MoveTo(10, 5))?
        .queue(SetForegroundColor(Color::DarkRed))?
        .queue(Print("world"))?
        .flush()?;
    stdin().read_line(&mut "".to_string())?;
    stdout().queue(Show)?;
    Ok(())
}
