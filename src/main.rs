use std::env;
use std::io;
use std::io::Read;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--help" {
        println!(
            "{}",
            "\
This is a stupid tiny parser for the output
produced by bazel's (https://bazel.build) test runner.
When running e.g. `bazel test //...`, a log file will
pre written and printed to stdout, which can be parsed
by this program to get a list of junit test results that
bazel produced.

Usage example:
    cat my.log | bazel-output-parser"
        );
        return Ok(());
    }

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    if buffer.is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "stdin was empty"));
    }

    let test_labels = bazel_output_parser::parser::parse(&buffer);

    if test_labels.is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "no tests were parsed"));
    }

    for label in test_labels {
        println!(
            "bazel-testlogs/{}{}{}/test.xml",
            label.path,
            if label.path.is_empty() { "" } else { "/" },
            label.name
        );
    }

    Ok(())
}
