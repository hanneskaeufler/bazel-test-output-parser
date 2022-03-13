use bazel_test_output_parser::{help, parser, sanitizer};
use std::{env, io, io::Read};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--help" {
        help::print_help();
        return Ok(());
    } else if args.len() > 1 && args[1] == "--version" {
        println!(env!("CARGO_PKG_VERSION"));
        return Ok(());
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
