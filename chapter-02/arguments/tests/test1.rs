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
    assert_eq!(out, format!("[{}]", program_path));
}

#[test]
fn one_argument() {
    let program_path = get_program_path();
    // /*
    let output = std::process::Command::new(&program_path)
        .arg("first")
        .output()
        .unwrap();
    // */
    /*
    let mut command = std::process::Command::new(&program_path);
    command.arg("firsta");
    let output = command.output().unwrap();
    */

    let out = String::from_utf8_lossy(&output.stdout);
    assert_eq!(out, format!("[{}][first]", program_path));
}

#[test]
fn three_spaced_arguments() {
    let program_path = get_program_path();
    let output = std::process::Command::new(&program_path)
        .arg(" first  ")
        .arg("second argument")
        .arg("third€")
        .output()
        .unwrap();
    let out = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        out,
        format!("[{}][ first  ][second argument][third€]", program_path)
    );
}
