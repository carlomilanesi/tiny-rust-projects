fn main() {
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read a line");
    let line1 = line.trim_end();
    print!("Uppercase is '{}'.", line1.to_uppercase());
    eprint!("Lowercase is '{}'.", line1.to_lowercase());
}
