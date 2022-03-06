use std::io;

fn main() -> Result<(), io::Error> {
    return Err(io::Error::new(io::ErrorKind::Other, "stdin was empty"));
}
