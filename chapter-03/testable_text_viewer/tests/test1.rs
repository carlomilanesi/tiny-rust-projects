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
// Prepare this test by running, from the folder ‘tests’:
// cargo run test1.data.txt --save-events test1.in.txt | tee test1.out.txt
fn redirect_input_output_error() {
    use std::process::{Command, Stdio};
    let program_path = get_program_path();
    let mut child = Command::new(&program_path)
        .arg("tests/test1.data.txt")
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
    assert_eq!(
        std::str::from_utf8(&output.stdout).unwrap(),
        include_str!("test1.out.txt")
    );
    assert_eq!(std::str::from_utf8(&output.stderr).unwrap(), "");
}
