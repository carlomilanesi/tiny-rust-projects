fn main() {
    for arg in std::env::args() {
        print!("[{}]", arg);
    }
}
