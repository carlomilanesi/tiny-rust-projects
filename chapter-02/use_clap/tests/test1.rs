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

#[test]
fn no_arguments() {
    let program_path = get_program_path();
    let output = std::process::Command::new(&program_path).output().unwrap();
    let out = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        out,
        "Options { \
        verbose: false, \
        crate_name: None, \
        edition: None, \
        error_format: Text, \
        input: [] \
        }"
    );
}

#[test]
fn short_verbose() {
    let program_path = get_program_path();
    let output = std::process::Command::new(&program_path)
        .arg("-V")
        .output()
        .unwrap();
    let out = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        out,
        "Options { \
        verbose: true, \
        crate_name: None, \
        edition: None, \
        error_format: Text, \
        input: [] \
        }"
    );
}

#[test]
fn all_options() {
    let program_path = get_program_path();
    let output = std::process::Command::new(&program_path)
        .arg("-V")
        .arg("--crate-name=my_crate")
        .arg("--edition=2013")
        .arg("--error-format")
        .arg("xml")
        .arg("first.txt")
        .arg("second.data")
        .arg("subdir/third")
        .output()
        .unwrap();
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "Options { \
        verbose: true, \
        crate_name: Some(\"my_crate\"), \
        edition: Some(2013), \
        error_format: Xml, \
        input: [\
        \"first.txt\", \
        \"second.data\", \
        \"subdir/third\"] \
        }"
    );
    assert_eq!(&output.stderr, b"");
}

#[test]
fn file_driven() -> Result<(), std::io::Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::process::Command;
    enum State {
        Args,
        Out,
        Err,
    }
    let program_path = get_program_path();
    let file = BufReader::new(File::open("tests/cases.txt").unwrap());
    let mut command = Command::new(&program_path);
    let mut out = "".to_string();
    let mut err = "".to_string();
    let mut state = State::Args;
    for line in file.lines() {
        let line = line.unwrap();
        let line = line.trim_end();
        match state {
            State::Args => {
                if line.len() == 0 {
                    let output = command.output().unwrap();
                    out = String::from_utf8_lossy(&output.stdout).to_string();
                    err = String::from_utf8_lossy(&output.stderr).to_string();
                    state = State::Out;
                } else {
                    command.arg(line);
                }
            }
            State::Out => {
                assert_eq!(format!("out:{}", out), line);
                state = State::Err;
            }
            State::Err => {
                assert_eq!(format!("err:{}", err), line);
                command = Command::new(&program_path);
                out = "".to_string();
                err = "".to_string();
                state = State::Args;
            }
        }
    }
    Ok(())
}
