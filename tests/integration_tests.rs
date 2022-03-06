use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn find_program() -> PathBuf {
    return PathBuf::from("target/debug/bazel-output-parser");
}

#[test]
fn test_fails_with_empty_stdin() {
    let status = Command::new(find_program())
        .stdin(Stdio::null())
        .status()
        .expect("failed to run the command");

    assert_eq!(status.code().unwrap(), 1)
}

#[test]
fn test_succeeds_with_typical_stdin() {
    let mut process = Command::new(find_program())
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to run the command");

    process
        .stdin
        .as_ref()
        .unwrap()
        .write_all(b"//:some_test            PASSED\n")
        .expect("failed to write to stdin");

    let status = process.wait().expect("process did not exit");
    assert!(status.success())
}
