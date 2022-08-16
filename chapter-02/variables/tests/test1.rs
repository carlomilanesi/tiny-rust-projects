use std::process::Command;

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
fn check_variable_definition() {
    use std::env::{remove_var, set_var, var};
    let var_name = "abcd";
    let var_value = "xyz";
    remove_var(var_name);
    assert!(var(var_name).is_err());
    set_var(var_name, var_value);
    assert_eq!(var(var_name), Ok(var_value.to_string()));
}

#[test]
fn passed_in_variables() {
    use std::collections::BTreeSet;
    let output = Command::new(&get_program_path())
        .env_clear()
        .env("ABCD", "1 X")
        .env(" ABC", "2")
        .env("ABC", "  3 ")
        .env("AB", "4")
        .arg("^ABC")
        .output()
        .unwrap();
    let out = String::from_utf8_lossy(&output.stdout);
    let actual_variables = out.lines().collect::<BTreeSet<&str>>();
    let expected_variables = BTreeSet::from_iter(["[ABCD]=[1 X]", "[ABC]=[  3 ]"]);
    assert_eq!(actual_variables, expected_variables);
}

#[test]
fn exported_variables() {
    std::env::set_var("ABCD", "X");
    let output = std::process::Command::new(&get_program_path())
        .arg("^ABC")
        .output()
        .unwrap();
    let out = String::from_utf8_lossy(&output.stdout);
    assert_eq!(out, "[ABCD]=[X]\n");
}
// In Windows, launch with this command:
//cargo test -- --test-threads 1
