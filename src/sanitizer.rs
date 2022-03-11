extern crate strip_ansi_escapes;
use std::io;

pub fn sanitize(buffer: &str) -> Result<String, io::Error> {
    if buffer.is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "stdin was empty"));
    }

    strip_ansi_escapes::strip(&buffer)
        .map(String::from_utf8)?
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))
}
