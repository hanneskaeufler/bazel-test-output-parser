extern crate strip_ansi_escapes;
use std::io;

pub fn sanitize(buffer: &str) -> Result<String, io::Error> {
    if buffer.is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "stdin was empty"));
    }

    return Ok(strip_ansi_escapes::strip(&buffer)
        .map(|s| String::from_utf8(s))?
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?);
}
