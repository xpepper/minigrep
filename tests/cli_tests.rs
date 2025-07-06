use std::fs;
use std::process::{Command, Output};

#[test]
fn runs_with_search_string_and_filename() {
    fs::write("test_file.txt", poem()).expect("Failed to write test file");

    let output = Command::new(env!("CARGO_BIN_EXE_minigrep"))
        .arg("a_search_string")
        .arg("test_file.txt")
        .output()
        .expect("failed to execute binary");

    assert_command_succeeded(&output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_contains(&stdout, "Searching for a_search_string");
    assert_contains(&stdout, "In file test_file.txt");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_is_empty(&stderr);

    fs::remove_file("test_file.txt").ok();
}

#[test]
fn fails_with_missing_parameters() {
    let output = Command::new(env!("CARGO_BIN_EXE_minigrep"))
        .output()
        .expect("failed to execute binary");

    assert!(!output.status.success());
}

#[test]
fn fails_with_missing_file() {
    let output = Command::new(env!("CARGO_BIN_EXE_minigrep"))
        .arg("a_search_string")
        .arg("non_existent_file.txt")
        .output()
        .expect("failed to execute binary");

    assert!(!output.status.success());
}

fn assert_is_empty(string: &str) {
    assert!(string.is_empty(), "Expected empty, but got: {string}");
}

fn assert_contains(string: &str, expected: &str) {
    assert!(string.contains(expected), "Actual string: {string}");
}

fn assert_command_succeeded(output: &Output) {
    assert!(
        output.status.success(),
        "Expected command to succeed, but got stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn poem() -> &'static str {
    "I'm nobody! Who are you?\n\
                 Are you nobody, too?\n\
                 Then there's a pair of us - don't tell!\n\
                 They'd banish us, you know.\n\
                 \n\
                 How dreary to be somebody!\n\
                 How public, like a frog\n\
                 To tell your name the livelong day\n\
                 To an admiring bog!\n"
}
