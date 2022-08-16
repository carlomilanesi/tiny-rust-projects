use crossterm::{style::Stylize, tty::IsTty};
use std::io::{stdin, stdout, BufRead, Result};

fn main() -> Result<()> {
    let use_styles = if let Some(arg) = std::env::args().nth(1) {
        match arg.as_str() {
            "always" => true,
            "never" => false,
            _ => stdout().is_tty(),
        }
    } else {
        stdout().is_tty()
    };
    for line in stdin().lock().lines() {
        for ch in line?.chars() {
            if use_styles && ch.is_digit(10) {
                print!("{}", ch.red().bold());
            } else {
                print!("{}", ch);
            }
        }
        println!();
    }
    Ok(())
}
