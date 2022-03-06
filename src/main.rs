use std::io;
use std::io::Read;

fn main() -> Result<(), io::Error> {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)?;
    bazel_output_parser::parser::parse(&buffer);

    if buffer.is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "stdin was empty"));
    }

    return Ok(());
}
