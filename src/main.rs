use std::io;
use std::io::Read;

fn main() -> Result<(), io::Error> {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)?;

    if buffer.is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "stdin was empty"));
    }

    let test_labels = bazel_output_parser::parser::parse(&buffer);

    for label in test_labels {
        println!(
            "bazel-testlogs/{}{}{}/test.xml",
            label.path,
            if label.path.is_empty() { "" } else { "/" },
            label.name
        );
    }

    return Ok(());
}
