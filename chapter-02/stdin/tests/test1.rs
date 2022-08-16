fn get_program_path() -> String {
    format!(
        "{}{}{}",
        if cfg!(debug_assertions) {
            "target/debug/"
        } else {
            "target/release/"
        },
        std::env::current_dir()
            .unwrap()
            .file_name()
            .unwrap()
            .to_string_lossy(),
        if cfg!(windows) { ".exe" } else { "" }
    )
}

use std::io::Write;

#[test]
fn redirect_input_output_error_with_bytes() {
    use std::process::{Command, Stdio};
    let program_path = get_program_path();
    let mut child = Command::new(&program_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect(&format!("Failed to launch the program {}.", program_path));
    let mut stdin = child.stdin.take().expect("Failed to open the stdin stream");
    std::thread::spawn(move || {
        stdin
            .write_all(b"Hi, World!")
            .expect("Failed to write to the stdin stream");
    });
    let output = child
        .wait_with_output()
        .expect("Failed to read the stdout or stderr streams");
    assert_eq!(output.stdout, b"Uppercase is 'HI, WORLD!'.");
    assert_eq!(output.stderr, b"Lowercase is 'hi, world!'.");
}

#[test]
fn redirect_input_output_error_with_utf8() {
    use std::process::{Command, Stdio};
    let program_path = get_program_path();
    let mut child = Command::new(&program_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect(&format!("Failed to launch the program `{}`.", program_path));
    let mut stdin = child.stdin.take().expect("Failed to open the stdin stream");
    std::thread::spawn(move || {
        stdin
            .write_all("Hello, è€!".as_bytes())
            .expect("Failed to write to the stdin stream");
    });
    let output = child
        .wait_with_output()
        .expect("Failed to read the stdout or stderr streams");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "Uppercase is 'HELLO, È€!'."
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "Lowercase is 'hello, è€!'."
    );
}
