use runfiles::Runfiles;
use std::process::Command;

#[test]
fn test_succeeds_with_empty_stdin() {
    let runfiles = Runfiles::create().unwrap();
    let program = runfiles.rlocation("__main__/src/main");

    let output = Command::new(program)
        .output()
        .expect("Running the command failed");

    assert_eq!(String::from_utf8(output.stdout).unwrap(), "hello\n")
}
