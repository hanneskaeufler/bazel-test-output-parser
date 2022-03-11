use bazel_test_output_parser::parser;
use bazel_test_output_parser::sanitizer;
use std::{env, io, io::Read};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--help" {
        println!(
            "\
This is a stupid tiny parser for the output
produced by bazel's (https://bazel.build) test runner.
When running e.g. `bazel test //...`, a log file will
pre written and printed to stdout, which can be parsed
by this program to get a list of junit test results that
bazel produced.

Usage example:
    cat my.log | {}",
            env!("CARGO_PKG_NAME")
        );
        return Ok(());
    } else if args.len() > 1 && args[1] == "--version" {
        println!(env!("CARGO_PKG_VERSION"));
    }

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let sanitized_buffer = sanitizer::sanitize(&buffer)?;
    let test_labels = parser::parse(&sanitized_buffer);

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
