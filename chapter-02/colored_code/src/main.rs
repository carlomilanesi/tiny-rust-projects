use crossterm::style::{Color, Stylize};
use std::io::{stdin, BufRead, Result};
fn main() -> Result<()> {
    for line in stdin().lock().lines() {
        //1
        for ch in line?.chars() {
            //2
            let ch_str = ch.to_string(); //3
            print!(
                "{}",
                match ch {
                    '{' | '}' => ch_str.red().bold(), //4
                    '[' | ']' => ch_str
                        .with(Color::Rgb {
                            //5
                            r: 0,
                            g: 180,
                            b: 220,
                        })
                        .bold(),
                    ' ' => ch_str.on(Color::Rgb {
                        //6
                        r: 232,
                        g: 232,
                        b: 232,
                    }),
                    '(' | ')' => ch_str.yellow().dim().bold(), //7
                    _ if ch.is_ascii_alphabetic() => ch_str.dark_green(), //8
                    _ if ch.is_ascii_digit() => ch_str.dark_magenta().italic(),
                    _ => ch_str.blue(),
                }
            );
        }
        println!();
    }
    Ok(())
}
