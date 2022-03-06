use std::io;
use std::io::Read;

fn main() -> Result<(), io::Error> {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)?;

    if buffer.is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "stdin was empty"));
    }

    return Ok(());
}
