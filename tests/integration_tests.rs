use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn find_program() -> PathBuf {
    PathBuf::from("target/debug/bazel-test-output-parser")
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

#[test]
fn test_fails_when_no_tests_were_parsed() {
    let mut process = Command::new(find_program())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to run the command");

    process
        .stdin
        .as_ref()
        .unwrap()
        .write_all(b"Just some output but no bazel tests")
        .expect("failed to write to stdin");

    let status = process.wait().expect("process did not exit");

    assert_eq!(status.code().unwrap(), 1)
}

#[test]
fn test_prints_test_xmls() {
    let process = Command::new(find_program())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to run the command");

    process
        .stdin
        .as_ref()
        .unwrap()
        .write_all(b"//:some_test            PASSED\n//other/thing:foo (cached) PASSED")
        .expect("failed to write to stdin");

    let output = process.wait_with_output().expect("process did not exit");

    assert_eq!(
        String::from_utf8(output.stdout).unwrap_or_default(),
        "bazel-testlogs/some_test/test.xml\nbazel-testlogs/other/thing/foo/test.xml\n"
    )
}

#[test]
fn test_prints_help() {
    let output = Command::new(find_program())
        .arg("--help")
        .output()
        .expect("failed to run the command");

    assert!(String::from_utf8(output.stdout).unwrap().contains("Usage"))
}

#[test]
fn test_prints_version() {
    let output = Command::new(find_program())
        .arg("--version")
        .output()
        .expect("failed to run the command");

    assert_eq!(
        format!("{}\n", env!("CARGO_PKG_VERSION")),
        String::from_utf8(output.stdout).unwrap()
    );
}
