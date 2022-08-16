use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};
use std::io::{stdout, Write};
use std::time::Duration;

fn main() -> Result<()> {
    enable_raw_mode()?;
    for _ in 0..10 {
        std::thread::sleep(Duration::from_millis(1000));
        print!("{}", "*");
        stdout().flush()?;
        if poll(Duration::ZERO)? {
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Esc => break,
                    _ => {}
                },
                _ => {}
            }
        }
    }
    disable_raw_mode()?;
    println!();
    Ok(())
}
