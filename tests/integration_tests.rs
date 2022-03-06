use runfiles::Runfiles;
use std::process::Command;

#[test]
fn test_fails_with_empty_stdin() {
    let runfiles = Runfiles::create().unwrap();
    let program = runfiles.rlocation("__main__/src/main");

    let status = Command::new(program)
        .status()
        .expect("failed to run the command");

    assert_eq!(status.code().unwrap(), 1)
}
