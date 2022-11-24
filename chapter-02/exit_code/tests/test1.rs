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
fn terminate_with_success() {
    let out = std::process::Command::new(&get_program_path())
        .arg("success")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    assert!(out.status.success());
    assert_eq!(out.stdout, b"Good!\n");
    assert_eq!(out.stderr, b"");
}

#[test]
fn terminate_with_abort() {
    let out = std::process::Command::new(&get_program_path())
        .arg("abort")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), None);
    assert!(!out.status.success());
    assert_eq!(out.stdout, b"");
    assert_eq!(out.stderr, b"");
}

#[test]
fn terminate_with_panic() {
    let out = std::process::Command::new(&get_program_path())
        .arg("panic")
        .arg("Ouch!")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(101));
    assert!(!out.status.success());
    assert_eq!(out.stdout, b"");
    let first_part = b"thread 'main' panicked at 'Ouch!'";
    assert_eq!(out.stderr[..first_part.len()], first_part[..]);
}

/*
#[test]
fn terminate_with_exit_0() {
    let out = std::process::Command::new(&get_program_path())
        .arg("exit")
        .arg("0")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    assert!(out.status.success());
    assert_eq!(out.stdout, b"");
    assert_eq!(out.stderr, b"");
}

#[test]
fn terminate_with_exit_100() {
    let out = std::process::Command::new(&get_program_path())
        .arg("exit")
        .arg("100")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(100));
    assert!(!out.status.success());
    assert_eq!(out.stdout, b"");
    assert_eq!(out.stderr, b"");
}

#[test]
fn terminate_with_error_0() {
    let out = std::process::Command::new(&get_program_path())
        .arg("error")
        .arg("0")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert!(!out.status.success());
    assert_eq!(out.stdout, b"");
    assert_eq!(out.stderr, b"Error: 0\n");
}

#[test]
fn terminate_with_error_100() {
    let out = std::process::Command::new(&get_program_path())
        .arg("error")
        .arg("100")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert!(!out.status.success());
    assert_eq!(out.stdout, b"");
    assert_eq!(out.stderr, b"Error: 100\n");
}
*/
