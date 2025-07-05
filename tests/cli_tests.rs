use std::process::Command;

#[test]
fn runs_with_search_string_and_filename() {
    std::fs::write("test_file.txt", poem()).expect("Failed to write test file");

    let output = Command::new(env!("CARGO_BIN_EXE_minigrep"))
        .arg("searchstring")
        .arg("test_file.txt")
        .output()
        .expect("failed to execute binary");

    assert!(
        output.status.success(),
        "Expected command to succeed, but got stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Hello"), "Actual stdout: {}", stdout);

    // Clean up the test file. If the cleanup fails, ignore it for the test.
    std::fs::remove_file("test_file.txt").ok();
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
