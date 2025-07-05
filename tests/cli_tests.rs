use std::process::Command;

#[test]
fn runs_with_search_string_and_filename() {
    let output = Command::new(env!("CARGO_BIN_EXE_minigrep"))
        .arg("searchstring")
        .arg("example-filename.txt")
        .output()
        .expect("failed to execute binary");

    assert!(
        output.status.success(),
        "Expected command to succeed, but got stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Hello"), "Actual stdout: {}", stdout);
}
