use runfiles::Runfiles;
use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_fails_with_empty_stdin() {
    let runfiles = Runfiles::create().unwrap();
    let program = runfiles.rlocation("__main__/src/main");

    let status = Command::new(program)
        .status()
        .expect("failed to run the command");

    assert_eq!(status.code().unwrap(), 1)
}

#[test]
fn test_succeeds_with_typical_stdin() {
    let runfiles = Runfiles::create().unwrap();
    let program = runfiles.rlocation("__main__/src/main");

    let mut process = Command::new(program)
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
