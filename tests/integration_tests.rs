use std::process::Command;

#[test]
fn test_succeeds_with_empty_bazel_output() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo hello")
        .output()
        .expect("Something went wrong");

    assert_eq!(String::from_utf8(output.stdout).unwrap(), "hello\n")
}
