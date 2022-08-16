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
fn test_h_slash_literals() {
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
            .write_all(b"H/\n")
            .expect("Failed to write to the stdin stream");
    });
    let output = child
        .wait_with_output()
        .expect("Failed to read the stdout or stderr streams");
    assert_eq!(
        output.stdout,
        b"\x1b[38;5;2mH\x1b[39m\x1b[38;5;12m/\x1b[39m\n"
    );
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
}

#[test]
fn test_h_slash_included() {
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
            .write_all(include_bytes!("test1.in.txt"))
            .expect("Failed to write to the stdin stream");
    });
    let output = child
        .wait_with_output()
        .expect("Failed to read the stdout or stderr streams");
    assert_eq!(output.stdout, include_bytes!("test1.out.txt"));
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
}
