use crossterm::terminal::{disable_raw_mode, enable_raw_mode, is_raw_mode_enabled};
use std::io::{Error, ErrorKind, Result};

fn run() -> Result<()> {
    println!("{}", is_raw_mode_enabled()?);
    enable_raw_mode()?;
    println!("{}", is_raw_mode_enabled()?);
    println!("{}", std::env::args().nth(1).unwrap());
    Ok(())
}

fn main() -> Result<()> {
    let result = std::panic::catch_unwind(run);
    disable_raw_mode()?;
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
