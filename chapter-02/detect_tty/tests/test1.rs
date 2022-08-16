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
fn test_always() {
    use std::process::{Command, Stdio};
    let program_path = get_program_path();
    let mut child = Command::new(&program_path)
        .arg("always")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect(&format!("Failed to launch the program `{}`.", program_path));
    let mut stdin = child.stdin.take().expect("Failed to open the stdin stream");
    std::thread::spawn(move || {
        stdin
            .write_all(b"Hi 42\n")
            .expect("Failed to write to the stdin stream");
    });
    let output = child
        .wait_with_output()
        .expect("Failed to read the stdout or stderr streams");
    assert_eq!(
        output.stdout,
        b"Hi \x1b[38;5;9m\x1b[1m4\x1b[0m\x1b[38;5;9m\x1b[1m2\x1b[0m\n"
    );
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
}

#[test]
fn test_never() {
    use std::process::{Command, Stdio};
    let program_path = get_program_path();
    let mut child = Command::new(&program_path)
        .arg("never")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect(&format!("Failed to launch the program `{}`.", program_path));
    let mut stdin = child.stdin.take().expect("Failed to open the stdin stream");
    std::thread::spawn(move || {
        stdin
            .write_all(b"Hi 42\n")
            .expect("Failed to write to the stdin stream");
    });
    let output = child
        .wait_with_output()
        .expect("Failed to read the stdout or stderr streams");
    assert_eq!(output.stdout, b"Hi 42\n");
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
}

#[test]
fn test_sh() {
    use std::process::{Command, Stdio};
    let program_path = "sh";
    let mut child = Command::new(program_path)
        .arg("-c")
        //.arg("target/debug/detect_tty always")
        //.arg("target/debug/detect_tty never")
        .arg("target/debug/detect_tty")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect(&format!("Failed to launch the program `{}`.", program_path));
    let mut stdin = child.stdin.take().expect("Failed to open the stdin stream");
    std::thread::spawn(move || {
        stdin
            .write_all(b"Hi 42\n")
            .expect("Failed to write to the stdin stream");
    });
    let output = child
        .wait_with_output()
        .expect("Failed to read the stdout or stderr streams");
    assert_eq!(
        output.stdout,
        b"Hi \x1b[38;5;9m\x1b[1m4\x1b[0m\x1b[38;5;9m\x1b[1m2\x1b[0m\n" //b"Hi 42\n"
    );
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
}
