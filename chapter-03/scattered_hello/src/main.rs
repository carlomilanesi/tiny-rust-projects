use crossterm::{
    cursor::MoveTo,
    style::Print,
    terminal::{Clear, ClearType},
    QueueableCommand, Result,
};
use std::io::{stdin, stdout, Write};

fn main() -> Result<()> {
    stdout()
        .queue(Clear(ClearType::All))?
        .queue(MoveTo(0, 0))?
        .queue(Print("X"))?
        .queue(MoveTo(30, 3))?
        .queue(Print("Hello"))?
        .queue(MoveTo(10, 5))?
        .queue(Print("world"))?
        .flush()?;
    stdin().read_line(&mut "".to_string())?;
    Ok(())
}
