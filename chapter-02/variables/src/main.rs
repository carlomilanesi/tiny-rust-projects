use regex::Regex;
use std::env::{args, vars};
fn main() {
    if let Some(pattern) = args().nth(1) {
        let re = Regex::new(&pattern).unwrap();
        for v in vars().filter(|v| re.is_match(&v.0)) {
            println!("[{}]=[{}]", v.0, v.1);
        }
    }
}
